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
/// `WEAVER_TEST_GGUF` overrides the location for a machine laid out
/// differently.
const MODEL: &str = "/opt/weaver/models/smollm2-360m-instruct-q8_0.gguf";

fn model_present() -> Option<PathBuf> {
    match std::env::var_os("WEAVER_TEST_GGUF") {
        // An explicit request that cannot be met is a failure rather than a
        // skip: a runner that asked for the load path is told it did not run,
        // instead of a green result claiming it did. The skip below survives
        // only for the implicit case, where this machine simply is not one
        // that holds the fixture.
        Some(named) => {
            let path = PathBuf::from(named);
            assert!(
                path.exists(),
                "WEAVER_TEST_GGUF names {}, which does not exist",
                path.display()
            );
            Some(path)
        }
        None => Path::new(MODEL).exists().then(|| PathBuf::from(MODEL)),
    }
}

/// The device tests bind device 0 and read device-global free memory, so a
/// parallel run would have one test's admit move another test's measurement,
/// an intermittent failure about scheduling rather than about the property.
/// They take this lock and run one at a time.
static DEVICE: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn device_lock() -> std::sync::MutexGuard<'static, ()> {
    // A poisoned lock means an earlier test panicked, which that test already
    // reported. Cascading its failure into this one would report it twice.
    DEVICE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
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
/// The second-admit refusal is asserted inside the same lifecycle rather than
/// in a second test, and the shape is load-bearing: one process performs one
/// load, exactly as the shipped binary does. The engine hangs on a
/// free-then-reload inside one process, found when the device lock serialized
/// what the parallel run had overlapped, and the shipped binary cannot meet
/// that hang because this crate begins empty, admits once, and dies. A second
/// in-process lifecycle exists only in a harness, so the harness does not
/// build one.
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
    if device_free_bytes().is_none() {
        eprintln!("SKIP a_real_admit_holds_the_device: no CUDA device");
        return;
    }
    let _device = device_lock();
    let before = device_free_bytes().expect("the driver answers under the lock");

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

    // Nothing is idempotent: a second admit refuses on the ordering with an
    // identical binding, while the first residency stands.
    let second = residency.admit(&binding, Headroom(64 * 1024 * 1024));
    assert!(
        matches!(
            second,
            Err(weaver_spu::residency::AdmitRefusal::AlreadyAttempted)
        ),
        "the second admit refuses on the ordering even with an identical binding"
    );

    residency.release().expect("the release succeeds");
    let after = device_free_bytes().expect("the driver still answers");
    assert!(
        after.saturating_sub(held) > 100 * 1024 * 1024,
        "the release returns the memory before answering: held {held}, after {after}"
    );
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
        // The streams go to null rather than pipes: llama.cpp writes model
        // metadata to stderr during the load, nothing here drains a pipe until
        // after the exchanges, and a load chatty enough to fill the buffer
        // would wedge the child mid-write against the parent mid-recv. This
        // test reads answers and an exit status, never output.
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
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
        // Held across the whole spawn-and-wait, because the child process is
        // what takes the device.
        let _device = device_lock();

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
        let mut child = child;
        let status = child.wait().expect("the binary exits");
        assert!(status.success(), "and the process exits clean");
    }
}
