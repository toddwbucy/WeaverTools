//! conforms: gate-shell-the-one-held-tool
//! conforms: gate-execution-one-clock
//! conforms: gate-execution-four-contents
//! conforms: gate-execution-group-kill
//! conforms: gate-execution-drain-rides-the-run
//!
//! The shell execution, per `weaver-gate-Spec` section 8 and the tool
//! boundary ruling of 2026-08-18: this crate holds one tool, the shell, and
//! holds it as its own outbound verb rather than as a table's member. The
//! agent's effect on the world crosses this membrane, the shell is that
//! crossing's general form, and the uid it runs under is the agent's outer
//! protective shell. A name that is not the shell's refuses by name, never
//! a nearest match, and there is no dyn table to search: one verb,
//! dispatched directly.
//!
//! **One clock, the caller's.** Every invocation carries the caller's
//! timeout, validated here against the declared maximum and adopted as the
//! kill clock, refused past the maximum rather than clamped. The answer is
//! one of the contract's four contents, told apart by who speaks: a result
//! is the shell's own words - a nonzero exit included - a refusal is this
//! crate's voice with nothing run, an error is the machinery's, and a kill
//! carries no tool voice by construction.

use weaver_types::{ToolExecution, ToolOutcome};

/// The one held tool's name. The model calls the shell by the name the
/// shell answers to everywhere else.
pub const SHELL_NAME: &str = "bash";

/// The maximum clock the shell declares, in milliseconds. A caller may ask
/// for less and never more, per the one-clock rule.
pub const SHELL_MAX_CLOCK_MS: u64 = 60_000;

/// The capture bound per pipe. Output past it is drained and discarded so
/// the pipe empties, and the truncation marks itself in the answer.
const SHELL_OUTPUT_BOUND: usize = 32 * 1024;

/// Execute one call, answering one of the exchange's four contents.
pub fn execute(execution: &ToolExecution) -> ToolOutcome {
    if execution.name.0 != SHELL_NAME {
        return ToolOutcome::Refused {
            reason: format!(
                "no tool named {} is held - the shell, {SHELL_NAME}, is the one tool",
                execution.name.0
            ),
        };
    }
    if execution.clock_ms == 0 || execution.clock_ms > SHELL_MAX_CLOCK_MS {
        return ToolOutcome::Refused {
            reason: format!(
                "the caller's clock of {}ms is outside the shell's declared maximum of \
                 {SHELL_MAX_CLOCK_MS}ms",
                execution.clock_ms
            ),
        };
    }
    let parsed: serde_json::Value = match serde_json::from_str(&execution.arguments) {
        Ok(value) => value,
        Err(_) => {
            return ToolOutcome::Refused {
                reason: "the arguments are not one JSON object".to_string(),
            };
        }
    };
    let Some(command) = parsed.get("command").and_then(|value| value.as_str()) else {
        return ToolOutcome::Refused {
            reason: "the arguments carry no command string".to_string(),
        };
    };
    // The home comes from the account database rather than from the
    // environment: the organ fan-out execs with an empty environment on
    // purpose, and the account database is where the uid's home is a fact
    // rather than an inheritance.
    let Some(home) = nix::unistd::User::from_uid(nix::unistd::getuid())
        .ok()
        .flatten()
        .map(|user| user.dir)
    else {
        return ToolOutcome::Errored {
            detail: "this uid has no home in the account database".to_string(),
        };
    };
    match run_in_home(
        command,
        &home.to_string_lossy(),
        std::time::Duration::from_millis(execution.clock_ms),
    ) {
        Ok(content) => ToolOutcome::Result { content },
        Err(end) => end.into_outcome(),
    }
}

/// How the shell's run ended when it did not answer: the machinery failed,
/// or the caller's clock expired and the group was killed. The split is the
/// contract's - the speaker differs - and the conversion to the wire's
/// contents happens here so `run_in_home` stays a plain function.
enum ShellEnd {
    Errored { detail: String },
    Killed { partial: Option<String> },
}

impl ShellEnd {
    fn into_outcome(self) -> ToolOutcome {
        match self {
            ShellEnd::Errored { detail } => ToolOutcome::Errored { detail },
            ShellEnd::Killed { partial } => ToolOutcome::Killed { partial },
        }
    }
}

/// Fork, supervise to the caller's clock, and account for the exit - every
/// path one of the contract's contents, never a hung exchange.
fn run_in_home(
    command: &str,
    home: &str,
    deadline: std::time::Duration,
) -> Result<String, ShellEnd> {
    use std::os::unix::process::CommandExt;
    use std::process::{Command, Stdio};

    // The child leads its own process group, so the kill below reaches
    // `bash -c`'s descendants too: a background child would otherwise
    // inherit the pipe's write end and hold the readers open after the
    // shell itself exited.
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .current_dir(home)
        // The gate's own environment is deliberately empty; the command
        // still deserves to know where it lives.
        .env("HOME", home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
        .map_err(|error| ShellEnd::Errored {
            detail: format!("the fork failed: {error}"),
        })?;
    let group = nix::unistd::Pid::from_raw(child.id() as i32);

    // Both pipes drain concurrently with the run, each on its own thread:
    // a pipe left unread to the exit fills at the kernel's buffer and
    // blocks the child's writes, which would convert a chatty command into
    // a false deadline failure. The drain keeps the bound and discards the
    // rest, so the capture is bounded while the pipe still empties.
    let stdout = child.stdout.take().expect("stdout is piped");
    let stderr = child.stderr.take().expect("stderr is piped");
    let out_reader = std::thread::spawn(move || drain_bounded(stdout, SHELL_OUTPUT_BOUND));
    let err_reader = std::thread::spawn(move || drain_bounded(stderr, SHELL_OUTPUT_BOUND));

    // Supervision: poll to the deadline, kill the group past it. `std`
    // carries no bounded wait, so the poll sleeps in small steps - coarse
    // and sufficient for a bound whose unit is seconds.
    //
    // **The exit is observed with `WNOWAIT` and the leader reaped only
    // after the group is signaled.** A reaping poll would free the leader's
    // pid at the moment of exit, and the group kill that follows would
    // signal an id the kernel may already have reissued; unreaped, the
    // leader holds the group id reserved until the `wait` below.
    let started = std::time::Instant::now();
    let status = loop {
        use nix::sys::wait::{Id, WaitPidFlag, WaitStatus, waitid};
        match waitid(
            Id::Pid(group),
            WaitPidFlag::WEXITED | WaitPidFlag::WNOHANG | WaitPidFlag::WNOWAIT,
        ) {
            Ok(WaitStatus::StillAlive) => {
                if started.elapsed() > deadline {
                    let _ = nix::sys::signal::killpg(group, nix::sys::signal::Signal::SIGKILL);
                    let _ = child.wait();
                    let out_drained = out_reader.join().unwrap_or((Vec::new(), false)).0;
                    let err_drained = err_reader.join().unwrap_or((Vec::new(), false)).0;
                    let mut partial = String::from_utf8_lossy(&out_drained).into_owned();
                    let errors = String::from_utf8_lossy(&err_drained);
                    if !errors.is_empty() {
                        if !partial.is_empty() {
                            partial.push('\n');
                        }
                        partial.push_str("stderr: ");
                        partial.push_str(&errors);
                    }
                    return Err(ShellEnd::Killed {
                        partial: (!partial.is_empty()).then_some(partial),
                    });
                }
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            Ok(_) => {
                // The command has exited; the group dies with it. A
                // background child the command left behind still holds the
                // pipes' write ends, and the joins below would wait on it -
                // so the answer is what the foreground command produced,
                // and stragglers are ended, not adopted.
                let _ = nix::sys::signal::killpg(group, nix::sys::signal::Signal::SIGKILL);
                break child.wait().map_err(|error| ShellEnd::Errored {
                    detail: format!("the supervision failed: {error}"),
                })?;
            }
            Err(error) => {
                let _ = nix::sys::signal::killpg(group, nix::sys::signal::Signal::SIGKILL);
                let _ = child.wait();
                let _ = out_reader.join();
                let _ = err_reader.join();
                return Err(ShellEnd::Errored {
                    detail: format!("the supervision failed: {error}"),
                });
            }
        }
    };

    let (out_bytes, out_cut) = out_reader.join().unwrap_or((Vec::new(), false));
    let (err_bytes, err_cut) = err_reader.join().unwrap_or((Vec::new(), false));

    let mut output = String::from_utf8_lossy(&out_bytes).into_owned();
    let errors = String::from_utf8_lossy(&err_bytes);
    if !errors.is_empty() {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str("stderr: ");
        output.push_str(&errors);
    }
    if !status.success() {
        let code = status
            .code()
            .map(|code| code.to_string())
            .unwrap_or_else(|| "killed by signal".to_string());
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(&format!("exit status: {code}"));
    }
    if out_cut || err_cut || output.len() > SHELL_OUTPUT_BOUND {
        let mut cut = output.len().min(SHELL_OUTPUT_BOUND);
        while !output.is_char_boundary(cut) {
            cut -= 1;
        }
        output.truncate(cut);
        output.push_str("\n[output truncated at 32 KiB]");
    }
    if output.is_empty() {
        output.push_str("(no output)");
    }
    Ok(output)
}

/// Reads a pipe to its end, keeping at most `bound` octets and reporting
/// whether anything past the bound was discarded. The read continues past
/// the bound on purpose: stopping would refill the pipe and block the
/// writer, which is the deadlock the bound exists to avoid.
fn drain_bounded(mut pipe: impl std::io::Read, bound: usize) -> (Vec<u8>, bool) {
    let mut kept = Vec::new();
    let mut truncated = false;
    let mut buffer = [0u8; 8192];
    loop {
        match pipe.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => {
                if kept.len() < bound {
                    let take = (bound - kept.len()).min(read);
                    kept.extend_from_slice(&buffer[..take]);
                    if take < read {
                        truncated = true;
                    }
                } else {
                    truncated = true;
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
        }
    }
    (kept, truncated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaver_types::ToolName;

    fn shell(arguments: &str, clock_ms: u64) -> ToolOutcome {
        execute(&ToolExecution {
            name: ToolName(SHELL_NAME.into()),
            arguments: arguments.into(),
            clock_ms,
        })
    }

    /// **A name that is not the shell's refuses by name, in this crate's
    /// own voice, and nothing runs.** The one verb has no nearest match.
    #[test]
    fn a_name_that_is_not_the_shells_refuses_by_name() {
        let outcome = execute(&ToolExecution {
            name: ToolName("calculator".into()),
            arguments: r#"{"command":"true"}"#.into(),
            clock_ms: 1_000,
        });
        let ToolOutcome::Refused { reason } = outcome else {
            panic!("a refusal answers: {outcome:?}");
        };
        assert!(
            reason.contains("no tool named calculator"),
            "the refusal names the name: {reason}"
        );
    }

    /// **The clock is validated before anything runs, refused and never
    /// clamped**: zero asks for nothing and past-maximum asks for more than
    /// the shell declares, and both refuse naming the bound.
    #[test]
    fn a_clock_outside_the_maximum_refuses_rather_than_clamps() {
        for clock in [0, SHELL_MAX_CLOCK_MS + 1] {
            let outcome = shell(r#"{"command":"true"}"#, clock);
            let ToolOutcome::Refused { reason } = outcome else {
                panic!("a refusal answers for {clock}: {outcome:?}");
            };
            assert!(
                reason.contains("declared maximum"),
                "the refusal names the bound: {reason}"
            );
        }
    }

    /// Malformed arguments are the gate's refusal - nothing ran, so no
    /// account is the tool's.
    #[test]
    fn malformed_arguments_refuse_in_the_gates_voice() {
        for (arguments, expected) in [
            ("not json", "the arguments are not one JSON object"),
            (
                r#"{"expr":"true"}"#,
                "the arguments carry no command string",
            ),
        ] {
            let outcome = shell(arguments, 1_000);
            let ToolOutcome::Refused { reason } = outcome else {
                panic!("a refusal answers: {outcome:?}");
            };
            assert!(reason.contains(expected), "for {arguments}: {reason}");
        }
    }

    /// **The shell runs where it promises and accounts for every ending**:
    /// output crosses, stderr is named, and a nonzero exit is a result -
    /// the shell's own answer - never an error.
    #[test]
    fn the_shell_runs_where_it_promises() {
        let outcome = shell(r#"{"command":"pwd"}"#, 10_000);
        let ToolOutcome::Result { content } = outcome else {
            panic!("pwd answers: {outcome:?}");
        };
        let home = std::env::var("HOME").expect("the suite has a HOME");
        let expected = std::fs::canonicalize(&home).expect("the home resolves");
        let reported = std::fs::canonicalize(content.trim()).expect("the pwd resolves");
        assert_eq!(reported, expected, "the command runs in the home directory");

        let outcome = shell(r#"{"command":"echo out; echo err 1>&2; exit 3"}"#, 10_000);
        let ToolOutcome::Result { content } = outcome else {
            panic!("a nonzero exit is still a result: {outcome:?}");
        };
        assert!(content.contains("out"), "stdout crosses: {content}");
        assert!(
            content.contains("stderr: err"),
            "stderr is named: {content}"
        );
        assert!(
            content.contains("exit status: 3"),
            "the exit is named: {content}"
        );
    }

    /// **A chatty command drains while it runs and a straggler does not
    /// hold the answer.** The first writes past the kernel's pipe buffer
    /// and must complete promptly with the bound's marker - a drain that
    /// waited for the exit would block the child's writes and convert it
    /// into a false kill. The second leaves a background child holding the
    /// pipe's write end, and the group kill at the exit is what lets the
    /// readers finish.
    #[test]
    fn a_chatty_command_and_a_straggler_both_answer_promptly() {
        let started = std::time::Instant::now();
        let outcome = shell(
            r#"{"command":"head -c 200000 /dev/zero | tr '\\0' 'x'"}"#,
            10_000,
        );
        let ToolOutcome::Result { content } = outcome else {
            panic!("a chatty command still answers: {outcome:?}");
        };
        assert!(
            content.contains("[output truncated at 32 KiB]"),
            "the bound marks itself: {} octets",
            content.len()
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "the drain rode along, no clock was consumed"
        );

        let started = std::time::Instant::now();
        let outcome = shell(r#"{"command":"echo done; sleep 30 &"}"#, 10_000);
        let ToolOutcome::Result { content } = outcome else {
            panic!("the foreground command answers: {outcome:?}");
        };
        assert!(content.contains("done"), "the answer crossed: {content}");
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "the straggler did not hold the answer open"
        );
    }

    /// **A command past the caller's clock is killed as its own case,
    /// carrying no tool voice and attaching what drained before the
    /// kill.** The clock here is tiny because the constant is what a
    /// deployment tunes and the mechanism is what this watches.
    #[test]
    fn a_command_past_the_clock_is_killed_with_its_partial_attached() {
        let outcome = shell(r#"{"command":"echo early; sleep 5"}"#, 200);
        let ToolOutcome::Killed { partial } = outcome else {
            panic!("the kill is its own case: {outcome:?}");
        };
        let partial = partial.expect("the drained output rides the kill");
        assert!(
            partial.contains("early"),
            "what drained before the kill is attached: {partial}"
        );

        let outcome = shell(r#"{"command":"sleep 5"}"#, 200);
        let ToolOutcome::Killed { partial } = outcome else {
            panic!("the kill is its own case: {outcome:?}");
        };
        assert!(
            partial.is_none(),
            "a silent command attaches nothing: {partial:?}"
        );
    }
}
