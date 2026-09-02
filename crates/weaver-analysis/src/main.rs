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
        Some(("compare", [left, right])) => run_compare(left, right),
        Some(("lens", rest)) => run_lens(rest),
        _ => refused(
            "usage: weaver-analysis derive <trace> --devices <n,..> --sink <path> \
             [--readout] [--field-depth <n>] [--surprisal] | preload <trace> <socket> \
             | read <diagnostic-trace> | compare <capture> <capture> \
             | lens <capture> --lens <path> --weights <path> [--layers 2,6,..] \
             [--positions p,..] [--topk 5] [--min-top5 0.9] [--rms-epsilon 1e-6]",
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

/// Two captures differenced, per `weaver-analysis-Spec` section 5: this is
/// certification step 3's own check performed where both records are held,
/// and what it licenses is the discard.
fn run_compare(left: &str, right: &str) -> std::process::ExitCode {
    let (Ok(a), Ok(b)) = (read_stream(left), read_stream(right)) else {
        eprintln!("{}", serde_json::json!({"analysis_refusal": "a record does not read"}));
        return std::process::ExitCode::FAILURE;
    };
    let left = weaver_analysis::Capture::of(&parse_record(&a));
    let right = weaver_analysis::Capture::of(&parse_record(&b));
    match weaver_analysis::compare(&left, &right) {
        weaver_analysis::Comparison::Identical { positions, values } => {
            println!(
                "{}",
                serde_json::json!({
                    "verdict": "identical", "positions": positions, "values": values,
                })
            );
            std::process::ExitCode::SUCCESS
        }
        weaver_analysis::Comparison::Diverged {
            turn,
            position,
            layer,
            left,
            right,
        } => {
            println!(
                "{}",
                serde_json::json!({
                    "verdict": "diverged", "turn": turn, "position": position,
                    "layer": layer, "left": left, "right": right,
                })
            );
            std::process::ExitCode::FAILURE
        }
        weaver_analysis::Comparison::Incomparable { detail } => {
            eprintln!("{}", serde_json::json!({"incomparable": detail}));
            std::process::ExitCode::FAILURE
        }
    }
}

/// A capture read through the lens, per Spec section 5: the control first
/// and the trajectories only above its bar.
fn run_lens(rest: &[String]) -> std::process::ExitCode {
    let refused = |why: String| {
        eprintln!("{}", serde_json::json!({"analysis_refusal": why}));
        std::process::ExitCode::FAILURE
    };
    let mut record = None;
    let mut lens_path = None;
    let mut weights = None;
    let mut layers: Vec<u32> = vec![2, 6, 10, 14, 18, 22];
    let mut positions: Option<Vec<u64>> = None;
    let mut topk = 5usize;
    let mut min_top5 = 0.9f64;
    let mut epsilon = 1e-6f32;
    let mut it = rest.iter();
    while let Some(argument) = it.next() {
        let mut value = |name: &str| -> Option<String> {
            it.next().filter(|v| !v.starts_with("--")).cloned().or_else(|| {
                eprintln!(
                    "{}",
                    serde_json::json!({"analysis_refusal": format!("{name} takes a value")})
                );
                None
            })
        };
        match argument.as_str() {
            "--lens" => match value("--lens") {
                Some(v) => lens_path = Some(v),
                None => return std::process::ExitCode::FAILURE,
            },
            "--weights" => match value("--weights") {
                Some(v) => weights = Some(v),
                None => return std::process::ExitCode::FAILURE,
            },
            "--layers" => match value("--layers") {
                Some(v) => {
                    let mut held = Vec::new();
                    for token in v.split(',') {
                        match token.parse() {
                            Ok(layer) => held.push(layer),
                            Err(_) => return refused(format!("--layers holds {token}")),
                        }
                    }
                    layers = held;
                }
                None => return std::process::ExitCode::FAILURE,
            },
            "--positions" => match value("--positions") {
                Some(v) => {
                    let mut held = Vec::new();
                    for token in v.split(',') {
                        match token.parse() {
                            Ok(position) => held.push(position),
                            Err(_) => return refused(format!("--positions holds {token}")),
                        }
                    }
                    positions = Some(held);
                }
                None => return std::process::ExitCode::FAILURE,
            },
            "--topk" => match value("--topk").map(|v| v.parse()) {
                Some(Ok(k)) => topk = k,
                Some(Err(_)) => return refused("--topk is not a count".to_string()),
                None => return std::process::ExitCode::FAILURE,
            },
            "--min-top5" => match value("--min-top5").map(|v| v.parse()) {
                Some(Ok(rate)) => min_top5 = rate,
                Some(Err(_)) => return refused("--min-top5 is not a rate".to_string()),
                None => return std::process::ExitCode::FAILURE,
            },
            "--rms-epsilon" => match value("--rms-epsilon").map(|v| v.parse()) {
                Some(Ok(e)) => epsilon = e,
                Some(Err(_)) => return refused("--rms-epsilon is not a number".to_string()),
                None => return std::process::ExitCode::FAILURE,
            },
            // The capture is a path, so a token wearing a flag's shape is a
            // mistyped flag rather than a positional: taking it would run
            // the reading against a file named `--lense`.
            other if record.is_none() && !other.starts_with('-') => {
                record = Some(other.to_string())
            }
            other => return refused(format!("unknown argument {other}")),
        }
    }
    let (Some(record), Some(lens_path), Some(weights)) = (record, lens_path, weights) else {
        return refused("lens takes <capture>, --lens, and --weights".to_string());
    };
    let Ok(text) = read_stream(&record) else {
        return refused("the capture does not read".to_string());
    };
    let capture = weaver_analysis::Capture::of(&parse_record(&text));
    let paired = capture.paired();
    if paired.is_empty() {
        return refused(
            "no column pairs with a drawn token: the record holds columns or \
             measurements, not both"
                .to_string(),
        );
    }

    // The identity, judged before anything large is read.
    let lens_file = std::path::Path::new(&lens_path);
    let manifest = match weaver_analysis::read_manifest(lens_file) {
        Ok(manifest) => manifest,
        Err(refusal) => return refused(format!("{refusal:?}")),
    };
    // **The digest reads in bounded chunks**, so identifying a model does
    // not hold its whole file in memory beside the parse that follows.
    let digest = match weaver_analysis::sha256_hex_of_file(std::path::Path::new(&weights)) {
        Ok(digest) => digest,
        Err(error) => return refused(format!("the weights do not read: {error}")),
    };
    if digest != manifest.fitted_for.model_safetensors_sha256 {
        eprintln!(
            "{}",
            serde_json::json!({
                "analysis_refusal": "the weights are not the ones the lens was fitted for",
                "held": digest,
                "manifest": manifest.fitted_for.model_safetensors_sha256,
            })
        );
        return std::process::ExitCode::FAILURE;
    }
    let lens = match weaver_analysis::Lens::open(lens_file, &manifest) {
        Ok(lens) => lens,
        Err(refusal) => return refused(format!("{refusal:?}")),
    };
    let unembedding =
        match weaver_analysis::Unembedding::open(std::path::Path::new(&weights), epsilon) {
            Ok(unembedding) => unembedding,
            Err(refusal) => return refused(format!("{refusal:?}")),
        };
    if unembedding.width() != lens.d_model() {
        return refused(format!(
            "the weights are {} wide and the lens is {}",
            unembedding.width(),
            lens.d_model()
        ));
    }

    // **The control precedes every reading and gates it**: the final
    // layer, no transport, against the token each position drew.
    let mut ranks = Vec::with_capacity(paired.len());
    for key in &paired {
        let column = &capture.columns[key];
        let Some(final_layer) = column.last() else {
            return refused("a column holds no layers".to_string());
        };
        let Some(logits) = unembedding.logits(final_layer) else {
            return refused("a column is not the model's width".to_string());
        };
        let Some(rank) =
            weaver_analysis::Unembedding::rank_of(&logits, capture.drawn[key] as usize)
        else {
            return refused("a drawn token is outside the vocabulary".to_string());
        };
        ranks.push(rank);
    }
    let top1 = ranks.iter().filter(|r| **r == 0).count();
    let top5 = ranks.iter().filter(|r| **r < 5).count();
    let top5_rate = top5 as f64 / ranks.len() as f64;
    println!(
        "{}",
        serde_json::json!({
            "control": "unembed(h_final) vs the drawn token",
            "positions": ranks.len(),
            "top1": top1,
            "top1_rate": (top1 as f64 / ranks.len() as f64 * 1e4).round() / 1e4,
            "top5": top5,
            "top5_rate": (top5_rate * 1e4).round() / 1e4,
        })
    );
    if top5_rate < min_top5 {
        eprintln!(
            "{}",
            serde_json::json!({
                "refusal": "the control is below the bar",
                "top5_rate": (top5_rate * 1e4).round() / 1e4,
                "min_top5": min_top5,
            })
        );
        return std::process::ExitCode::FAILURE;
    }

    let chosen: Vec<&(Option<String>, u64)> = match &positions {
        Some(wanted) => paired.iter().filter(|(_, p)| wanted.contains(p)).collect(),
        None => paired.iter().step_by((paired.len() / 6).max(1)).take(6).collect(),
    };
    // **A reading of nothing is a refusal, not a success.** A position the
    // capture does not hold names no column, and exiting zero over an empty
    // selection would report silence as a reading.
    if chosen.is_empty() {
        return refused(format!(
            "no chosen position is in the capture: {:?}",
            positions.unwrap_or_default()
        ));
    }
    for key in chosen {
        let column = &capture.columns[key];
        let mut trajectory = serde_json::Map::new();
        // **A layer that answers nothing is named**, so an invalid layer
        // request is distinguishable from a layer the lens declines: a
        // silent skip would read as a lens that had nothing to say there.
        let mut skipped: Vec<u32> = Vec::new();
        for layer in &layers {
            let read = column
                .get(*layer as usize)
                .and_then(|residual| lens.transport(*layer, residual))
                .and_then(|transported| unembedding.logits(&transported));
            match read {
                Some(logits) => {
                    trajectory.insert(
                        layer.to_string(),
                        serde_json::json!(weaver_analysis::Unembedding::top_k(&logits, topk)),
                    );
                }
                None => skipped.push(*layer),
            }
        }
        if trajectory.is_empty() {
            return refused(format!(
                "no requested layer reads at position {}: the capture holds {} \
                 layers and the lens fits {:?}",
                key.1,
                column.len(),
                lens.source_layers()
            ));
        }
        let mut line = serde_json::json!({
            "turn": key.0,
            "position": key.1,
            "drawn": capture.drawn[key],
            "trajectory": trajectory,
        });
        if !skipped.is_empty() {
            line["skipped_layers"] = serde_json::json!(skipped);
        }
        println!("{line}");
    }
    std::process::ExitCode::SUCCESS
}
