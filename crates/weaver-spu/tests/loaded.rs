//! conforms: spu-release-frees-before-answering
//! conforms: spu-readout-refused-at-admit
//!
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

mod common;

use std::path::{Path, PathBuf};

use weaver_spu::readout::ReadoutElection;
use weaver_spu::residency::{Headroom, Residency};
use weaver_types::{ArtifactRef, DecoderInstruction, DeviceOrdinal, ModelBinding, SpuInstruction};

/// A small real model this workshop holds. The test is about the load path,
/// not the model, so the fixture is the smallest artifact whose architecture
/// and template agree. That second condition is not decoration: the smaller
/// smollm2-360m declares `llama` and renders ChatML, so the registry hands it
/// the Llama 3 stop set it was never trained against and the load refuses,
/// correctly. Until an entry carries that pairing the fixture cannot be chosen
/// on size alone. `WEAVER_TEST_GGUF` overrides the location for a machine laid
/// out differently.
const MODEL: &str = "/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf";

fn model_present() -> Option<PathBuf> {
    match std::env::var_os("WEAVER_TEST_GGUF") {
        // An explicit request that cannot be met is a failure rather than a
        // skip: a runner that asked for the load path is told it did not run,
        // instead of a green result claiming it did. The skip below survives
        // only for the implicit case, where this machine simply is not one
        // that holds the fixture. Both branches demand a regular file, since
        // a directory or a socket at the path would fail later and blame the
        // loader.
        Some(named) => {
            let path = PathBuf::from(named);
            assert!(
                path.is_file(),
                "WEAVER_TEST_GGUF names {}, which is not a regular file",
                path.display()
            );
            Some(path)
        }
        None => Path::new(MODEL).is_file().then(|| PathBuf::from(MODEL)),
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
    DEVICE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// One context held for a whole measurement sequence, so the probes read a
/// stable view: creating and dropping a context per probe would retain and
/// release the primary context around each read.
fn device_context() -> Option<std::sync::Arc<cudarc::driver::CudaContext>> {
    cudarc::driver::CudaContext::new(0).ok()
}

fn free_bytes(context: &cudarc::driver::CudaContext) -> u64 {
    let (free, _total) = context.mem_get_info().expect("the driver answers");
    free as u64
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
    let Some(context) = device_context() else {
        eprintln!("SKIP a_real_admit_holds_the_device: no CUDA device");
        return;
    };
    let _device = device_lock();
    let before = free_bytes(&context);

    let mut residency = Residency::new();
    let binding = ModelBinding {
        artifact: ArtifactRef(model.to_string_lossy().into_owned()),
        devices: vec![DeviceOrdinal(0)],
    };

    let admitted = residency.admit(&binding, Headroom(64 * 1024 * 1024), ReadoutElection(false), false);
    let resident = match admitted {
        Ok(resident) => resident,
        Err(refusal) => panic!("the admit succeeds against a real artifact, got {refusal:?}"),
    };

    // The weights hash is real, never the sentinel, against bytes that loaded.
    assert!(
        !resident.weights_hash.is_sentinel(),
        "a loaded artifact hashes to a value"
    );
    let held = free_bytes(&context);
    // The artifact is ~370 MB quantized. The device must be holding a
    // substantial part of that; the threshold is far below the artifact size
    // so allocator granularity cannot flake it, while far above noise.
    assert!(
        before.saturating_sub(held) > 100 * 1024 * 1024,
        "the admit holds device memory: before {before}, held {held}"
    );

    // Nothing is idempotent: a second admit refuses on the ordering with an
    // identical binding, while the first residency stands.
    let second = residency.admit(&binding, Headroom(64 * 1024 * 1024), ReadoutElection(false), false);
    assert!(
        matches!(
            second,
            Err(weaver_spu::residency::AdmitRefusal::AlreadyAttempted)
        ),
        "the second admit refuses on the ordering even with an identical binding"
    );

    residency.release().expect("the release succeeds");
    let after = free_bytes(&context);
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
    use std::os::fd::AsRawFd;
    use std::process::{Command, Stdio};

    use crate::common::{ask, bound_receives, place_inherited, seqpacket_pair, wait_bounded};
    use weaver_types::{LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Payload};

    /// **The election the wire carries is the election the judgment
    /// receives.** An admit whose instruction elects readout against a family
    /// whose engine cannot tap refuses at admit, per Spec section 7 and
    /// charter step 3. The refusal crosses as `DeviceCannotAdmit`, the floor
    /// case the election failure maps onto, and it lands at the third step,
    /// so the device this file otherwise requires is never touched on this
    /// path.
    ///
    /// Perturbation: replace the dispatched election at the admit arm with
    /// `ReadoutElection(false)`, the placeholder the routeless seam once
    /// forced, and this test fails on this machine, the admit proceeding past
    /// the judgment onto the device and answering `Admitted`. Watched under
    /// exactly that change, which is why the device skip stays: without a
    /// device the perturbed run refuses later for other reasons and the watch
    /// distinguishes nothing.
    #[test]
    fn an_admit_electing_readout_refuses_across_the_seam() {
        let Some(model) = model_present() else {
            eprintln!("SKIP an_admit_electing_readout_refuses: no model at {MODEL}");
            return;
        };
        if device_context().is_none() {
            eprintln!("SKIP an_admit_electing_readout_refuses: no CUDA device");
            return;
        }
        let _device = device_lock();

        let (lifecycle, child_lifecycle) = seqpacket_pair();
        let (_decode, child_decode) = seqpacket_pair();
        let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        place_inherited(
            &mut command,
            &[child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()],
        );
        let mut child = command.spawn().expect("the binary starts");
        bound_receives(&lifecycle, 120);

        let refused = ask(
            &lifecycle,
            1,
            LifecycleDirective::Admit {
                instruction: SpuInstruction {
                    classify: None,
                    decoder: DecoderInstruction {
                        model_binding: ModelBinding {
                            artifact: ArtifactRef(model.to_string_lossy().into_owned()),
                            devices: vec![DeviceOrdinal(0)],
                        },
                        residual_readout_election: true,
                    field_election: None,
                    surprisal_election: false,
                        identity: vec![],
                        tunable_values: [
                            ("max-tokens-per-turn".to_string(), 4096.0),
                            ("context-capacity".to_string(), 4096.0),
                            ("seed".to_string(), 11.0),
                        ]
                            .into_iter()
                            .collect(),
                    },
                },
            },
        );
        assert_eq!(
            refused.payload,
            Payload::Refusal(LifecycleRefusal::DeviceCannotAdmit),
            "an elected readout no shipped family can honor refuses at admit"
        );
        assert_eq!(refused.exchange.ordinal, 1, "on the exchange that asked");

        drop(lifecycle);
        let status = wait_bounded(&mut child, 30, "the refused worker exits");
        assert!(
            status.success(),
            "a refused admit is an answer, not a death: the worker serves on and \
             exits clean when the channel closes"
        );
    }

    /// **The contract's success path, whole:** admit answers `Admitted`,
    /// release answers `Released`, each on the exchange that asked, across the
    /// real seam, against a real artifact, on a real device.
    ///
    /// Every wait is bounded. The receives carry a timeout sized to a model
    /// load, and the child's exit is polled against a deadline with a kill on
    /// expiry, so a wedged stage becomes a failure naming itself rather than a
    /// hang the harness cannot end.
    #[test]
    fn admit_then_release_round_trips_across_the_seam() {
        let Some(model) = model_present() else {
            eprintln!("SKIP admit_then_release_round_trips: no model at {MODEL}");
            return;
        };
        if device_context().is_none() {
            eprintln!("SKIP admit_then_release_round_trips: no CUDA device");
            return;
        }
        // Held across the whole spawn-and-wait, because the child process is
        // what takes the device.
        let _device = device_lock();

        let (lifecycle, child_lifecycle) = seqpacket_pair();
        let (decode_parent, child_decode) = seqpacket_pair();
        let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
        // The streams go to a file rather than pipes: llama.cpp writes model
        // metadata to stderr during the load, nothing here drains a pipe until
        // after the exchanges, and a load chatty enough to fill the buffer
        // would wedge the child mid-write against the parent mid-recv. A file
        // has no such buffer, and unlike null it leaves the child's account on
        // disk for the failure that needs it.
        let log_path =
            std::env::temp_dir().join(format!("weaver-spu-seam-child-{}.log", std::process::id()));
        let log = std::fs::File::create(&log_path).expect("a child log file");
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::from(log));
        place_inherited(
            &mut command,
            &[child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()],
        );
        let mut child = command.spawn().expect("the binary starts");
        // Generous against a model load, far under the harness's patience.
        bound_receives(&lifecycle, 120);

        let admitted = ask(
            &lifecycle,
            1,
            LifecycleDirective::Admit {
                instruction: SpuInstruction {
                    classify: None,
                    decoder: DecoderInstruction {
                        model_binding: ModelBinding {
                            artifact: ArtifactRef(model.to_string_lossy().into_owned()),
                            devices: vec![DeviceOrdinal(0)],
                        },
                        residual_readout_election: false,
                        field_election: None,
                        surprisal_election: false,
                        identity: vec![],
                        tunable_values: [
                            ("max-tokens-per-turn".to_string(), 4096.0),
                            ("context-capacity".to_string(), 4096.0),
                            ("seed".to_string(), 11.0),
                        ]
                            .into_iter()
                            .collect(),
                    },
                },
            },
        );
        assert_eq!(
            admitted.payload,
            Payload::Answer(LifecycleAnswer::Admitted),
            "the admit answers Admitted across the seam"
        );
        assert_eq!(admitted.exchange.ordinal, 1);

        // The decode end closes before the release is asked: once the admit
        // confirms, the worker's decode phase owns it until the channel ends,
        // per the serve loop's one-loop rule, so a release sent while the
        // decode end stands would wait behind a phase that never returns.
        drop(decode_parent);

        let released = ask(&lifecycle, 2, LifecycleDirective::Release);
        assert_eq!(
            released.payload,
            Payload::Answer(LifecycleAnswer::Released),
            "the release answers Released across the seam"
        );
        assert_eq!(released.exchange.ordinal, 2);

        drop(lifecycle);
        let status = wait_bounded(
            &mut child,
            30,
            &format!(
                "admit_then_release_round_trips, child stderr at {}",
                log_path.display()
            ),
        );
        assert!(status.success(), "and the process exits clean");
        std::fs::remove_file(&log_path).ok();
    }

    /// **The seam serves, end to end:** an open renders and makes the prefix
    /// resident, an append renders the turn, generates against the real
    /// engine, and answers with the generation and its measurement, a flush
    /// returns the session to its prefix, and the whole conversation crosses
    /// the real decode socket as the trio's JSON. This is the decode seam's
    /// service act watched whole, per `weaver-harness-spu-decode-contract`
    /// section 2 and `weaver-spu-Spec` section 9.
    #[test]
    fn the_declared_ceiling_governs_the_generation() {
        use weaver_traits::{ContentBlock, Message, Role};
        use weaver_types::{SessionId, TokenAnswer, TokenDirective, TurnKey};

        let Some(model) = model_present() else {
            eprintln!("SKIP the_declared_ceiling: no model at {MODEL}");
            return;
        };
        if device_context().is_none() {
            eprintln!("SKIP the_declared_ceiling: no CUDA device");
            return;
        }
        let _device = device_lock();

        let (lifecycle, child_lifecycle) = seqpacket_pair();
        let (decode_parent, child_decode) = seqpacket_pair();
        let log_path = std::env::temp_dir().join(format!(
            "weaver-spu-ceiling-child-{}.log",
            std::process::id()
        ));
        let log = std::fs::File::create(&log_path).expect("a child log file");
        let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::from(log));
        place_inherited(
            &mut command,
            &[child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()],
        );
        let mut child = command.spawn().expect("the binary starts");
        bound_receives(&lifecycle, 120);
        bound_receives(&decode_parent, 120);
        let decode = weaver_spu::channel::decode_from_owned(decode_parent);

        // The declaration elects a ceiling far below any natural stop, so
        // the only way the count below comes out right is the declared
        // value reaching the generation's stop condition: a test at 4096
        // would pass whenever a short answer finished naturally, proving
        // only that the knob did not refuse.
        let admitted = ask(
            &lifecycle,
            1,
            LifecycleDirective::Admit {
                instruction: SpuInstruction {
                    classify: None,
                    decoder: DecoderInstruction {
                        model_binding: ModelBinding {
                            artifact: ArtifactRef(model.to_string_lossy().into_owned()),
                            devices: vec![DeviceOrdinal(0)],
                        },
                        residual_readout_election: false,
                        field_election: None,
                        surprisal_election: false,
                        identity: vec![],
                        tunable_values: [
                            ("max-tokens-per-turn".to_string(), 9.0),
                            ("context-capacity".to_string(), 4096.0),
                            ("seed".to_string(), 11.0),
                        ]
                            .into_iter()
                            .collect(),
                    },
                },
            },
        );
        assert_eq!(admitted.payload, Payload::Answer(LifecycleAnswer::Admitted));

        let message = |text: &str| Message {
            role: Role::User,
            content: vec![ContentBlock::Text { text: text.into() }],
        };
        let send = |directive: &TokenDirective| {
            let body = serde_json::to_vec(directive).expect("a directive renders");
            decode.send_octets(&body).expect("the frame sends");
        };
        let recv = || -> TokenAnswer {
            let frame = decode.recv_octets().expect("an answer arrives");
            serde_json::from_slice(&frame).expect("the answer parses")
        };

        send(&TokenDirective::Open {
            session: SessionId("s-ceiling".into()),
            messages: vec![message("You count plainly.")],
        });
        assert_eq!(recv(), TokenAnswer::Opened, "the session opens");

        send(&TokenDirective::AppendAndGenerate {
            turn: TurnKey("t-1".into()),
            delta: vec![message(
                "Count upward from one, comma separated, without stopping.",
            )],
        });
        let mut streamed = 0usize;
        let generation = loop {
            match recv() {
                TokenAnswer::Token { .. } => streamed += 1,
                TokenAnswer::Generated(generation) => break generation,
                other => panic!("tokens then the close, got {other:?}"),
            }
        };
        assert_eq!(
            streamed, 9,
            "the declared nine-token ceiling governed the generation"
        );
        let measurement: serde_json::Value =
            serde_json::from_str(generation.measurement.get()).expect("measurement is JSON");
        assert_eq!(
            measurement["output_tokens"]
                .as_array()
                .expect("output tokens")
                .len(),
            9,
            "the measurement agrees with the stream"
        );

        drop(decode);
        drop(lifecycle);
        let _ = child.wait();
        let _ = std::fs::remove_file(&log_path);
    }

    /// **An elected surprisal reaches the wire, and the perplexity stands
    /// beside it rather than instead of it.** The declined posture is
    /// pinned by the seam test below, which is this suite's ordinary
    /// fixture, so what this adds is the other state: the election has to
    /// be watched in both, a flag that is always false passing a test that
    /// never sets it true.
    ///
    /// The election is fixed at admit for the residency's life, per
    /// `weaver-spu-PRD` section 13.12, so a second admit is what it takes
    /// to see the other arm rather than a second directive.
    #[test]
    fn an_elected_surprisal_reaches_the_wire() {
        use weaver_traits::{ContentBlock, Message, Role};
        use weaver_types::{SessionId, TokenAnswer, TokenDirective, TurnKey};

        let Some(model) = model_present() else {
            eprintln!("SKIP an_elected_surprisal: no model at {MODEL}");
            return;
        };
        if device_context().is_none() {
            eprintln!("SKIP an_elected_surprisal: no CUDA device");
            return;
        }
        let _device = device_lock();

        let (lifecycle, child_lifecycle) = seqpacket_pair();
        let (decode_parent, child_decode) = seqpacket_pair();
        let log_path = std::env::temp_dir().join(format!(
            "weaver-spu-ceiling-child-{}.log",
            std::process::id()
        ));
        let log = std::fs::File::create(&log_path).expect("a child log file");
        let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::from(log));
        place_inherited(
            &mut command,
            &[child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()],
        );
        let mut child = command.spawn().expect("the binary starts");
        bound_receives(&lifecycle, 120);
        bound_receives(&decode_parent, 120);
        let decode = weaver_spu::channel::decode_from_owned(decode_parent);

        // The declaration elects a ceiling far below any natural stop, so
        // the only way the count below comes out right is the declared
        // value reaching the generation's stop condition: a test at 4096
        // would pass whenever a short answer finished naturally, proving
        // only that the knob did not refuse.
        let admitted = ask(
            &lifecycle,
            1,
            LifecycleDirective::Admit {
                instruction: SpuInstruction {
                    classify: None,
                    decoder: DecoderInstruction {
                        model_binding: ModelBinding {
                            artifact: ArtifactRef(model.to_string_lossy().into_owned()),
                            devices: vec![DeviceOrdinal(0)],
                        },
                        residual_readout_election: false,
                        field_election: None,
                        surprisal_election: true,
                        identity: vec![],
                        tunable_values: [
                            ("max-tokens-per-turn".to_string(), 9.0),
                            ("context-capacity".to_string(), 4096.0),
                            ("seed".to_string(), 11.0),
                        ]
                            .into_iter()
                            .collect(),
                    },
                },
            },
        );
        assert_eq!(admitted.payload, Payload::Answer(LifecycleAnswer::Admitted));

        let message = |text: &str| Message {
            role: Role::User,
            content: vec![ContentBlock::Text { text: text.into() }],
        };
        let send = |directive: &TokenDirective| {
            let body = serde_json::to_vec(directive).expect("a directive renders");
            decode.send_octets(&body).expect("the frame sends");
        };
        let recv = || -> TokenAnswer {
            let frame = decode.recv_octets().expect("an answer arrives");
            serde_json::from_slice(&frame).expect("the answer parses")
        };

        send(&TokenDirective::Open {
            session: SessionId("s-surprisal".into()),
            messages: vec![message("You count plainly.")],
        });
        assert_eq!(recv(), TokenAnswer::Opened, "the session opens");

        send(&TokenDirective::AppendAndGenerate {
            turn: TurnKey("t-1".into()),
            delta: vec![message(
                "Count upward from one, comma separated, without stopping.",
            )],
        });
        let mut streamed = 0usize;
        let generation = loop {
            match recv() {
                TokenAnswer::Token { .. } => streamed += 1,
                TokenAnswer::Generated(generation) => break generation,
                other => panic!("tokens then the close, got {other:?}"),
            }
        };
        assert!(streamed > 0, "the model answered across the seam");
        let measurement: serde_json::Value =
            serde_json::from_str(generation.measurement.get()).expect("measurement is JSON");
        // **The elected posture on the wire**, the other half of what
        // `an_opened_session_generates_across_the_decode_seam` pins for the
        // declined one. Both members stand, and they are paired with the
        // tokens position for position, which is the property that makes a
        // per-position reading readable at all.
        let tokens = measurement["output_tokens"]
            .as_array()
            .expect("output tokens")
            .len();
        let surprisals = measurement["surprisals"]
            .as_array()
            .expect("the elected vector is on the wire")
            .len();
        assert_eq!(
            surprisals, tokens,
            "the vector is paired with the tokens: {measurement}"
        );
        assert_eq!(
            measurement["entropies"].as_array().map(Vec::len),
            Some(tokens),
            "and so is the entropy, which carries no election"
        );
        assert!(
            measurement["perplexity"].is_number(),
            "the perplexity stands beside the vector rather than instead \
             of it: {measurement}"
        );

        drop(decode);
        drop(lifecycle);
        let _ = child.wait();
        let _ = std::fs::remove_file(&log_path);
    }

    #[test]
    fn an_opened_session_generates_across_the_decode_seam() {
        use weaver_traits::{ContentBlock, Message, Role};
        use weaver_types::{SessionId, TokenAnswer, TokenDirective, TurnKey};

        let Some(model) = model_present() else {
            eprintln!("SKIP an_opened_session_generates: no model at {MODEL}");
            return;
        };
        if device_context().is_none() {
            eprintln!("SKIP an_opened_session_generates: no CUDA device");
            return;
        }
        let _device = device_lock();

        let (lifecycle, child_lifecycle) = seqpacket_pair();
        let (decode_parent, child_decode) = seqpacket_pair();
        let log_path = std::env::temp_dir().join(format!(
            "weaver-spu-decode-child-{}.log",
            std::process::id()
        ));
        let log = std::fs::File::create(&log_path).expect("a child log file");
        let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::from(log));
        place_inherited(
            &mut command,
            &[child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()],
        );
        let mut child = command.spawn().expect("the binary starts");
        bound_receives(&lifecycle, 120);
        bound_receives(&decode_parent, 120);
        let decode = weaver_spu::channel::decode_from_owned(decode_parent);

        let admitted = ask(
            &lifecycle,
            1,
            LifecycleDirective::Admit {
                instruction: SpuInstruction {
                    classify: None,
                    decoder: DecoderInstruction {
                        model_binding: ModelBinding {
                            artifact: ArtifactRef(model.to_string_lossy().into_owned()),
                            devices: vec![DeviceOrdinal(0)],
                        },
                        residual_readout_election: false,
                        field_election: None,
                        surprisal_election: false,
                        identity: vec![],
                        tunable_values: [
                            ("max-tokens-per-turn".to_string(), 4096.0),
                            ("context-capacity".to_string(), 4096.0),
                            ("seed".to_string(), 37.0),
                        ]
                            .into_iter()
                            .collect(),
                    },
                },
            },
        );
        assert_eq!(admitted.payload, Payload::Answer(LifecycleAnswer::Admitted));

        let message = |text: &str| Message {
            role: Role::User,
            content: vec![ContentBlock::Text { text: text.into() }],
        };
        let send = |directive: &TokenDirective| {
            let body = serde_json::to_vec(directive).expect("a directive renders");
            decode.send_octets(&body).expect("the frame sends");
        };
        let recv = || -> TokenAnswer {
            let frame = decode.recv_octets().expect("an answer arrives");
            serde_json::from_slice(&frame).expect("the answer parses")
        };

        send(&TokenDirective::Open {
            session: SessionId("s-1".into()),
            messages: vec![message("You answer in as few words as possible.")],
        });
        assert_eq!(recv(), TokenAnswer::Opened, "the session opens");

        send(&TokenDirective::AppendAndGenerate {
            turn: TurnKey("t-1".into()),
            delta: vec![message("Say one word.")],
        });
        // **The stream precedes the close and never disagrees with it**, per
        // the contract's coherence guarantee: token frames arrive as drawn,
        // the pieces accumulate to the emission, and the close carries the
        // generation whole.
        let mut pieces = String::new();
        let mut streamed = 0usize;
        let generation = loop {
            match recv() {
                TokenAnswer::Token { piece, .. } => {
                    pieces.push_str(&piece);
                    streamed += 1;
                }
                TokenAnswer::Generated(generation) => break generation,
                other => panic!("the stream carries tokens then the close, got {other:?}"),
            }
        };
        assert!(
            !generation.emission.is_empty(),
            "a real model said something"
        );
        assert!(streamed > 0, "the seam streamed rather than batched");
        assert_eq!(
            pieces, generation.emission,
            "the pieces accumulate to the emission, stream and close agreeing"
        );
        let request: serde_json::Value =
            serde_json::from_str(generation.request.get()).expect("the request splice is JSON");
        eprintln!("DIAG-RENDERED: {}", request["rendered"]);
        eprintln!("DIAG-EMISSION: {:?}", generation.emission);
        assert!(
            request["rendered"]
                .as_str()
                .is_some_and(|r| r.contains("Say one word.")),
            "the request carries the family's render of the delta"
        );
        assert!(
            request["template"].as_str().is_some(),
            "and the template identity"
        );
        assert!(
            request["sampling"]["temperature"].is_number(),
            "and the effective sampling"
        );
        // **The declared seed reaches the effective sampling**, which is the
        // whole of the tunable path added 2026-08-20: the value this fixture
        // supplies is the value the request reports, so a recorded run names
        // the draw it was and is re-entered by declaring it again.
        //
        // **The value is 37 because it must not be 11.** Eleven was the
        // compiled seed this knob carried while it was frozen, so a fixture
        // declaring eleven would pass against a still-frozen knob and prove
        // nothing about the route. Restore `Disposition::Frozen(11)` and
        // this fails, which is the perturbation, and it only fails because
        // the two numbers differ.
        assert_eq!(
            request["sampling"]["seed"], 37.0,
            "the declared seed is the effective one"
        );
        let measurement: serde_json::Value =
            serde_json::from_str(generation.measurement.get()).expect("the measurement is JSON");
        assert_eq!(
            measurement["blocks"][0]["label"], "turn-delta",
            "the block carries the declared label"
        );
        assert!(
            measurement["output_tokens"]
                .as_array()
                .is_some_and(|tokens| !tokens.is_empty()),
            "the tokens out are in the measurement"
        );
        assert!(
            measurement["timings"]["decode_ns"].is_string(),
            "the timings ride the decimal-string rule"
        );

        // **The unelected posture on the wire**, per `weaver-spu-PRD`
        // section 13.12: this fixture declares no surprisal election, so
        // the vector is absent and the perplexity stands in its place. The
        // entropies carry no election and are here either way.
        //
        // **The absence is checked as an absent member rather than a null.**
        // A member present and carrying nothing is the empty vector's
        // defect wearing another shape, and it is what an unguarded
        // serialization would produce.
        assert!(
            measurement.get("surprisals").is_none(),
            "no election, no vector, and no member for it: {measurement}"
        );
        assert!(
            measurement["perplexity"].is_number(),
            "the perplexity stands in the vector's place: {measurement}"
        );
        assert!(
            measurement["entropies"]
                .as_array()
                .is_some_and(|bits| !bits.is_empty()),
            "and the entropy carries no election at all"
        );
        assert_eq!(
            measurement["entropies"].as_array().map(Vec::len),
            measurement["output_tokens"].as_array().map(Vec::len),
            "the entropy is paired with the tokens position for position"
        );

        send(&TokenDirective::Flush { keep: 0 });
        let flushed = recv();
        let TokenAnswer::Flushed {
            resident_before,
            resident_after,
        } = flushed
        else {
            panic!("the flush reaches its outcome: {flushed:?}");
        };
        assert!(
            resident_before > resident_after,
            "the flush cut what stood: {resident_before} to {resident_after}"
        );

        drop(decode);
        let released = ask(&lifecycle, 2, LifecycleDirective::Release);
        assert_eq!(released.payload, Payload::Answer(LifecycleAnswer::Released));

        drop(lifecycle);
        let status = wait_bounded(
            &mut child,
            30,
            &format!(
                "the served worker exits, child stderr at {}",
                log_path.display()
            ),
        );
        assert!(status.success(), "and the process exits clean");
        std::fs::remove_file(&log_path).ok();
    }
}
