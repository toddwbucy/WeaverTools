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
        Some(("signals", [record])) => run_signals(record, 2.0),
        Some(("signals", [record, k])) => match k.parse::<f32>() {
            // A bar that parses is not yet a bar: an infinity clears
            // nothing and a NaN compares false against everything, so
            // either would answer "no spikes" about a series that has
            // them.
            Ok(k) if k.is_finite() => run_signals(record, k),
            _ => {
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "analysis_refusal": format!("the spike bar is not a finite number: {k}")
                    })
                );
                std::process::ExitCode::FAILURE
            }
        },
        Some(("lens", rest)) => run_lens(rest),
        Some(("field", rest)) => run_field(rest),
        _ => refused(
            "usage: weaver-analysis derive <trace> --devices <n,..> --sink <path> \
             [--sink-kind file|pipe] [--readout] [--field-depth <n>] [--surprisal] | preload <trace> <socket> \
             | read <diagnostic-trace> | compare <capture> <capture> \
             | signals <record> [spike-bar] \
             | lens <capture> --lens <path> --weights <path> [--layers 2,6,..] \
             [--positions p,.. (a file defaults to a spread of eight)] [--topk 5] \
             [--min-top5 0.9] [--rms-epsilon 1e-6] \
             | field <record> --position <turn>:<position> [--run <run>]",
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
        sink_kind: weaver_analysis::declare::SinkKind::File,
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
            "--sink-kind" => {
                let Some(value) = it.next().filter(|v| !v.starts_with("--")) else {
                    return refused("--sink-kind takes file or pipe".to_string());
                };
                match weaver_analysis::declare::SinkKind::parse(value) {
                    Some(kind) => inputs.sink_kind = kind,
                    None => {
                        return refused(format!("--sink-kind is file or pipe, not {value}"));
                    }
                }
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
                        return refused(format!("--field-depth is not a depth: {value}"));
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
    let (Some(trace), false, false) = (
        trace,
        inputs.devices.is_empty(),
        inputs.sink_path.is_empty(),
    ) else {
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "derive takes <trace>, --devices, and --sink"})
        );
        return std::process::ExitCode::FAILURE;
    };
    let Ok(text) = read_stream(&trace) else {
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "the trace does not read"})
        );
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
            eprintln!(
                "{}",
                serde_json::json!({"derive_refusal": format!("{refusal:?}")})
            );
            std::process::ExitCode::FAILURE
        }
    }
}

fn run_preload(trace: &str, socket: &str) -> std::process::ExitCode {
    let Ok(text) = read_stream(trace) else {
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "the trace does not read"})
        );
        return std::process::ExitCode::FAILURE;
    };
    let events = parse_record(&text);
    let Some(session) = events.first().map(|e| e.envelope.session.clone()) else {
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "the record holds no event"})
        );
        return std::process::ExitCode::FAILURE;
    };
    let distillates = weaver_analysis::project(&events);
    let outcome = std::os::unix::net::UnixStream::connect(socket).and_then(|stream| {
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
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "the record does not read"})
        );
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
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "a record does not read"})
        );
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
            it.next()
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .or_else(|| {
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
            "--rms-epsilon" => match value("--rms-epsilon") {
                Some(v) => match weaver_analysis::rms_epsilon(&v) {
                    Some(e) => epsilon = e,
                    None => {
                        return refused(format!(
                            "--rms-epsilon is not a finite positive number: {v}"
                        ));
                    }
                },
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
    // **On a stream the analyst names the positions**, per Spec section 5:
    // a spread over the whole record cannot be chosen without the whole
    // record, and a reader that buffered to find one would keep what a
    // pipe exists not to keep.
    let streaming = record == "-" || is_fifo(&record);
    if streaming && positions.is_none() {
        return refused(
            "a streamed record needs --positions: the default spread wants a whole \
             record, and holding one would keep what the pipe exists not to keep"
                .to_string(),
        );
    }
    // **The file's default spread is eight positions**, per Spec section 5
    // as of 2026-09-04, taken by a first read that learns the record's
    // positions and keeps nothing else, because a file can be read twice
    // and a pipe cannot. The analyst's own list replaces it.
    let mut first_read_identity: Option<(u64, std::time::SystemTime)> = None;
    if !streaming && positions.is_none() {
        first_read_identity = file_identity(&record);
        let source = match weaver_analysis::stream::open(&record) {
            Ok(source) => source,
            Err(error) => return refused(format!("the record does not open: {error}")),
        };
        let mut held = weaver_analysis::capture::Positions::default();
        if let weaver_analysis::Drained::Refused(why) = weaver_analysis::drain(source, &mut held) {
            return refused(why);
        }
        positions = Some(weaver_analysis::capture::spread(&held.held, DEFAULT_SPREAD));
    }

    // The identity, judged before anything large is read.
    let lens_file = std::path::Path::new(&lens_path);
    let manifest = match weaver_analysis::read_manifest(lens_file) {
        Ok(manifest) => manifest,
        Err(refusal) => return refused(format!("{refusal:?}")),
    };
    // **The identity is judged against the files the read will open**, each
    // digest recomputed in bounded chunks and never trusted from a name: one
    // file for a model kept whole, each index-named shard for a sharded one.
    let verified = match weaver_analysis::verify_weights(std::path::Path::new(&weights), &manifest)
    {
        Ok(verified) => verified,
        Err(weaver_analysis::LensRefusal::WeightsDisagree {
            file,
            held,
            manifest,
        }) => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "analysis_refusal": "the weights are not the ones the lens was fitted for",
                    "file": file,
                    "held": held,
                    "manifest": manifest,
                })
            );
            return std::process::ExitCode::FAILURE;
        }
        Err(refusal) => return refused(format!("{refusal:?}")),
    };
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

    // **The reading is taken as the stream drains**, per Spec section 5:
    // the control's rank is computed at the moment a position pairs with
    // the token it drew, and the column is dropped there. What is held is
    // one turn's final layers and the named positions.
    // **The second read reads the file the first read chose over.** A
    // file that moved between them would have its spread taken over one
    // record and its reading over another, so the two reads are held to
    // one length and one modification time and a change refuses.
    if first_read_identity.is_some() && file_identity(&record) != first_read_identity {
        return refused(
            "the record changed between the read that chose the spread and the read".to_string(),
        );
    }
    let source = match weaver_analysis::stream::open(&record) {
        Ok(source) => source,
        Err(error) => return refused(format!("the record does not open: {error}")),
    };
    let mut ranks: Vec<usize> = Vec::new();
    let mut faults: Vec<String> = Vec::new();
    let mut drawn: std::collections::BTreeMap<weaver_analysis::capture::Key, u32> =
        std::collections::BTreeMap::new();
    let outcome;
    let kept;
    {
        let mut pair = |key: &weaver_analysis::capture::Key, column: &[f32], token: u32| {
            drawn.insert(key.clone(), token);
            match unembedding.logits(column) {
                Some(logits) => {
                    match weaver_analysis::Unembedding::rank_of(&logits, token as usize) {
                        Some(rank) => ranks.push(rank),
                        None => faults.push(format!("token {token} is outside the vocabulary")),
                    }
                }
                None => faults.push("a column is not the model's width".to_string()),
            }
        };
        let mut reader = weaver_analysis::capture::Streaming::new(
            positions.clone().unwrap_or_default(),
            &mut pair,
        );
        match weaver_analysis::drain(source, &mut reader) {
            weaver_analysis::Drained::Refused(why) => return refused(why),
            _ => {}
        }
        outcome = reader.outcome.clone();
        kept = std::mem::take(&mut reader.kept);
    }
    if let Some(fault) = faults.first() {
        return refused(fault.clone());
    }
    if ranks.is_empty() {
        return refused(
            "no column pairs with a drawn token: the record holds columns or \
             measurements, not both"
                .to_string(),
        );
    }
    // **A reading is produced only over a certified close**, per the
    // charter: a readout from an uncertified replay is a picture of an
    // unknown run, however it was read.
    match outcome.as_deref() {
        Some("certified") => {}
        Some(other) => {
            return refused(format!(
                "the record's bracket closed {other}, not certified"
            ));
        }
        None => {
            return refused(
                "the record's bracket did not close: nothing is produced for an \
                 unclosed replay"
                    .to_string(),
            );
        }
    }
    let top1 = ranks.iter().filter(|r| **r == 0).count();
    let top5 = ranks.iter().filter(|r| **r < 5).count();
    let top5_rate = top5 as f64 / ranks.len() as f64;
    println!(
        "{}",
        serde_json::json!({
            "control": "unembed(h_final) vs the drawn token",
            "weights": verified.iter().cloned().collect::<std::collections::BTreeMap<_, _>>(),
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

    let chosen: Vec<&weaver_analysis::capture::Key> = kept.keys().collect();
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
        let column = &kept[key];
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
            "drawn": drawn.get(key),
            "trajectory": trajectory,
        });
        if !skipped.is_empty() {
            line["skipped_layers"] = serde_json::json!(skipped);
        }
        println!("{line}");
    }
    std::process::ExitCode::SUCCESS
}

/// The per-position signal series, drained from any record - serving or
/// diagnostic, file or pipe - per `weaver-analysis-Spec` section 5's
/// class. Needs no lens, no weights, and no tap: the entropies ride every
/// generation and the surprisals ride their election.
fn run_signals(record: &str, spike_bar: f32) -> std::process::ExitCode {
    let source = match weaver_analysis::stream::open(record) {
        Ok(source) => source,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"analysis_refusal": format!("the record does not open: {error}")})
            );
            return std::process::ExitCode::FAILURE;
        }
    };
    let mut reader = weaver_analysis::Signals::default();
    if let weaver_analysis::Drained::Refused(why) = weaver_analysis::drain(source, &mut reader) {
        eprintln!("{}", serde_json::json!({"analysis_refusal": why}));
        return std::process::ExitCode::FAILURE;
    }
    // **A diagnostic record's series is gated on its own bracket**, per
    // `weaver-diagnostic-PRD` section 4: a series read from an
    // uncertified replay is a picture of an unknown run exactly as a
    // readout is. A serving record carries no bracket and no gate is
    // owed - it is a record of what happened rather than a claim that
    // something was reproduced.
    if !reader.licensed() {
        eprintln!(
            "{}",
            serde_json::json!({
                "analysis_refusal": match reader.outcome.as_deref() {
                    Some(other) => format!("the record's bracket closed {other}, not certified"),
                    None => "the record's bracket did not close".to_string(),
                }
            })
        );
        return std::process::ExitCode::FAILURE;
    }
    let series = &reader.series;
    if series.points.is_empty() {
        eprintln!(
            "{}",
            serde_json::json!({"analysis_refusal": "the record holds no measured generation"})
        );
        return std::process::ExitCode::FAILURE;
    }
    let with_entropy = series.points.iter().filter(|p| p.entropy.is_some()).count();
    let with_surprisal = series
        .points
        .iter()
        .filter(|p| p.surprisal.is_some())
        .count();
    println!(
        "{}",
        serde_json::json!({
            "positions": series.points.len(),
            "with_entropy": with_entropy,
            "with_surprisal": with_surprisal,
            "perplexities": series.perplexities.iter()
                .map(|(turn, value)| serde_json::json!({"turn": turn, "perplexity": value}))
                .collect::<Vec<_>>(),
        })
    );
    for point in &series.points {
        println!(
            "{}",
            serde_json::json!({
                "turn": point.turn,
                "ordinal": point.ordinal,
                "token": point.token,
                "entropy": point.entropy,
                "surprisal": point.surprisal,
            })
        );
    }
    // **The spikes are named rather than left to the reader's eye**: the
    // bar is the caller's, stated in deviations, and what clears it is
    // where a series turned.
    let spikes = series.spikes(spike_bar);
    eprintln!(
        "{}",
        serde_json::json!({
            "spike_bar_deviations": spike_bar,
            "spikes": spikes.iter().map(|p| serde_json::json!({
                "turn": p.turn, "ordinal": p.ordinal, "token": p.token,
                "surprisal": p.surprisal,
            })).collect::<Vec<_>>(),
        })
    );
    std::process::ExitCode::SUCCESS
}

/// A position's field, per `weaver-analysis-Spec` section 5 as of
/// 2026-09-04: the one `model.field` event at the asked turn and position,
/// drained from any record - serving or diagnostic, file or pipe - and
/// spliced as the record spelled it. Gated on no certified close, because
/// the field is the record's own fact about a position rather than a
/// reading taken over a replay.
fn run_field(rest: &[String]) -> std::process::ExitCode {
    let refused = |why: String| {
        eprintln!("{}", serde_json::json!({"analysis_refusal": why}));
        std::process::ExitCode::FAILURE
    };
    let mut record = None;
    let mut position = None;
    let mut run = None;
    let mut it = rest.iter();
    while let Some(argument) = it.next() {
        match argument.as_str() {
            "--position" => match it.next().filter(|v| !v.starts_with("--")) {
                Some(v) => position = Some(v.clone()),
                None => return refused("--position takes <turn>:<position>".to_string()),
            },
            "--run" => match it.next().filter(|v| !v.starts_with("--")) {
                Some(v) => run = Some(v.clone()),
                None => return refused("--run takes a run".to_string()),
            },
            other if record.is_none() && !other.starts_with('-') => {
                record = Some(other.to_string())
            }
            other => return refused(format!("unknown argument {other}")),
        }
    }
    let (Some(record), Some(position)) = (record, position) else {
        return refused("field takes <record> and --position <turn>:<position>".to_string());
    };
    let address = match weaver_analysis::Address::parse(&position) {
        Ok(address) => address,
        Err(why) => return refused(why),
    };
    let source = match weaver_analysis::stream::open(&record) {
        Ok(source) => source,
        Err(error) => return refused(format!("the record does not open: {error}")),
    };
    // **Each answer leaves as it completes**, so what the read holds is
    // never more than the one position it was asked for.
    let mut emit = |answer: &weaver_analysis::Answer| {
        println!(
            "{}",
            serde_json::to_string(answer).expect("an answer renders")
        );
    };
    let mut reader = weaver_analysis::FieldReader::new(address.clone(), run.clone(), &mut emit);
    match weaver_analysis::drain(source, &mut reader) {
        weaver_analysis::Drained::Refused(why) => return refused(why),
        weaver_analysis::Drained::Exhausted => reader.finish(),
        weaver_analysis::Drained::Stopped => {}
    }
    // **A missing election and a missing position are two refusals**, told
    // apart by whether any field landed at all: the record says which.
    if !reader.seen_field {
        return refused(match reader.elected_depth {
            Some(depth) => format!(
                "the record elected the field at depth {depth} and holds no model.field event"
            ),
            None if reader.diagnostic() => {
                "the record holds no model.field event: the field was not elected for the \
                 replay"
                    .to_string()
            }
            None => "the record holds no model.field event: the field was not elected at its \
                     load"
                .to_string(),
        });
    }
    if reader.answered == 0 {
        return refused(match run {
            Some(run) => format!(
                "the record holds no field at {}:{} in run {run}",
                address.turn, address.position
            ),
            None => format!(
                "the record holds no field at {}:{}",
                address.turn, address.position
            ),
        });
    }
    std::process::ExitCode::SUCCESS
}

/// The file's default spread, per Spec section 5 as of 2026-09-04.
const DEFAULT_SPREAD: usize = 8;

/// A file's length and modification time, the identity the two reads of
/// the default spread are held to, or nothing where the path does not stat.
fn file_identity(path: &str) -> Option<(u64, std::time::SystemTime)> {
    let meta = std::fs::metadata(path).ok()?;
    Some((meta.len(), meta.modified().ok()?))
}

/// Whether a path names a FIFO, which is how a reading learns it is
/// draining rather than reading a record that stands still.
fn is_fifo(path: &str) -> bool {
    use std::os::unix::fs::FileTypeExt;
    std::fs::metadata(path)
        .map(|m| m.file_type().is_fifo())
        .unwrap_or(false)
}
