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
        "session: s-1\n",
        "spu-instruction:\n",
        "  decoder:\n",
        "    model-binding:\n",
        "      artifact: qwen3-4b-instruct\n",
        "      devices: [0]\n",
        "    residual-readout-election: false\n",
        "    identity:\n",
        "      - role: user\n",
        "        content:\n",
        "          - type: text\n",
        "            text: You answer briefly.\n",
        "    tunable-values: {}\n",
        "tool-set: []\n",
        "permission-mode: ask\n",
        "gate-instruction:\n",
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
    let decoder = &config.spu_instruction.decoder;
    assert_eq!(decoder.model_binding.devices.len(), 1);
    assert_eq!(config.permission_mode, weaver_traits::PermissionMode::Ask);
    assert!(!decoder.residual_readout_election);
    // The identity material parses to its canonical messages: one user
    // message with a single text block, per the full_config fixture.
    assert_eq!(decoder.identity.len(), 1);
    assert_eq!(decoder.identity[0].role, weaver_traits::Role::User);
    assert!(matches!(
        decoder.identity[0].content.as_slice(),
        [weaver_traits::ContentBlock::Text { text }] if text == "You answer briefly."
    ));
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
    let source = full_config().replace("    residual-readout-election: false\n", "");
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
        .replace("    model-binding:\n", "")
        .replace("      artifact: qwen3-4b-instruct\n", "")
        .replace("      devices: [0]\n", "");
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

/// **A tunable value that is not finite refuses the load**, per Spec section 2.
/// The floor judges finiteness because it is true of any value regardless of
/// which parameter carried it, and leaves whether a value suits its parameter
/// to the SPU, which holds the dispositions that say which names are counts.
///
/// Perturbation: drop the `check_tunable_values` call from `parse` and this
/// test fails, `.nan` reaching a sampler through a config that parsed clean.
#[test]
fn a_non_finite_tunable_value_refuses() {
    for bad in [".nan", ".inf", "-.inf"] {
        let source = full_config().replace(
            "    tunable-values: {}\n",
            &format!("    tunable-values:\n      temperature: {bad}\n"),
        );
        let err = parse(&source).expect_err("refuses");
        assert_eq!(err.kind, ConfigErrorKind::BadValue, "{bad} must not parse");
        assert_eq!(
            err.field,
            Some(FieldName("tunable-values.temperature".into())),
            "and the refusal names the value it refused"
        );
    }
}

/// A finite value parses and reaches the declaration, so the check above is
/// refusing the bad case rather than the field.
#[test]
fn a_finite_tunable_value_parses() {
    let source = full_config().replace(
        "    tunable-values: {}\n",
        "    tunable-values:\n      temperature: 0.2\n",
    );
    let config = parse(&source).expect("parses");
    assert_eq!(
        config.spu_instruction.decoder.tunable_values.get("temperature"),
        Some(&0.2f64)
    );
}

/// The one optional field: absent means None, which admin resolves to the
/// ruled default, and the standing fixtures above never mention it, which
/// is the absence path exercised by every test in this file.
#[test]
fn an_absent_state_election_is_none() {
    let config = parse(&full_config()).expect("parses");
    assert_eq!(config.state_election, None);
    assert_eq!(
        weaver_types::StateElection::default(),
        weaver_types::StateElection {
            all_kinds: true,
            keys: Vec::new(),
        },
        "the resolved default's spelling is the Spec's"
    );
}

/// A present election parses whole, keys meaningful beside all-kinds true.
#[test]
fn a_present_state_election_parses() {
    let source = format!(
        concat!(
            "{}state-election:\n",
            "  all-kinds: true\n",
            "  keys:\n",
            "    - kind: turn.closed\n",
            "      paths: [close]\n",
            "    - kind: message.user\n",
            "      paths: [content]\n",
        ),
        full_config()
    );
    let config = parse(&source).expect("parses");
    let election = config.state_election.expect("present");
    assert!(election.all_kinds);
    assert_eq!(election.keys.len(), 2);
    assert_eq!(election.keys[0].kind, "turn.closed");
    assert_eq!(election.keys[1].paths, vec!["content".to_string()]);
}

/// Inside a present block the required-field discipline resumes: a block
/// missing a member refuses, and an unknown member refuses.
#[test]
fn a_partial_state_election_refuses() {
    for tail in [
        "state-election:\n  keys: []\n",
        "state-election:\n  all-kinds: true\n",
        "state-election:\n  all-kinds: true\n  keys: []\n  extra: 1\n",
        "state-election:\n  all-kinds: true\n  keys:\n    - kind: load\n",
    ] {
        let source = format!("{}{}", full_config(), tail);
        assert!(parse(&source).is_err(), "{tail} must refuse");
    }
}

/// The classify role is optional by presence, per `weaver-types-Spec`
/// section 2 as of the classifier act: absence parses as the operator
/// declaring no classifier, presence requires the binding whole, and an
/// unknown key inside the section refuses like any other.
#[test]
fn the_classify_role_is_optional_by_presence() {
    let without = parse(&full_config()).expect("parses");
    assert!(without.spu_instruction.classify.is_none());

    let with = full_config().replace(
        "    tunable-values: {}\n",
        concat!(
            "    tunable-values: {}\n",
            "  classify:\n",
            "    model-binding:\n",
            "      artifact: modernbert-base-zeroshot\n",
            "      devices: [0]\n",
        ),
    );
    let config = parse(&with).expect("parses with the role");
    let classify = config.spu_instruction.classify.expect("present");
    assert_eq!(
        classify.model_binding.devices,
        vec![weaver_types::DeviceOrdinal(0)]
    );

    let missing_binding = full_config().replace(
        "    tunable-values: {}\n",
        concat!("    tunable-values: {}\n", "  classify: {}\n"),
    );
    let err = parse(&missing_binding).expect_err("a present section is whole");
    assert!(matches!(err.kind, ConfigErrorKind::MissingField { .. }), "{err:?}");
}
