//! conforms: state-store-is-a-port
//!
//! The engines behind the port, one module each behind its feature, per
//! `weaver-state-Spec` sections 1 and 3.

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(test)]
fn raw_objects_survive_the_engine_and_answers(store: &mut dyn crate::store::Store) {
    use crate::store::{
        Distillate, render_identity_answer, render_recall_answer, render_replay_answer,
    };
    use serde_json::value::RawValue;
    use std::collections::BTreeMap;

    const RAW: &str = r#"{"z": 1.00, "a": {"second":2,"first":1}}"#;
    store
        .land(&Distillate {
            session: "raw-session".into(),
            run: "raw-run".into(),
            turn: None,
            kind: "message.system".into(),
            sequence: 1,
            pairs: vec![("content".into(), RAW.into())],
        })
        .expect("lands raw object");
    for (ask, field, events, render) in [
        (
            "replay",
            "events",
            store.replay("raw-session").unwrap(),
            render_replay_answer as fn(&[crate::store::RecalledEvent]) -> String,
        ),
        (
            "recall",
            "events",
            store.recall("raw-session", None).unwrap(),
            render_recall_answer,
        ),
        (
            "identity",
            "messages",
            store.identity("raw-session").unwrap(),
            render_identity_answer,
        ),
    ] {
        assert_eq!(events.len(), 1, "{ask}");
        assert_eq!(
            events[0].pairs,
            [("content".into(), RAW.into())],
            "{ask} engine bytes"
        );
        let object =
            |raw: &str| serde_json::from_str::<BTreeMap<String, Box<RawValue>>>(raw).unwrap();
        let frame = object(&render(&events));
        let answer = object(frame["answer"].get());
        let body = object(answer[ask].get());
        let rows: Vec<Box<RawValue>> = serde_json::from_str(body[field].get()).unwrap();
        let row = object(rows[0].get());
        let pairs = object(row["pairs"].get());
        assert_eq!(pairs["content"].get(), RAW, "{ask} answer bytes");
    }
}
