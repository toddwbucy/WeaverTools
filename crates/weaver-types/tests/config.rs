//! conforms: types-required-field-refuses
//! conforms: types-unknown-key-refuses
//! conforms: types-config-names-kebab
//!
//! The config-parse tests of `weaver-types-Spec` sections 2 and 5, running only
//! with the `config` feature, which is the parser's whole surface.
#![cfg(feature = "config")]

use weaver_types::{ConfigErrorKind, FieldName, parse};

fn full_config() -> String {
    concat!(
        "model-binding:\n",
        "  artifact: qwen3-4b-instruct\n",
        "  devices: [0]\n",
        "tool-set: []\n",
        "permission-mode: ask\n",
        "residual-readout-election: false\n",
        "gate-instruction:\n",
        "  socket-path: /run/weaver/alpha/gate.sock\n",
        "  access-rule:\n",
        "    allowed-uids: [1000]\n",
        "    allowed-gids: []\n",
        "    denied-uids: [1701]\n",
        "trace-sink:\n",
        "  kind: file\n",
        "  path: /var/lib/weaver/alpha/trace.ndjson\n",
        "  create: true\n",
    )
    .to_string()
}

/// The operator's kebab-case document parses into the typed surface.
#[test]
fn a_complete_config_parses() {
    let config = parse(&full_config()).expect("parses");
    assert_eq!(config.model_binding.devices.len(), 1);
    assert_eq!(config.permission_mode, weaver_traits::PermissionMode::Ask);
    assert!(!config.residual_readout_election);
}

/// A missing required field refuses the parse, run separately for the
/// residual-readout election, since it is the one a builder is most likely to
/// make optional.
///
/// Perturbation: make the field `Option` with a default and this parse
/// succeeds - an operator who stated no readout has silently declined it.
/// Watched to fail under exactly that change.
#[test]
fn missing_residual_readout_election_refuses() {
    let source = full_config().replace("residual-readout-election: false\n", "");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::MissingField);
    assert_eq!(
        err.field,
        Some(FieldName("residual-readout-election".into()))
    );
}

/// Every field of the declared surface is required, not only the elected one.
#[test]
fn missing_model_binding_refuses() {
    let source = full_config()
        .replace("model-binding:\n", "")
        .replace("  artifact: qwen3-4b-instruct\n", "")
        .replace("  devices: [0]\n", "");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::MissingField);
}

/// A field no organ registered refuses rather than being ignored: a mistyped
/// `permission-mode` must not vanish.
///
/// Perturbation: remove `deny_unknown_fields` from `AgentConfig` and the
/// mistyped key parses as unknown and is discarded, while the real field goes
/// missing - the exact failure the rejection exists to prevent. Watched to
/// fail under exactly that removal.
#[test]
fn unknown_key_refuses() {
    let source = full_config().replace("permission-mode:", "permission-modes:");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(
        err.kind,
        ConfigErrorKind::UnknownField,
        "the mistyped key itself must be the refusal, not the hole it left"
    );
    assert_eq!(err.field, Some(FieldName("permission-modes".into())));
}

/// An empty device set is a parse error rather than a default: a binding
/// assigning no device is a declaration the operator did not finish.
#[test]
fn empty_device_set_refuses() {
    let source = full_config().replace("devices: [0]", "devices: []");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::BadValue);
    assert_eq!(err.field, Some(FieldName("model-binding".into())));
}

/// A value outside an elected vocabulary refuses: `permission-mode` elects from
/// `weaver-traits` and a mode that crate does not define is a bad value.
#[test]
fn unknown_permission_mode_refuses() {
    let source = full_config().replace("permission-mode: ask", "permission-mode: maybe");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::BadValue);
}

/// An unknown key inside the sink refuses, per variant, because the derive's
/// `deny_unknown_fields` does not compose with an internally tagged enum and
/// the surface check in `parse` is the mechanism instead.
///
/// Perturbation: remove the `check_trace_sink_surface` call from `parse` and
/// all three parses succeed with the stray key silently discarded. Watched to
/// fail under exactly that removal.
#[test]
fn unknown_key_inside_file_sink_refuses() {
    let source = full_config().replace("  create: true\n", "  create: true\n  mode: append\n");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::UnknownField);
    assert_eq!(err.field, Some(FieldName("trace-sink.mode".into())));
}

#[test]
fn unknown_key_inside_pipe_sink_refuses() {
    let source = full_config()
        .replace("  kind: file\n", "  kind: pipe\n")
        .replace("  create: true\n", "  create: true\n  buffered: true\n");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::UnknownField);
    assert_eq!(err.field, Some(FieldName("trace-sink.buffered".into())));
}

/// The socket surface is two keys, so `create` itself is the stray here: a
/// creation flag on a socket sink would promise an act admin cannot perform.
#[test]
fn unknown_key_inside_socket_sink_refuses() {
    let source = full_config()
        .replace("  kind: file\n", "  kind: socket\n")
        .replace("  create: true\n", "  linger: false\n");
    let err = parse(&source).expect_err("refuses");
    assert_eq!(err.kind, ConfigErrorKind::UnknownField);
    assert_eq!(err.field, Some(FieldName("trace-sink.linger".into())));
}
