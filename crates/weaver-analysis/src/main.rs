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
    let mut it = rest.iter();
    while let Some(argument) = it.next() {
        match argument.as_str() {
            "--devices" => {
                inputs.devices = it
                    .next()
                    .map(|v| v.split(',').filter_map(|d| d.parse().ok()).collect())
                    .unwrap_or_default();
            }
            "--sink" => inputs.sink_path = it.next().cloned().unwrap_or_default(),
            "--readout" => inputs.readout = true,
            "--surprisal" => inputs.surprisal = true,
            "--field-depth" => inputs.field_depth = it.next().and_then(|v| v.parse().ok()),
            "--out" => out = it.next().cloned(),
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
