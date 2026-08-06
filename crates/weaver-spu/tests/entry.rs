//! conforms: spu-descriptor-count-check
//! conforms: spu-dumpable-flag-cleared
//!
//! The process facts of `weaver-spu-Spec` section 2, exercised as a process.
//!
//! These run the built binary rather than calling into it, because what they
//! assert is what the process holds and what the kernel says about it at the
//! moment after entry. A unit test inside the crate can reach neither: the
//! count check is about descriptors this process was handed, and the dumpable
//! flag is read from outside by an observer the process cannot lie to.

use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use nix::libc;
use nix::sys::socket::{AddressFamily, SockFlag, SockType, socketpair};

fn seqpacket_pair() -> (OwnedFd, OwnedFd) {
    socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::SOCK_CLOEXEC,
    )
    .expect("a socketpair")
}

/// Start the binary holding exactly the descriptors named, placed at 3, 4, and
/// upward in the order given.
///
/// The placement runs in two passes on purpose. A source descriptor can already
/// sit on a number this test is about to write, so writing the targets directly
/// would clobber a source that has not been placed yet. The first pass lifts
/// every source above the target range with `F_DUPFD`, and the second pass
/// writes the targets from those copies. `dup2` clears close-on-exec on the
/// target it writes, which is what lets these two survive the exec while every
/// other descriptor in this process closes.
fn spawn_holding(child_ends: Vec<RawFd>) -> Child {
    let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    unsafe {
        command.pre_exec(move || {
            // Pass one: lift every source clear of the target range.
            let mut lifted = Vec::with_capacity(child_ends.len());
            for source in &child_ends {
                let high = libc::fcntl(*source, libc::F_DUPFD, 32);
                if high < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                lifted.push(high);
            }
            // Pass two: write the targets, which also clears close-on-exec.
            for (offset, high) in lifted.iter().enumerate() {
                if libc::dup2(*high, 3 + offset as RawFd) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            // `F_DUPFD` CLEARS close-on-exec on the copy rather than inheriting
            // it, so these would survive the exec and inflate the very count
            // the child checks. Closing them is required, not tidiness.
            for high in lifted {
                libc::close(high);
            }
            Ok(())
        });
    }
    command.spawn().expect("the binary starts")
}

/// **Entry verifies that it holds exactly two descriptors beyond the standard
/// streams, and refuses to serve if it holds more.**
///
/// A count above two means the harness's fork discipline failed upstream and
/// this process is not the one to continue past it. The check is a count rather
/// than an identification, since this crate cannot know what a stray descriptor
/// refers to, so this test leaks a third socket rather than anything meaningful.
///
/// Perturbation: remove the `held != 2` branch from `adopt` and this test
/// fails, because the process proceeds to serve and blocks on its first read
/// instead of exiting. Watched under exactly that removal.
#[test]
fn a_leaked_descriptor_refuses_to_serve() {
    let (_lifecycle, child_lifecycle) = seqpacket_pair();
    let (_decode, child_decode) = seqpacket_pair();
    let (_leaked, child_leaked) = seqpacket_pair();

    let child = spawn_holding(vec![
        child_lifecycle.as_raw_fd(),
        child_decode.as_raw_fd(),
        child_leaked.as_raw_fd(),
    ]);
    let out = child.wait_with_output().expect("the binary exits");

    assert!(
        !out.status.success(),
        "a process holding a leaked descriptor refuses to serve"
    );
    let complaint = String::from_utf8_lossy(&out.stderr);
    assert!(
        complaint.contains("held_beyond_standard_streams\":3"),
        "the refusal reports the count it found, got {complaint}"
    );
}

/// The clean case: exactly two descriptors, so the count check passes and the
/// process serves. Without this, the test above would pass against a binary
/// that refused unconditionally.
#[test]
fn exactly_two_descriptors_serve() {
    let (lifecycle, child_lifecycle) = seqpacket_pair();
    let (_decode, child_decode) = seqpacket_pair();

    let mut child = spawn_holding(vec![child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()]);

    // A served process blocks on its first read rather than exiting. Closing
    // the parent's end is what ends it, and a clean exit is the proof it got
    // past entry.
    assert!(
        child.try_wait().expect("a wait succeeds").is_none(),
        "the process is still running, which means entry passed"
    );
    drop(lifecycle);
    let out = child.wait_with_output().expect("the binary exits");
    assert!(
        out.status.success(),
        "a process holding exactly two descriptors serves and exits clean, stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// **The dumpable flag is cleared at entry.**
///
/// The adversary is a same-uid process attaching by `ptrace` or reaching
/// through `/proc/[pid]/fd` into the process holding the weights and both
/// channels. The mechanism is a set and not a check.
///
/// **The observation is the kernel's rather than the process's.** Clearing the
/// flag transfers ownership of `/proc/[pid]` to `root:root`, so the parent reads
/// the fact from the filesystem and the child has no way to report it falsely.
/// A test that asked the child to attest would pass against a child that lied.
///
/// Perturbation: remove the `clear_dumpable` call from `adopt` and this test
/// fails, because `/proc/[pid]/fd` stays owned by the invoking uid. Watched
/// under exactly that removal.
#[test]
fn the_dumpable_flag_is_clear_after_entry() {
    use std::os::unix::fs::MetadataExt;

    let (lifecycle, child_lifecycle) = seqpacket_pair();
    let (_decode, child_decode) = seqpacket_pair();

    let mut child = spawn_holding(vec![child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()]);
    let pid = child.id();

    // Entry runs and then blocks on the first read. Poll rather than sleep a
    // fixed span, so the test is not a race on a loaded machine.
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

    drop(lifecycle);
    let _ = child.wait();

    assert_eq!(
        owner,
        Some(0),
        "a cleared dumpable flag makes /proc/{pid}/fd owned by root, found {owner:?}"
    );
}
