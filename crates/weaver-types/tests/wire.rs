//! conforms: types-loop0-encoding-json
//! conforms: types-tagging-test
//!
//! The wire-shape tests of `weaver-types-Spec` section 4.3. The Spec states the
//! envelope's layout rather than leaving it to a reader, and these tests hold
//! the code to the stated octets, including the two failure shapes an earlier
//! draft of the Spec verified against serde 1.x: duplicate tag names and
//! newtype variants that cannot serialize.

use weaver_types::{
    AgentName, AgentState, AgentSummary, ExchangeId, LifecycleAnswer, LifecycleDirective,
    LifecycleRefusal, Opener, OrganEnvelope, Payload, Position, RefusingOrgan,
};

/// The envelope's layout, exactly as the Spec states it.
#[test]
fn envelope_layout_is_the_stated_shape() {
    let envelope = OrganEnvelope {
        exchange: ExchangeId {
            opener: Opener::Admin,
            ordinal: 7,
        },
        position: Position::Open,
        payload: Payload::Directive(LifecycleDirective::Load {
            agent: AgentName("alpha".to_string()),
        }),
    };
    let json = serde_json::to_string(&envelope).expect("serializes");
    assert_eq!(
        json,
        concat!(
            "{\"exchange\":{\"opener\":\"admin\",\"ordinal\":7},",
            "\"position\":\"open\",",
            "\"payload\":{\"kind\":\"directive\",\"body\":{\"kind\":\"load\",\"agent\":\"alpha\"}}}"
        )
    );
    let back: OrganEnvelope = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(envelope, back);
}

/// The sequence-carrying case round-trips, which is what the adjacent tagging
/// of `Payload` exists to make possible.
#[test]
fn sequence_carrying_answer_round_trips() {
    let envelope = OrganEnvelope {
        exchange: ExchangeId {
            opener: Opener::Admin,
            ordinal: 9,
        },
        position: Position::Close,
        payload: Payload::Answer(LifecycleAnswer::Agents {
            agents: vec![
                AgentSummary {
                    name: AgentName("alpha".into()),
                    state: AgentState::Idle,
                },
                AgentSummary {
                    name: AgentName("beta".into()),
                    state: AgentState::Unloaded,
                },
            ],
        }),
    };
    let json = serde_json::to_string(&envelope).expect("serializes");
    let back: OrganEnvelope = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(envelope, back);
}

/// The boxed aggregate case round-trips: a refusing organ's reason reaches
/// admin without translation.
#[test]
fn organ_refused_carries_the_inner_reason_unchanged() {
    let refusal = LifecycleRefusal::OrganRefused {
        organ: RefusingOrgan::Spu,
        reason: Box::new(LifecycleRefusal::DeviceCannotAdmit),
    };
    let json = serde_json::to_string(&refusal).expect("serializes");
    assert_eq!(
        json,
        "{\"kind\":\"organ_refused\",\"organ\":\"spu\",\"reason\":{\"kind\":\"device_cannot_admit\"}}"
    );
    let back: LifecycleRefusal = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(refusal, back);
}

/// An unknown payload kind refuses rather than defaulting, the same refusal
/// posture the message model holds.
#[test]
fn unknown_payload_kind_refuses() {
    let json = "{\"kind\":\"telemetry\",\"body\":{}}";
    assert!(serde_json::from_str::<Payload>(json).is_err());
}

/// **The measurement splices and never quotes**, per `weaver-types-Spec`
/// section 4.4: the payload of `Generated` carries pre-serialized JSON as a
/// member of the answer's object, and a quoted string in its place is the
/// double encoding the `RawValue` election exists to prevent.
///
/// The round trip is through bytes, because the read side is where a tagging
/// that buffers fails: a shape that serializes clean and cannot come back is
/// a wire type only one party can use.
#[test]
fn the_measurement_splices_and_never_quotes() {
    let measurement = serde_json::value::RawValue::from_string(
        r#"{"tokens":[151643,872],"timings":{"prefill_ms":12}}"#.to_string(),
    )
    .expect("valid JSON splices");
    let request = serde_json::value::RawValue::from_string(
        serde_json::json!({"rendered":"<|im_start|>user\nhi<|im_end|>\n","template":"qwen2","sampling":{}}).to_string(),
    )
    .expect("valid JSON splices");
    let answer = weaver_types::TokenAnswer::Generated(weaver_types::Generation {
        emission: "two words".to_string(),
        finish: weaver_types::Finish::Completed,
        request,
        measurement,
    });

    let bytes = serde_json::to_string(&answer).expect("serializes");
    let encoded: serde_json::Value = serde_json::from_str(&bytes).expect("valid JSON");
    assert_eq!(
        encoded["kind"], "generated",
        "adjacently tagged under the spliced-member arm, not externally: {bytes}"
    );
    assert!(
        encoded["body"]["measurement"].is_object(),
        "the measurement nests in the adjacent body as an object: {bytes}"
    );
    assert!(
        bytes.contains(r#""measurement":{"tokens":[151643,872]"#),
        "the measurement is a member, spliced: {bytes}"
    );
    assert!(
        !bytes.contains(r#""measurement":"{"#),
        "and never a quoted string: {bytes}"
    );

    let back: weaver_types::TokenAnswer = serde_json::from_str(&bytes).expect("deserializes");
    assert_eq!(back, answer, "and the round trip holds through bytes");
}

/// The directive's ask round-trips with its tunable map, the map carrying
/// `f64` because the wire's numbers are, per Spec section 4.4.
#[test]
fn the_token_directive_round_trips_through_bytes() {
    let directive = weaver_types::TokenDirective::AppendAndGenerate {
        turn: weaver_types::TurnKey("t-7".into()),
        delta: vec![],
        tunable: [("temperature".to_string(), 0.7f64)].into_iter().collect(),
    };
    let bytes = serde_json::to_string(&directive).expect("serializes");
    assert!(
        bytes.contains(r#""kind":"append_and_generate""#),
        "internally tagged, snake case: {bytes}"
    );
    let back: weaver_types::TokenDirective = serde_json::from_str(&bytes).expect("deserializes");
    assert_eq!(back, directive);
}

/// The refusal's overflow carries the session's own account, per the decode
/// contract's section 5, and the fieldless cases are plain tagged strings.
#[test]
fn the_token_refusal_carries_the_account() {
    let refusal = weaver_types::TokenRefusal::Overflow {
        resident: 4096,
        requested: 512,
        capacity: 4352,
    };
    let bytes = serde_json::to_string(&refusal).expect("serializes");
    assert!(bytes.contains(r#""kind":"overflow""#) && bytes.contains(r#""resident":4096"#));
    let back: weaver_types::TokenRefusal = serde_json::from_str(&bytes).expect("deserializes");
    assert_eq!(back, refusal);
    let not_open: weaver_types::TokenRefusal =
        serde_json::from_str(r#"{"kind":"not_open"}"#).expect("a fieldless case reads");
    assert_eq!(not_open, weaver_types::TokenRefusal::NotOpen);
}
