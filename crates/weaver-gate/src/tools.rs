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
    vec![Box::new(Calculator)]
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
        r#"{"name":"calculator","description":"Evaluate an arithmetic expression.","parameters":{"type":"object","properties":{"expression":{"type":"string","description":"An arithmetic expression over + - * / and parentheses."}},"required":["expression"]}}"#
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

/// A recursive-descent evaluation of `+ - * /`, unary minus, and parentheses.
/// No names, no calls, no assignment: an expression the grammar does not
/// cover is the tool's failure, in words that name the position.
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
    let mut value = parse_atom(tokens, position)?;
    while let Some(&op) = tokens.get(*position) {
        match op {
            '*' => {
                *position += 1;
                value *= parse_atom(tokens, position)?;
            }
            '/' => {
                *position += 1;
                let divisor = parse_atom(tokens, position)?;
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

fn parse_atom(tokens: &[char], position: &mut usize) -> Result<f64, String> {
    match tokens.get(*position) {
        Some('-') => {
            *position += 1;
            Ok(-parse_atom(tokens, position)?)
        }
        Some('(') => {
            *position += 1;
            let value = parse_sum(tokens, position)?;
            if tokens.get(*position) != Some(&')') {
                return Err(format!("unclosed parenthesis at position {position}"));
            }
            *position += 1;
            Ok(value)
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
