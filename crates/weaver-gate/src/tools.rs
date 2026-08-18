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
    use std::process::{Command, Stdio};

    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .current_dir(home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| ToolFailure {
            detail: format!("the fork failed: {error}"),
        })?;

    // Supervision: poll to the deadline, kill past it. `std` carries no
    // bounded wait, so the poll sleeps in small steps - coarse and
    // sufficient for a bound whose unit is seconds.
    let started = std::time::Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if started.elapsed() > deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(ToolFailure {
                        detail: format!(
                            "the command ran past the {}s deadline and was killed",
                            deadline.as_secs()
                        ),
                    });
                }
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            Err(error) => {
                return Err(ToolFailure {
                    detail: format!("the supervision failed: {error}"),
                });
            }
        }
    };

    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        use std::io::Read;
        let _ = stdout.read_to_string(&mut output);
    }
    let mut errors = String::new();
    if let Some(mut stderr) = child.stderr.take() {
        use std::io::Read;
        let _ = stderr.read_to_string(&mut errors);
    }
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
    if output.len() > HOME_CLI_OUTPUT_BOUND {
        let mut cut = HOME_CLI_OUTPUT_BOUND;
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

/// A recursive-descent evaluation of a scientific expression grammar:
/// `+ - * / ^`, unary minus, parentheses, the constants `pi` and `e`, and
/// the one-argument functions `sin cos tan asin acos atan sqrt ln log exp
/// abs`, angles in radians, `log` base ten. No assignment, no user names, no
/// calls beyond the function set: an expression the grammar does not cover
/// is the tool's failure, in words that name the position.
fn evaluate(expression: &str) -> Result<f64, String> {
    let tokens: Vec<char> = expression.chars().filter(|c| !c.is_whitespace()).collect();
    let mut position = 0usize;
    let value = parse_sum(&tokens, &mut position)?;
    if position != tokens.len() {
        return Err(format!("unexpected character at position {position}"));
    }
    if !value.is_finite() {
        return Err("the result is not a finite number".to_string());
    }
    Ok(value)
}

fn parse_sum(tokens: &[char], position: &mut usize) -> Result<f64, String> {
    let mut value = parse_product(tokens, position)?;
    while let Some(&op) = tokens.get(*position) {
        match op {
            '+' => {
                *position += 1;
                value += parse_product(tokens, position)?;
            }
            '-' => {
                *position += 1;
                value -= parse_product(tokens, position)?;
            }
            _ => break,
        }
    }
    Ok(value)
}

fn parse_product(tokens: &[char], position: &mut usize) -> Result<f64, String> {
    let mut value = parse_power(tokens, position)?;
    while let Some(&op) = tokens.get(*position) {
        match op {
            '*' => {
                *position += 1;
                value *= parse_power(tokens, position)?;
            }
            '/' => {
                *position += 1;
                let divisor = parse_power(tokens, position)?;
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
fn parse_power(tokens: &[char], position: &mut usize) -> Result<f64, String> {
    // Unary minus lives at this level and binds looser than `^`, so `-2^2`
    // is `-(2^2)` and an exponent's own sign, `2^-3`, recurses through here.
    if tokens.get(*position) == Some(&'-') {
        *position += 1;
        return Ok(-parse_power(tokens, position)?);
    }
    let base = parse_atom(tokens, position)?;
    if tokens.get(*position) == Some(&'^') {
        *position += 1;
        let exponent = parse_power(tokens, position)?;
        return Ok(base.powf(exponent));
    }
    Ok(base)
}

fn parse_atom(tokens: &[char], position: &mut usize) -> Result<f64, String> {
    match tokens.get(*position) {
        Some('(') => {
            *position += 1;
            let value = parse_sum(tokens, position)?;
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
            let argument = parse_sum(tokens, position)?;
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
        assert_eq!(
            content.trim(),
            std::env::var("HOME").expect("the suite has a HOME"),
            "the command runs in the home directory"
        );

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
