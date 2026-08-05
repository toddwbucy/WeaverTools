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
