//! The success path: a real artifact admitted onto a real device and released,
//! per `weaver-spu-Spec` section 3 and `weaver-harness-spu-contract` sections 3
//! and 4. This is the path `tests/seam.rs` names as its gap, closed here for
//! the build that carries a backend.
//!
//! Everything in this file needs the `cuda` and `gguf` features, a CUDA device,
//! and a real GGUF on disk. Each absence skips loudly: a suite that reports
//! success for tests it did not run is the failure mode this corpus treats as
//! worse than a red result.

#![cfg(all(feature = "cuda", feature = "gguf"))]

use std::path::{Path, PathBuf};

use weaver_spu::residency::{Headroom, Residency};
use weaver_types::{ArtifactRef, DeviceOrdinal, ModelBinding};

/// A small real model this workshop holds. The test is about the load path,
/// not the model, so the smallest artifact on the box is the right fixture.
const MODEL: &str = "/opt/weaver/models/smollm2-360m-instruct-q8_0.gguf";

fn model_present() -> Option<PathBuf> {
    let path = Path::new(MODEL);
    path.exists().then(|| path.to_path_buf())
}

fn device_free_bytes() -> Option<u64> {
    // Read through the same driver surface the admission judges with.
    use cudarc::driver::CudaContext;
    let context = CudaContext::new(0).ok()?;
    let (free, _total) = context.mem_get_info().ok()?;
    Some(free as u64)
}

/// **Admit holds the weights on the device, and release frees them before it
/// answers.** The Spec carries the release ordering as review's, the instrument
/// being priced against a driver seam the Spec does not introduce. This build
/// has the driver itself, so the test reads the device: free memory drops
/// across the admit and returns across the release, which is the confirmation
/// being a fact about the device rather than a statement of intent.
///
/// Perturbation: make `Residency::release` answer `Ok` without taking the
/// resident and this test fails at the release assertion, because the weights
/// stay on the device. Watched under exactly that change.
#[test]
fn a_real_admit_holds_the_device_and_release_frees_it() {
    let Some(model) = model_present() else {
        eprintln!("SKIP a_real_admit_holds_the_device: no model at {MODEL}");
        return;
    };
    let Some(before) = device_free_bytes() else {
        eprintln!("SKIP a_real_admit_holds_the_device: no CUDA device");
        return;
    };

    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(model.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0)],
    };

    let admitted = residency.admit(&binding, Headroom(64 * 1024 * 1024));
    let resident = match admitted {
        Ok(resident) => resident,
        Err(refusal) => panic!("the admit succeeds against a real artifact, got {refusal:?}"),
    };

    // The weights hash is real, never the sentinel, against bytes that loaded.
    assert!(
        !resident.weights_hash.is_sentinel(),
        "a loaded artifact hashes to a value"
    );
    let held = device_free_bytes().expect("the driver still answers");
    // The artifact is ~370 MB quantized. The device must be holding a
    // substantial part of that; the threshold is far below the artifact size
    // so allocator granularity cannot flake it, while far above noise.
    assert!(
        before.saturating_sub(held) > 100 * 1024 * 1024,
        "the admit holds device memory: before {before}, held {held}"
    );

    residency.release().expect("the release succeeds");
    let after = device_free_bytes().expect("the driver still answers");
    assert!(
        after.saturating_sub(held) > 100 * 1024 * 1024,
        "the release returns the memory before answering: held {held}, after {after}"
    );
}

/// A second admit after a successful first refuses on the ordering, which the
/// unit suite shows against a refused first admit and this shows against a
/// completed one: nothing is idempotent, and a matching binding changes
/// nothing.
#[test]
fn a_second_admit_after_a_completed_first_refuses() {
    let Some(model) = model_present() else {
        eprintln!("SKIP a_second_admit_after_a_completed_first: no model at {MODEL}");
        return;
    };
    if device_free_bytes().is_none() {
        eprintln!("SKIP a_second_admit_after_a_completed_first: no CUDA device");
        return;
    }

    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(model.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0)],
    };
    residency
        .admit(&binding, Headroom(64 * 1024 * 1024))
        .expect("the first admit succeeds");
    let second = residency.admit(&binding, Headroom(64 * 1024 * 1024));
    assert!(
        matches!(
            second,
            Err(weaver_spu::residency::AdmitRefusal::AlreadyAttempted)
        ),
        "the second admit refuses on the ordering even with an identical binding"
    );
    residency.release().expect("the release still succeeds");
}

mod seam_success {
    //! The same round trip crossing the real socket into the real binary,
    //! which is what the contract governs: an answer to admit arrives only
    //! after the device holds the weights, and an answer to release only after
    //! the device is free.

    use super::*;
    use std::os::fd::{AsRawFd, OwnedFd, RawFd};
    use std::os::unix::process::CommandExt;
    use std::process::{Child, Command, Stdio};

    use nix::libc;
    use nix::sys::socket::{
        AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair,
    };
    use weaver_types::{
        ExchangeId, LifecycleAnswer, LifecycleDirective, Opener, OrganEnvelope, Payload,
        Position,
    };

    fn seqpacket_pair() -> (OwnedFd, OwnedFd) {
        socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("a socketpair")
    }

    fn spawn_holding(child_ends: [RawFd; 2]) -> Child {
        let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        unsafe {
            command.pre_exec(move || {
                let mut lifted = [0 as RawFd; 2];
                for (slot, source) in lifted.iter_mut().zip(&child_ends) {
                    let high = libc::fcntl(*source, libc::F_DUPFD, 32);
                    if high < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    *slot = high;
                }
                for (offset, high) in lifted.iter().enumerate() {
                    if libc::dup2(*high, 3 + offset as RawFd) < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                for high in lifted {
                    libc::close(high);
                }
                Ok(())
            });
        }
        command.spawn().expect("the binary starts")
    }

    fn ask(end: &OwnedFd, ordinal: u64, directive: LifecycleDirective) -> OrganEnvelope {
        let envelope = OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal,
            },
            position: Position::Open,
            payload: Payload::Directive(directive),
        };
        let body = serde_json::to_vec(&envelope).expect("an envelope encodes");
        send(end.as_raw_fd(), &body, MsgFlags::empty()).expect("the directive is sent");
        let mut buffer = vec![0u8; 64 * 1024];
        let read = recv(end.as_raw_fd(), &mut buffer, MsgFlags::empty()).expect("an answer");
        buffer.truncate(read);
        serde_json::from_slice(&buffer).expect("the answer is an envelope")
    }

    /// **The contract's success path, whole:** admit answers `Admitted`,
    /// release answers `Released`, each on the exchange that asked, across the
    /// real seam, against a real artifact, on a real device.
    #[test]
    fn admit_then_release_round_trips_across_the_seam() {
        let Some(model) = model_present() else {
            eprintln!("SKIP admit_then_release_round_trips: no model at {MODEL}");
            return;
        };
        if device_free_bytes().is_none() {
            eprintln!("SKIP admit_then_release_round_trips: no CUDA device");
            return;
        }

        let (lifecycle, child_lifecycle) = seqpacket_pair();
        let (_decode, child_decode) = seqpacket_pair();
        let child = spawn_holding([child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()]);

        let admitted = ask(
            &lifecycle,
            1,
            LifecycleDirective::Admit {
                binding: ModelBinding {
                    artifact: ArtifactRef(model.to_string_lossy().into_owned()),
                    devices: vec![DeviceOrdinal(0)],
                },
            },
        );
        assert_eq!(
            admitted.payload,
            Payload::Answer(LifecycleAnswer::Admitted),
            "the admit answers Admitted across the seam"
        );
        assert_eq!(admitted.exchange.ordinal, 1);

        let released = ask(&lifecycle, 2, LifecycleDirective::Release);
        assert_eq!(
            released.payload,
            Payload::Answer(LifecycleAnswer::Released),
            "the release answers Released across the seam"
        );
        assert_eq!(released.exchange.ordinal, 2);

        drop(lifecycle);
        let out = child.wait_with_output().expect("the binary exits");
        assert!(out.status.success(), "and the process exits clean");
    }
}
