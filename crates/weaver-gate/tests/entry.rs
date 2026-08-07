//! conforms: gate-dumpable-flag-cleared
//! conforms: gate-closure-is-death
//!
//! The process facts of `weaver-gate-Spec` section 2, exercised as a process.
//!
//! **The second walk: a same-uid process attaches to the gate.** The adversary
//! is the agent's tool surface, the attack a `ptrace` or `/proc/[pid]/fd` reach
//! into the one process of the worker's family that faces outward. The
//! mechanism is the dumpable flag cleared at entry, a set and not a check.
//!
//! **The third walk: the boundary outlives its interior.** The adversary is
//! timing, a client conversing with a gate whose harness is gone. The test
//! closes the harness end of a standing pair and confirms the gate exits and
//! the listener is gone. **It reaches the closure half and no further:** the
//! parent-death signal's backing needs a double that forks from a thread and
//! ends that thread while its process lives, which this suite does not buy, and
//! killing the double as a process would end the forking thread too, making the
//! perturbation one that can never be seen to fail.

use std::io::Write;
use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use nix::libc;
use nix::sys::socket::{AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair};
use weaver_types::{
    AccessRule, ExchangeId, GateInstruction, LifecycleAnswer, LifecycleDirective, Opener,
    OrganEnvelope, Payload, Position,
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

/// Place the child's end at descriptor 3 and mark everything above it
/// close-on-exec.
///
/// Marked, never closed: Rust reports a failed exec to the parent through a
/// close-on-exec pipe it opens above this range, and closing that pipe makes
/// the parent read EOF and conclude the spawn succeeded. Nothing in the closure
/// allocates, since it runs in the child of a multithreaded test process.
fn spawn_gate(child_end: RawFd) -> (Child, PathBuf) {
    let log_path = std::env::temp_dir().join(format!(
        "weaver-gate-entry-{}-{}.log",
        std::process::id(),
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let log = std::fs::File::create(&log_path).expect("a child log file");
    let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-gate"));
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::from(log));
    unsafe {
        command.pre_exec(move || {
            let high = libc::fcntl(child_end, libc::F_DUPFD, 32);
            if high < 0 {
                return Err(std::io::Error::last_os_error());
            }
            if libc::dup2(high, 3) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            libc::close(high);
            // Checked: an unchecked sweep that failed would leave the child
            // holding inheritable descriptors and refusing to serve for a
            // reason the test could not see. Failing the spawn says it here.
            if libc::syscall(
                libc::SYS_close_range,
                4u32,
                u32::MAX,
                libc::CLOSE_RANGE_CLOEXEC,
            ) != 0
            {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    (command.spawn().expect("the binary starts"), log_path)
}

static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

fn wait_bounded(child: &mut Child, seconds: u64, what: &str) -> std::process::ExitStatus {
    let deadline = Instant::now() + Duration::from_secs(seconds);
    loop {
        match child.try_wait().expect("a wait succeeds") {
            Some(status) => return status,
            None if Instant::now() > deadline => {
                child.kill().ok();
                child.wait().ok();
                panic!("{what}: the child did not exit within {seconds}s");
            }
            None => std::thread::sleep(Duration::from_millis(25)),
        }
    }
}

fn ask(end: &OwnedFd, ordinal: u64, directive: LifecycleDirective) -> OrganEnvelope {
    // Bounded, so a gate that never answers is a failure naming the exchange
    // rather than a hang the harness cannot end.
    nix::sys::socket::setsockopt(
        end,
        nix::sys::socket::sockopt::ReceiveTimeout,
        &nix::sys::time::TimeVal::new(30, 0),
    )
    .expect("the receive timeout sets");

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
    let read = recv(end.as_raw_fd(), &mut buffer, MsgFlags::empty())
        .unwrap_or_else(|errno| panic!("no answer to ordinal {ordinal}: {errno}"));
    assert!(
        read > 0,
        "the gate closed without answering ordinal {ordinal}"
    );
    buffer.truncate(read);
    serde_json::from_slice(&buffer).expect("the answer is an envelope")
}

fn socket_path(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("weaver-gate-entry-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch dir");
    let path = dir.join("gate.sock");
    std::fs::remove_file(&path).ok();
    path
}

fn instruction(path: PathBuf) -> GateInstruction {
    GateInstruction {
        socket_path: path,
        access_rule: AccessRule {
            allowed_uids: std::collections::BTreeSet::new(),
            allowed_gids: std::collections::BTreeSet::new(),
            denied_uids: std::collections::BTreeSet::new(),
        },
    }
}

/// **The dumpable flag is cleared at entry**, read from outside by an observer
/// the process cannot lie to: clearing it transfers ownership of `/proc/[pid]`
/// to root, so the parent reads the fact from the filesystem.
///
/// Perturbation: remove the `set_dumpable` call from `adopt` and this test
/// fails, because `/proc/[pid]/fd` stays owned by the invoking uid. Watched
/// under exactly that removal.
#[test]
fn the_dumpable_flag_is_clear_after_entry() {
    use std::os::unix::fs::MetadataExt;

    // Under uid 0 the observation is vacuous: /proc/[pid]/fd is root-owned
    // either way, so the test would pass with the clear removed.
    if nix::unistd::geteuid().is_root() {
        eprintln!("SKIP the_dumpable_flag_is_clear_after_entry: root owns /proc either way");
        return;
    }

    let (harness, child_end) = seqpacket_pair();
    let (mut child, log_path) = spawn_gate(child_end.as_raw_fd());
    let pid = child.id();

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut owner = None;
    while Instant::now() < deadline {
        if let Ok(meta) = std::fs::metadata(format!("/proc/{pid}/fd")) {
            owner = Some(meta.uid());
            if meta.uid() == 0 {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(25));
    }

    drop(harness);
    wait_bounded(&mut child, 30, "the_dumpable_flag_is_clear_after_entry");
    std::fs::remove_file(&log_path).ok();

    assert_eq!(
        owner,
        Some(0),
        "a cleared dumpable flag makes /proc/{pid}/fd owned by root, found {owner:?}"
    );
}

/// **Closure is death.** The harness end of a standing pair closes while the
/// gate holds a raised hook, and the gate exits and its listener is gone.
///
/// The double stays alive as a process throughout: it is this test, and it
/// simply drops its end. That matters, because killing a double as a process
/// would end the forking thread too and the parent-death signal would exit the
/// gate whether or not the closure handling stands, which is a perturbation
/// that can never be seen to fail.
///
/// Perturbation: make the `Closed` arm of `serve` continue the loop instead of
/// returning and this test fails, because the gate never exits and the bounded
/// wait kills it. Watched under exactly that change.
#[test]
fn a_closed_channel_ends_the_gate_and_its_listener() {
    let path = socket_path("closure");
    let (harness, child_end) = seqpacket_pair();
    let (mut child, log_path) = spawn_gate(child_end.as_raw_fd());

    let ready = ask(
        &harness,
        1,
        LifecycleDirective::Raise {
            instruction: instruction(path.clone()),
        },
    );
    assert_eq!(
        ready.payload,
        Payload::Answer(LifecycleAnswer::GateReady),
        "the gate raises"
    );
    // Ready is a fact about a listener: a dial reaches it right now.
    let mut standing = UnixStream::connect(&path).expect("the raised hook accepts a dial");
    standing.write_all(b"x").ok();

    // The interior goes away. Nothing else is done to the gate.
    drop(harness);

    let status = wait_bounded(&mut child, 30, "a_closed_channel_ends_the_gate");
    assert!(
        status.success(),
        "closure is an orderly exit, stderr: {}",
        std::fs::read_to_string(&log_path).unwrap_or_default()
    );
    assert!(
        UnixStream::connect(&path).is_err(),
        "the listener died with the process that held it"
    );
    // The path survives: this crate unlinks nothing, even on the way out.
    assert!(path.exists(), "the operator's artifact is left where it was");
    std::fs::remove_file(&path).ok();
    std::fs::remove_file(&log_path).ok();
}

/// The raise-then-lower round trip across the real seam, into the real binary.
#[test]
fn raise_then_lower_round_trips_across_the_seam() {
    let path = socket_path("round-trip");
    let (harness, child_end) = seqpacket_pair();
    let (mut child, log_path) = spawn_gate(child_end.as_raw_fd());

    let ready = ask(
        &harness,
        1,
        LifecycleDirective::Raise {
            instruction: instruction(path.clone()),
        },
    );
    assert_eq!(ready.payload, Payload::Answer(LifecycleAnswer::GateReady));
    assert_eq!(ready.exchange.ordinal, 1);

    let stopped = ask(&harness, 2, LifecycleDirective::Lower);
    assert_eq!(stopped.payload, Payload::Answer(LifecycleAnswer::GateStopped));
    assert_eq!(stopped.exchange.ordinal, 2);
    assert!(
        UnixStream::connect(&path).is_err(),
        "stopped is answered only after the close returned"
    );

    drop(harness);
    let status = wait_bounded(&mut child, 30, "raise_then_lower_round_trips");
    assert!(status.success());
    std::fs::remove_file(&path).ok();
    std::fs::remove_file(&log_path).ok();
}
