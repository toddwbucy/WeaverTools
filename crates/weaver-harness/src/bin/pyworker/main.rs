//! The Python-iterating worker, a builder's composition root per
//! `weaver-harness-Spec` section 6 and issue #134. Framework identical to
//! the demonstration worker - same arguments, same loop 0, same crossing -
//! with the loop body marshalled into an embedded interpreter by
//! `py_loop`. Which binary the unit starts is the operator's provisioning,
//! so this reaches an agent by the `worker-binary` configuration and
//! nothing else.
//!
//! The loop file's path arrives by the `WEAVER_PY_LOOP` environment
//! variable, which a deployment sets through the standing `unit-properties`
//! configuration (`Environment=WEAVER_PY_LOOP=...`), no admin change
//! involved. Unset, the default below stands. The loop files themselves
//! live in `dev_python`, the developer's directory by the standing prefix.
//!
//! The binary compiles behind the `pyworker` feature, per
//! `weaver-harness-Spec` section 1, so no default build touches the
//! interpreter:
//!
//! ```text
//! cargo build --release -p weaver-harness --features pyworker --bin pyworker
//! ```

mod py_loop;

use std::process::ExitCode;

use weaver_harness::{Harness, OrganBinaries, OrganParameters, bind_coordination};

const DEFAULT_LOOP: &str = "/usr/local/libexec/weaver/loops/dev_loop.py";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(socket), Some(spu), Some(gate)) = (args.next(), args.next(), args.next()) else {
        eprintln!(
            "pyworker <coordination-socket> <spu-binary> <gate-binary> [identity] \
             [--headroom-bytes N]   (loop file: $WEAVER_PY_LOOP)"
        );
        return ExitCode::FAILURE;
    };
    let mut identity = String::new();
    let mut parameters = OrganParameters::default();
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--headroom-bytes" => match args.next() {
                Some(value) => parameters.headroom_bytes = Some(value),
                None => {
                    eprintln!("pyworker: --headroom-bytes takes a value");
                    return ExitCode::FAILURE;
                }
            },
            other if other.starts_with("--") => {
                eprintln!("pyworker: unknown parameter {other}");
                return ExitCode::FAILURE;
            }
            positional => identity = positional.to_string(),
        }
    }
    let loop_path = std::path::PathBuf::from(
        std::env::var("WEAVER_PY_LOOP").unwrap_or_else(|_| DEFAULT_LOOP.to_string()),
    );

    let listener = match bind_coordination(std::path::Path::new(&socket)) {
        Ok(listener) => listener,
        Err(fault) => {
            eprintln!("pyworker: the coordination socket did not bind: {fault:?}");
            return ExitCode::FAILURE;
        }
    };
    let harness = match Harness::listen(
        listener,
        OrganBinaries {
            spu: spu.into(),
            gate: gate.into(),
        },
        parameters,
    ) {
        Ok(harness) => harness,
        Err(fault) => {
            eprintln!("pyworker: the hygiene set refused construction: {fault:?}");
            return ExitCode::FAILURE;
        }
    };

    match harness.serve(&identity, &[], |seat, text| {
        py_loop::drive(&loop_path, seat, text)
    }) {
        Ok(_) => ExitCode::SUCCESS,
        Err(fault) => {
            eprintln!("pyworker: service ended below the exchange layer: {fault:?}");
            ExitCode::FAILURE
        }
    }
}
