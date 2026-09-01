//! The invocation's composition root, and nothing else, per
//! `weaver-analysis-Spec` section 1: the operator's three acts, each a
//! subcommand, each taking a byte stream from its invocation. The socket it
//! dials is opened under whatever identity this process was invoked with,
//! and none is minted, per Spec section 4.

use std::io::Read;

use weaver_analysis::{AnalystInputs, Gated, parse_record};

fn main() -> std::process::ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let refused = |why: &str| {
        eprintln!("{}", serde_json::json!({"analysis_refusal": why}));
        std::process::ExitCode::FAILURE
    };
    match arguments.split_first().map(|(v, rest)| (v.as_str(), rest)) {
        Some(("derive", rest)) => run_derive(rest),
        Some(("preload", [trace, socket])) => run_preload(trace, socket),
        Some(("read", [trace])) => run_read(trace),
        _ => refused(
            "usage: weaver-analysis derive <trace> --devices <n,..> --sink <path> \
             [--readout] [--field-depth <n>] [--surprisal] | preload <trace> <socket> \
             | read <diagnostic-trace>",
        ),
    }
}

fn read_stream(path: &str) -> std::io::Result<String> {
    let mut text = String::new();
    if path == "-" {
        std::io::stdin().read_to_string(&mut text)?;
    } else {
        std::fs::File::open(path)?.read_to_string(&mut text)?;
    }
    Ok(text)
}

fn run_derive(rest: &[String]) -> std::process::ExitCode {
    let mut trace = None;
    let mut inputs = AnalystInputs {
        devices: Vec::new(),
        readout: false,
        field_depth: None,
        surprisal: false,
        sink_path: String::new(),
    };
    let mut out: Option<String> = None;
    let refused = |why: String| {
        eprintln!("{}", serde_json::json!({"analysis_refusal": why}));
        std::process::ExitCode::FAILURE
    };
    let mut it = rest.iter();
    while let Some(argument) = it.next() {
        match argument.as_str() {
            // **A numeric token that does not parse refuses naming it**,
            // never a silent discard: a device list that dropped a
            // malformed entry, or a field depth that quietly became no
            // election, would run a diagnostic the analyst did not ask
            // for.
            "--devices" => {
                let Some(value) = it.next() else {
                    return refused("--devices takes a value".to_string());
                };
                let mut devices = Vec::new();
                for token in value.split(',') {
                    match token.parse() {
                        Ok(device) => devices.push(device),
                        Err(_) => {
                            return refused(format!(
                                "--devices holds a token that is not a device ordinal: {token}"
                            ));
                        }
                    }
                }
                inputs.devices = devices;
            }
            "--sink" => {
                let Some(value) = it.next().filter(|v| !v.starts_with("--")) else {
                    return refused("--sink takes a path".to_string());
                };
                inputs.sink_path = value.clone();
            }
            "--readout" => inputs.readout = true,
            "--surprisal" => inputs.surprisal = true,
            "--field-depth" => {
                let Some(value) = it.next() else {
                    return refused("--field-depth takes a value".to_string());
                };
                match value.parse() {
                    Ok(depth) => inputs.field_depth = Some(depth),
                    Err(_) => {
                        return refused(format!(
                            "--field-depth is not a depth: {value}"
                        ));
                    }
                }
            }
            "--out" => {
                let Some(value) = it.next().filter(|v| !v.starts_with("--")) else {
                    return refused("--out takes a path".to_string());
                };
                out = Some(value.clone());
            }
            other if trace.is_none() => trace = Some(other.to_string()),
            other => {
                eprintln!(
                    "{}",
                    serde_json::json!({"analysis_refusal": format!("unknown argument {other}")})
                );
                return std::process::ExitCode::FAILURE;
            }
        }
    }
    let (Some(trace), false, false) = (trace, inputs.devices.is_empty(), inputs.sink_path.is_empty())
    else {
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "derive takes <trace>, --devices, and --sink"})
        );
        return std::process::ExitCode::FAILURE;
    };
    let Ok(text) = read_stream(&trace) else {
        eprintln!("{}", serde_json::json!({"analysis_refusal": "the trace does not read"}));
        return std::process::ExitCode::FAILURE;
    };
    match weaver_analysis::derive(&parse_record(&text), &inputs) {
        Ok(declaration) => match out {
            Some(path) => match std::fs::write(&path, declaration) {
                Ok(()) => std::process::ExitCode::SUCCESS,
                Err(_) => {
                    eprintln!(
                        "{}",
                        serde_json::json!({"analysis_refusal": "the declaration does not write"})
                    );
                    std::process::ExitCode::FAILURE
                }
            },
            None => {
                print!("{declaration}");
                std::process::ExitCode::SUCCESS
            }
        },
        Err(refusal) => {
            eprintln!("{}", serde_json::json!({"derive_refusal": format!("{refusal:?}")}));
            std::process::ExitCode::FAILURE
        }
    }
}

fn run_preload(trace: &str, socket: &str) -> std::process::ExitCode {
    let Ok(text) = read_stream(trace) else {
        eprintln!("{}", serde_json::json!({"analysis_refusal": "the trace does not read"}));
        return std::process::ExitCode::FAILURE;
    };
    let events = parse_record(&text);
    let Some(session) = events.first().map(|e| e.envelope.session.clone()) else {
        eprintln!("{}", serde_json::json!({"analysis_refusal": "the record holds no event"}));
        return std::process::ExitCode::FAILURE;
    };
    let distillates = weaver_analysis::project(&events);
    let outcome = std::os::unix::net::UnixStream::connect(socket)
        .and_then(|stream| {
            let mut sender = weaver_analysis::preload::open(stream, &session)?;
            for distillate in &distillates {
                sender.send(distillate)?;
            }
            sender.seal()
        });
    match outcome {
        Ok(()) => {
            println!(
                "{}",
                serde_json::json!({
                    "preloaded": distillates.len(),
                    "session": session,
                    "sealed": true,
                })
            );
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"analysis_refusal": format!("the preload died: {error}")})
            );
            std::process::ExitCode::FAILURE
        }
    }
}

fn run_read(trace: &str) -> std::process::ExitCode {
    let Ok(text) = read_stream(trace) else {
        eprintln!("{}", serde_json::json!({"analysis_refusal": "the record does not read"}));
        return std::process::ExitCode::FAILURE;
    };
    let events = parse_record(&text);
    match weaver_analysis::gate(&events) {
        Gated::Produces { passes } => {
            for pass in &passes {
                println!(
                    "{}",
                    serde_json::json!({
                        "run": pass.run,
                        "reader_elected": pass.reader_elected,
                        "outcome": match &pass.outcome {
                            Some(weaver_analysis::Outcome::Certified) => "certified".to_string(),
                            Some(weaver_analysis::Outcome::Diverged { detail }) =>
                                format!("diverged {detail}"),
                            Some(weaver_analysis::Outcome::Abandoned { detail }) =>
                                format!("abandoned {detail}"),
                            // Unreachable by the gate's own filter, kept
                            // total for the enum.
                            None => "not ended".to_string(),
                        },
                    })
                );
            }
            std::process::ExitCode::SUCCESS
        }
        Gated::Nothing { why } => {
            eprintln!("{}", serde_json::json!({"nothing_produced": why}));
            std::process::ExitCode::FAILURE
        }
    }
}
