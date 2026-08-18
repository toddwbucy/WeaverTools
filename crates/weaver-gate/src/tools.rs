//! The tools this gate holds, and the execution that answers for them, per
//! the tool workflow's opening act and `weaver-harness-gate-contract`
//! section 2.
//!
//! **The gate executes and the harness dispatches the exchange**, per the
//! mechanism election of `weaver-gate-PRD` section 7: the loop reaches tools
//! through this crate, so execution lives on this side of the loop's
//! membrane and the result crosses back exactly once, as the exchange's
//! answer. The calculator computes in this process and forks nothing; the
//! per-call fork the charter's mechanism names engages with the first
//! subprocess tool, which is not this act's, and the trait is indifferent to
//! which a tool does.
//!
//! **A name this table does not hold is refused by name, never a nearest
//! match** - the family registry's discipline one organ over - and the
//! refusal is content, because a tool that does not exist is a fact the
//! model must learn.

use weaver_traits::{Tool, ToolFailure};
use weaver_types::{ToolExecution, ToolOutcome};

/// The tools this binary carries. The calculator is the reference tool the
/// program's order of work names, and the table grows by acts rather than by
/// configuration: what the operator's `tool-set` elects from is what is
/// compiled here.
fn held() -> Vec<Box<dyn Tool + Send>> {
    vec![Box::new(Calculator), Box::new(HomeCli)]
}

/// Execute one call against the held table, answering one of the exchange's
/// three contents.
pub fn execute(execution: &ToolExecution) -> ToolOutcome {
    for tool in held() {
        if tool.name() == execution.name.0 {
            return match block_on(tool.execute(&execution.arguments)) {
                Ok(content) => ToolOutcome::Result { content },
                Err(ToolFailure { detail }) => ToolOutcome::Failure { detail },
            };
        }
    }
    ToolOutcome::Unheld {
        name: execution.name.clone(),
    }
}

/// Drive one tool future to completion on this thread, std only.
///
/// The trait's future is boxed and `Send` for the general case; this crate
/// holds no runtime, per its own charter's no-runtime rule, so it polls with
/// a thread-parking waker. A ready-immediate future, the calculator's case,
/// completes on the first poll and parks nothing.
fn block_on<T>(
    mut future: std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + '_>>,
) -> T {
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    struct Parker(std::thread::Thread);
    impl Wake for Parker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }

    let waker = Waker::from(Arc::new(Parker(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::park(),
        }
    }
}

/// The reference tool: arithmetic over `+ - * /` and parentheses, computed in
/// this process. The arguments are one JSON object carrying `expression`, per
/// the schema it advertises, and everything wrong with a call is the tool's
/// own failure in its own words - content the model reasons over.
pub struct Calculator;

impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }

    fn schema(&self) -> &str {
        r#"{"name":"calculator","description":"Evaluate a scientific expression.","parameters":{"type":"object","properties":{"expression":{"type":"string","description":"An expression over + - * / ^, parentheses, the constants pi and e, and the functions sin cos tan asin acos atan sqrt ln log exp abs (radians, log base 10)."}},"required":["expression"]}}"#
    }

    fn execute(
        &self,
        arguments: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, ToolFailure>> + Send + '_>>
    {
        let arguments = arguments.to_string();
        Box::pin(async move {
            let parsed: serde_json::Value =
                serde_json::from_str(&arguments).map_err(|_| ToolFailure {
                    detail: "the arguments are not one JSON object".to_string(),
                })?;
            let expression = parsed
                .get("expression")
                .and_then(|value| value.as_str())
                .ok_or_else(|| ToolFailure {
                    detail: "the arguments carry no expression string".to_string(),
                })?;
            let value = evaluate(expression).map_err(|detail| ToolFailure { detail })?;
            // Integers render without the trailing zero a float would carry,
            // because "1591" is what a model quotes and "1591.0" invites it
            // to copy the artifact.
            Ok(if value.fract() == 0.0 && value.abs() < 1e15 {
                format!("{}", value as i64)
            } else {
                format!("{value}")
            })
        })
    }
}

/// How long one command may run before the fork is killed, the composition
/// bound like the harness's tool-round cap: a command that hangs would
/// otherwise hold the turn open without limit, and the kill is the tool's
/// own failure in its own words rather than a hung exchange.
const HOME_CLI_DEADLINE: std::time::Duration = std::time::Duration::from_secs(30);

/// How much combined output crosses back, under the organ envelope's 64 KiB
/// bound with room for the answer's own framing. Past it the output truncates
/// with a named marker, because a model reasoning over silently missing bytes
/// is worse than one told the output was cut.
const HOME_CLI_OUTPUT_BOUND: usize = 32 * 1024;

/// **The first subprocess tool, and the per-call fork the charter's
/// mechanism names.** One `bash -c` per call, forked by this crate and
/// supervised to a deadline, working directory the home of the uid this
/// process runs as - which is the agent's own home under the deployment's
/// boundary, and the kernel bound the charter states: what the command
/// reaches is what the uid reaches, no more, no classification asked.
pub struct HomeCli;

impl Tool for HomeCli {
    fn name(&self) -> &str {
        "home_cli"
    }

    fn schema(&self) -> &str {
        r#"{"name":"home_cli","description":"Run a shell command in the agent's own home directory.","parameters":{"type":"object","properties":{"command":{"type":"string","description":"A bash command line. Runs with the agent's identity, in its home directory, killed after 30 seconds."}},"required":["command"]}}"#
    }

    fn execute(
        &self,
        arguments: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, ToolFailure>> + Send + '_>>
    {
        let arguments = arguments.to_string();
        Box::pin(async move {
            let parsed: serde_json::Value =
                serde_json::from_str(&arguments).map_err(|_| ToolFailure {
                    detail: "the arguments are not one JSON object".to_string(),
                })?;
            let command = parsed
                .get("command")
                .and_then(|value| value.as_str())
                .ok_or_else(|| ToolFailure {
                    detail: "the arguments carry no command string".to_string(),
                })?;
            let home = std::env::var("HOME").map_err(|_| ToolFailure {
                detail: "this process has no HOME to run in".to_string(),
            })?;
            run_in_home(command, &home)
        })
    }
}

/// Fork, supervise to the deadline, and account for the exit - every path a
/// content answer or the tool's own failure, never a hung exchange.
fn run_in_home(command: &str, home: &str) -> Result<String, ToolFailure> {
    run_in_home_with_deadline(command, home, HOME_CLI_DEADLINE)
}

fn run_in_home_with_deadline(
    command: &str,
    home: &str,
    deadline: std::time::Duration,
) -> Result<String, ToolFailure> {
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
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
        .map_err(|error| ToolFailure {
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
    let out_reader = std::thread::spawn(move || drain_bounded(stdout, HOME_CLI_OUTPUT_BOUND));
    let err_reader = std::thread::spawn(move || drain_bounded(stderr, HOME_CLI_OUTPUT_BOUND));

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
                    let _ = out_reader.join();
                    let _ = err_reader.join();
                    return Err(ToolFailure {
                        detail: format!(
                            "the command ran past the {}s deadline and was killed",
                            deadline.as_secs()
                        ),
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
                break child.wait().map_err(|error| ToolFailure {
                    detail: format!("the supervision failed: {error}"),
                })?;
            }
            Err(error) => {
                let _ = nix::sys::signal::killpg(group, nix::sys::signal::Signal::SIGKILL);
                let _ = child.wait();
                let _ = out_reader.join();
                let _ = err_reader.join();
                return Err(ToolFailure {
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
    if out_cut || err_cut || output.len() > HOME_CLI_OUTPUT_BOUND {
        let mut cut = output.len().min(HOME_CLI_OUTPUT_BOUND);
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

/// A recursive-descent evaluation of a scientific expression grammar:
/// `+ - * / ^`, unary minus, parentheses, the constants `pi` and `e`, and
/// the one-argument functions `sin cos tan asin acos atan sqrt ln log exp
/// abs`, angles in radians, `log` base ten. No assignment, no user names, no
/// calls beyond the function set: an expression the grammar does not cover
/// is the tool's failure, in words that name the position.
/// The nesting an expression may reach before it refuses. The expression
/// is the model's, so its length and nesting are nobody's promise, and the
/// recursive descent below is bounded so the process answers a hostile
/// depth in the tool's own words instead of overflowing its stack.
const MAX_DEPTH: usize = 64;

fn evaluate(expression: &str) -> Result<f64, String> {
    let tokens: Vec<char> = expression.chars().filter(|c| !c.is_whitespace()).collect();
    let mut position = 0usize;
    let value = parse_sum(&tokens, &mut position, 0)?;
    if position != tokens.len() {
        return Err(format!("unexpected character at position {position}"));
    }
    if !value.is_finite() {
        return Err("the result is not a finite number".to_string());
    }
    Ok(value)
}

fn parse_sum(tokens: &[char], position: &mut usize, depth: usize) -> Result<f64, String> {
    if depth >= MAX_DEPTH {
        return Err(format!("the expression nests deeper than {MAX_DEPTH}"));
    }
    let mut value = parse_product(tokens, position, depth)?;
    while let Some(&op) = tokens.get(*position) {
        match op {
            '+' => {
                *position += 1;
                value += parse_product(tokens, position, depth)?;
            }
            '-' => {
                *position += 1;
                value -= parse_product(tokens, position, depth)?;
            }
            _ => break,
        }
    }
    Ok(value)
}

fn parse_product(tokens: &[char], position: &mut usize, depth: usize) -> Result<f64, String> {
    let mut value = parse_power(tokens, position, depth)?;
    while let Some(&op) = tokens.get(*position) {
        match op {
            '*' => {
                *position += 1;
                value *= parse_power(tokens, position, depth + 1)?;
            }
            '/' => {
                *position += 1;
                let divisor = parse_power(tokens, position, depth + 1)?;
                if divisor == 0.0 {
                    return Err("division by zero".to_string());
                }
                value /= divisor;
            }
            _ => break,
        }
    }
    Ok(value)
}

/// `^` binds tighter than `*` and `/` and associates right, so `2^3^2` is
/// `2^(3^2)` and `-2^2` is `-(2^2)`, the conventions a scientific reader
/// expects.
fn parse_power(tokens: &[char], position: &mut usize, depth: usize) -> Result<f64, String> {
    // The bound holds here as well as at `parse_sum`: this level recurses
    // into itself for the unary minus and for the exponent, and a chain of
    // either reaches no `parse_sum` on the way down.
    if depth >= MAX_DEPTH {
        return Err(format!("the expression nests deeper than {MAX_DEPTH}"));
    }
    // Unary minus lives at this level and binds looser than `^`, so `-2^2`
    // is `-(2^2)` and an exponent's own sign, `2^-3`, recurses through here.
    if tokens.get(*position) == Some(&'-') {
        *position += 1;
        return Ok(-parse_power(tokens, position, depth + 1)?);
    }
    let base = parse_atom(tokens, position, depth)?;
    if tokens.get(*position) == Some(&'^') {
        *position += 1;
        let exponent = parse_power(tokens, position, depth + 1)?;
        return Ok(base.powf(exponent));
    }
    Ok(base)
}

fn parse_atom(tokens: &[char], position: &mut usize, depth: usize) -> Result<f64, String> {
    match tokens.get(*position) {
        Some('(') => {
            *position += 1;
            let value = parse_sum(tokens, position, depth + 1)?;
            if tokens.get(*position) != Some(&')') {
                return Err(format!("unclosed parenthesis at position {position}"));
            }
            *position += 1;
            Ok(value)
        }
        Some(c) if c.is_ascii_alphabetic() => {
            let start = *position;
            while tokens
                .get(*position)
                .is_some_and(|c| c.is_ascii_alphabetic())
            {
                *position += 1;
            }
            let word: String = tokens[start..*position].iter().collect();
            match word.as_str() {
                "pi" => return Ok(std::f64::consts::PI),
                "e" => return Ok(std::f64::consts::E),
                _ => {}
            }
            if tokens.get(*position) != Some(&'(') {
                return Err(format!("unknown name {word} at position {start}"));
            }
            *position += 1;
            let argument = parse_sum(tokens, position, depth + 1)?;
            if tokens.get(*position) != Some(&')') {
                return Err(format!("unclosed call to {word} at position {position}"));
            }
            *position += 1;
            match word.as_str() {
                "sin" => Ok(argument.sin()),
                "cos" => Ok(argument.cos()),
                "tan" => Ok(argument.tan()),
                "asin" => Ok(argument.asin()),
                "acos" => Ok(argument.acos()),
                "atan" => Ok(argument.atan()),
                "sqrt" => {
                    if argument < 0.0 {
                        return Err("square root of a negative number".to_string());
                    }
                    Ok(argument.sqrt())
                }
                "ln" => {
                    if argument <= 0.0 {
                        return Err("logarithm of a non-positive number".to_string());
                    }
                    Ok(argument.ln())
                }
                "log" => {
                    if argument <= 0.0 {
                        return Err("logarithm of a non-positive number".to_string());
                    }
                    Ok(argument.log10())
                }
                "exp" => Ok(argument.exp()),
                "abs" => Ok(argument.abs()),
                _ => Err(format!("unknown function {word} at position {start}")),
            }
        }
        Some(c) if c.is_ascii_digit() || *c == '.' => {
            let start = *position;
            while tokens
                .get(*position)
                .is_some_and(|c| c.is_ascii_digit() || *c == '.')
            {
                *position += 1;
            }
            let text: String = tokens[start..*position].iter().collect();
            text.parse::<f64>()
                .map_err(|_| format!("not a number at position {start}"))
        }
        _ => Err(format!("expected a value at position {position}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaver_types::ToolName;

    /// The calculator computes, and its answers are the strings a model
    /// quotes: integers without the float's trailing zero.
    #[test]
    fn the_calculator_computes() {
        let outcome = execute(&ToolExecution {
            name: ToolName("calculator".into()),
            arguments: r#"{"expression":"37 * 43"}"#.into(),
        });
        assert_eq!(
            outcome,
            ToolOutcome::Result {
                content: "1591".into()
            }
        );
    }

    /// **Everything wrong with a call is the tool's failure in its own
    /// words**, content and never a fault: bad JSON, a missing expression, a
    /// grammar miss, division by zero.
    #[test]
    fn a_bad_call_fails_in_the_tools_own_words() {
        let deep = format!(r#"{{"expression":"{}1"}}"#, "(".repeat(90));
        let minus_chain = format!(r#"{{"expression":"{}1"}}"#, "-".repeat(90));
        let power_chain = format!(r#"{{"expression":"1{}"}}"#, "^1".repeat(90));
        let failing = [
            ("not json", "the arguments are not one JSON object"),
            (
                r#"{"expr":"1"}"#,
                "the arguments carry no expression string",
            ),
            (r#"{"expression":"1 +"}"#, "expected a value at position 2"),
            (r#"{"expression":"1/0"}"#, "division by zero"),
            (
                r#"{"expression":"sqrt(-1)"}"#,
                "square root of a negative number",
            ),
            (
                r#"{"expression":"ln(0)"}"#,
                "logarithm of a non-positive number",
            ),
            (
                r#"{"expression":"bogus(1)"}"#,
                "unknown function bogus at position 0",
            ),
            (r#"{"expression":"x + 1"}"#, "unknown name x at position 0"),
            (deep.as_str(), "the expression nests deeper than 64"),
            (minus_chain.as_str(), "the expression nests deeper than 64"),
            (power_chain.as_str(), "the expression nests deeper than 64"),
        ];
        for (arguments, expected) in failing {
            let outcome = execute(&ToolExecution {
                name: ToolName("calculator".into()),
                arguments: arguments.into(),
            });
            assert_eq!(
                outcome,
                ToolOutcome::Failure {
                    detail: expected.into()
                },
                "for {arguments}"
            );
        }
    }

    /// **The home CLI forks, answers, and accounts for every ending.** The
    /// commands are deliberately harmless - the suite runs at whatever uid
    /// builds the tree - and what is asserted is the mechanism: output
    /// crosses, stderr is named, a nonzero exit is named, and the working
    /// directory is the home the tool promises.
    #[test]
    fn the_home_cli_runs_where_it_promises() {
        let outcome = execute(&ToolExecution {
            name: ToolName("home_cli".into()),
            arguments: r#"{"command":"pwd"}"#.into(),
        });
        let ToolOutcome::Result { content } = outcome else {
            panic!("pwd answers: {outcome:?}");
        };
        let home = std::env::var("HOME").expect("the suite has a HOME");
        let expected = std::fs::canonicalize(&home).expect("the home resolves");
        let reported = std::fs::canonicalize(content.trim()).expect("the pwd resolves");
        assert_eq!(reported, expected, "the command runs in the home directory");

        let outcome = execute(&ToolExecution {
            name: ToolName("home_cli".into()),
            arguments: r#"{"command":"echo out; echo err 1>&2; exit 3"}"#.into(),
        });
        let ToolOutcome::Result { content } = outcome else {
            panic!("a nonzero exit is still an answer: {outcome:?}");
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
    /// hold the answer.** The first command writes past the kernel's pipe
    /// buffer and must complete promptly with the bound's marker - a drain
    /// that waited for the exit would block the child's writes and convert
    /// it into a false deadline kill. The second leaves a background child
    /// holding the pipe's write end; the group kill at the exit is what
    /// lets the readers finish, so the answer arrives in the foreground
    /// command's time, not the straggler's.
    #[test]
    fn a_chatty_command_and_a_straggler_both_answer_promptly() {
        let home = std::env::var("HOME").expect("the suite has a HOME");

        let started = std::time::Instant::now();
        let content = super::run_in_home_with_deadline(
            "head -c 200000 /dev/zero | tr '\\0' 'x'",
            &home,
            std::time::Duration::from_secs(10),
        )
        .expect("a chatty command still answers");
        assert!(
            content.contains("[output truncated at 32 KiB]"),
            "the bound marks itself: {} octets",
            content.len()
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "the drain rode along, no deadline was consumed"
        );

        let started = std::time::Instant::now();
        let content = super::run_in_home_with_deadline(
            "echo done; sleep 30 &",
            &home,
            std::time::Duration::from_secs(10),
        )
        .expect("the foreground command answers");
        assert!(content.contains("done"), "the answer crossed: {content}");
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "the straggler did not hold the answer open"
        );
    }

    /// The deadline is the tool's own failure in its own words, not a hung
    /// exchange: the kill path watched directly through the supervisor with
    /// a tiny bound, the constant being what a deployment tunes.
    #[test]
    fn a_command_past_the_deadline_is_killed_and_says_so() {
        let failure = super::run_in_home_with_deadline(
            "sleep 5",
            &std::env::var("HOME").expect("HOME"),
            std::time::Duration::from_millis(100),
        )
        .expect_err("the sleep must be killed");
        assert!(
            failure.detail.contains("deadline and was killed"),
            "the kill names itself: {}",
            failure.detail
        );
    }

    /// **A name the table does not hold refuses by name, never a nearest
    /// match.** Perturbation: make `execute` fall back to the first held tool
    /// on a miss and this fails, the outcome then being a `Result`.
    #[test]
    fn an_unheld_name_is_refused_by_name() {
        let outcome = execute(&ToolExecution {
            name: ToolName("calculatr".into()),
            arguments: "{}".into(),
        });
        assert_eq!(
            outcome,
            ToolOutcome::Unheld {
                name: ToolName("calculatr".into())
            }
        );
    }

    /// Parentheses, precedence, unary minus, and floats hold together.
    #[test]
    fn the_grammar_covers_its_claims() {
        for (expression, expected) in [
            ("2 + 3 * 4", "14"),
            ("(2 + 3) * 4", "20"),
            ("-(2 + 3) * 4", "-20"),
            ("1 / 4", "0.25"),
            ("10 - 2 - 3", "5"),
            ("2^10", "1024"),
            ("2^3^2", "512"),
            ("-2^2", "-4"),
            ("sqrt(16) + 2", "6"),
            ("cos(0)", "1"),
            ("ln(e)", "1"),
            ("log(1000)", "3"),
            ("abs(-3) * exp(0)", "3"),
            ("sin(pi / 2)", "1"),
        ] {
            let outcome = execute(&ToolExecution {
                name: ToolName("calculator".into()),
                arguments: format!(r#"{{"expression":"{expression}"}}"#),
            });
            assert_eq!(
                outcome,
                ToolOutcome::Result {
                    content: expected.into()
                },
                "for {expression}"
            );
        }
    }
}
