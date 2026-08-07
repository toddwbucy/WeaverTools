//! conforms: gate-authenticated-at-accept
//! conforms: gate-agent-uid-denied-by-construction
//! conforms: gate-stopped-follows-close
//!
//! The reference walk of `weaver-gate-Spec` section 6, exercised against a real
//! listener and a real connecting process.
//!
//! **The reference walk: an elected tool dials the agent's own mouth.** The
//! adversary is the agent's tool surface running as the agent uid, the attack a
//! dial of the named socket the instruction declares, the prompt-yourself loop
//! the charter names. The mechanism is the predicate at accept with the agent's
//! own uid denied by construction.

use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use weaver_gate::hook::{AcceptOutcome, Hook};
use weaver_types::{AccessRule, GateInstruction};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "weaver-gate-boundary-{}-{name}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("a scratch dir");
    let path = dir.join("gate.sock");
    std::fs::remove_file(&path).ok();
    path
}

/// An instruction whose rule permits this very uid, which is the mistake the
/// construction has to survive.
fn permissive_instruction(path: PathBuf) -> GateInstruction {
    let mut allowed_uids = BTreeSet::new();
    allowed_uids.insert(nix::unistd::getuid().as_raw());
    GateInstruction {
        socket_path: path,
        access_rule: AccessRule {
            allowed_uids,
            allowed_gids: BTreeSet::new(),
            denied_uids: BTreeSet::new(),
        },
    }
}

/// **The agent uid is denied by construction, not by configuration.**
///
/// The instruction here explicitly *permits* this process's own uid, which is
/// the operator mistake the design exists to survive. The raise adds this uid
/// to the deny set unconditionally, and denial wins over permission, so the
/// dial is refused anyway.
///
/// The test dials as this very process, which runs as the same uid the gate
/// does. That is exactly the reference walk's adversary: the agent's own tool
/// surface reaching the agent's own mouth.
///
/// Perturbation: remove the `rule.denied_uids.insert(getuid())` line from
/// `Hook::raise` and this test fails, because the operator's permissive rule
/// then admits the dial. Watched under exactly that removal.
#[test]
fn the_agent_uid_is_refused_even_when_the_rule_permits_it() {
    let path = scratch("agent-uid");
    let hook = Hook::raise(&permissive_instruction(path.clone())).expect("the raise binds");

    assert!(
        hook.rule().denied_uids.contains(&nix::unistd::getuid().as_raw()),
        "the raise added this process's own uid to the deny set"
    );

    let mut client = UnixStream::connect(&path).expect("the dial reaches the listener");
    let outcome = hook.accept();
    assert_eq!(
        outcome.err(),
        Some(AcceptOutcome::Denied),
        "the agent's own uid is refused at accept"
    );

    // **Refused by closure, with nothing written back.** The client sees the
    // end of the connection rather than an answer: an admitted-looking reply to
    // a refused peer is a conversation the boundary already declined.
    let mut answer = Vec::new();
    client
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .expect("a read bound");
    // Strictly `Ok(0)`: the peer wrote nothing before the refusal, so the
    // close is orderly and the read reaches end-of-stream rather than a reset.
    // Accepting any error here would have let a written answer followed by a
    // reset pass as a refusal.
    let read = client.read_to_end(&mut answer);
    assert_eq!(
        read.ok(),
        Some(0),
        "the refused peer reaches end-of-stream with nothing written, got {answer:?}"
    );

    hook.lower();
    std::fs::remove_file(&path).ok();
}

/// **Every connection is authenticated at accept, before any byte is read.**
///
/// The client writes immediately on connecting. The refusal must land without
/// that content ever being read, which is what "before any content is read"
/// means: the accept judges the credential the kernel supplies, not anything
/// the peer says about itself.
///
/// Perturbation: move the `authorized` call in `Hook::accept` to after a read
/// of the stream and this test still passes on the refusal, which is why what
/// it asserts is the *absence of an answer* rather than the refusal alone.
/// Removing the `authorized` check entirely fails it. Watched under exactly
/// that removal.
#[test]
fn a_peer_is_judged_before_its_content_is_read() {
    let path = scratch("judged-first");
    let hook = Hook::raise(&permissive_instruction(path.clone())).expect("the raise binds");

    let mut client = UnixStream::connect(&path).expect("the dial reaches the listener");
    client
        .write_all(b"{\"this\":\"is never read\"}\n")
        .expect("the client speaks first");

    assert_eq!(
        hook.accept().err(),
        Some(AcceptOutcome::Denied),
        "the credential decides, not the content"
    );

    hook.lower();
    std::fs::remove_file(&path).ok();
}

/// **The lower closes the listener first and confirms after**, so nothing new
/// can arrive once the harness proceeds. Read from outside: a dial after the
/// lower is refused by the kernel, the listener being gone.
///
/// Perturbation: make `Hook::lower` leak the listener with `std::mem::forget`
/// and this test fails, because the dial after the lower still connects.
/// Watched under exactly that change.
#[test]
fn a_dial_after_the_lower_finds_no_listener() {
    let path = scratch("after-lower");
    let hook = Hook::raise(&permissive_instruction(path.clone())).expect("the raise binds");
    assert!(
        UnixStream::connect(&path).is_ok(),
        "the standing hook accepts a dial"
    );

    hook.lower();

    assert!(
        UnixStream::connect(&path).is_err(),
        "once lowered, the path is a stale entry and nothing is listening"
    );
    // And the path itself survives: this crate unlinks nothing.
    assert!(path.exists(), "the operator's artifact is left where it was");
    std::fs::remove_file(&path).ok();
}

/// **The bind refuses what it finds in the way, and unlinks nothing.**
///
/// Perturbation: add an unlink before the bind in `Hook::raise` and this test
/// fails, because the second raise then succeeds by deleting the first's
/// socket. Watched under exactly that addition.
#[test]
fn an_occupied_path_refuses_and_the_occupant_survives() {
    let path = scratch("occupied");
    let first = Hook::raise(&permissive_instruction(path.clone())).expect("the first binds");

    let second = Hook::raise(&permissive_instruction(path.clone()));
    assert!(second.is_err(), "an occupied path refuses the raise");

    // The first hook is untouched: its listener still answers.
    assert!(
        UnixStream::connect(&path).is_ok(),
        "the occupant was not unlinked out of the way"
    );

    first.lower();
    std::fs::remove_file(&path).ok();
}
