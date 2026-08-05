//! conforms: types-config-format-yaml
//! conforms: types-config-names-kebab
//! conforms: types-no-default-derive
//! conforms: types-trace-sink-discriminated
//! conforms: types-config-parse-total
//!
//! The agent config: the declarative document that defines an agent, per
//! `weaver-types-Spec` section 2. Written by the operator, validated by admin
//! before a process exists, read by the harness for the elections it carries.
//! The format is YAML, elected against the charter's writer-audience criterion,
//! and the parser sits behind the non-default `config` cargo feature: only admin
//! and the harness parse the file, and the wire types below compile with the
//! feature off.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::identity::AccessRule;

/// The declared surface of an agent, six fields, every one required.
///
/// Absence is a refusal rather than a default: an operator who stated no
/// residual readout has not thereby declined it, and admin refusing the load is
/// how that operator learns the file is incomplete. This is why the type derives
/// no `Default` and [`parse`] returns no partial value.
///
/// Field names are kebab-case on disk and snake_case here, by explicit election,
/// and `deny_unknown_fields` is the fixed-surface mechanism: a field no organ
/// registered refuses rather than being ignored, so a typo in `permission-mode`
/// cannot silently vanish.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AgentConfig {
    pub model_binding: ModelBinding,
    pub tool_set: Vec<ToolName>,
    pub permission_mode: weaver_traits::PermissionMode,
    pub residual_readout_election: bool,
    pub gate_instruction: GateInstruction,
    pub trace_sink: TraceSink,
}

/// The model artifact and the devices it is assigned to.
///
/// The vector is ordered and the order is the shard order. An empty set is a
/// parse error rather than a default, because a binding assigning no device is a
/// declaration the operator did not finish, and defaulting it to device zero is
/// the placement decision this crate exists not to make. Whether the devices
/// exist and can shard is admission's; this crate answers well-formed only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ModelBinding {
    pub artifact: ArtifactRef,
    pub devices: Vec<DeviceOrdinal>,
}

/// The instruction the gate resolves and never interprets beyond its fields.
///
/// The operator writes it, admin validates it, the harness carries it
/// uninterpreted, and the gate consumes exactly two things from it: the socket
/// path to bind and the access rule the predicate judges against, per
/// `weaver-gate-Spec` section 3, whose demand this satellite carries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GateInstruction {
    pub socket_path: PathBuf,
    pub access_rule: AccessRule,
}

/// Where the stream lands: a sink, not only a path.
///
/// A file, a pipe, or a socket are all conforming sinks, so the field carries a
/// discriminated shape and admin opens by the discriminant rather than guessing
/// from the filesystem. `File` and `Pipe` carry a creation flag because admin
/// can make either; a socket sink is different in kind, something on the
/// operator's side must already be listening, so a creation flag would promise
/// an act admin cannot perform and a missing socket sink always refuses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum TraceSink {
    File { path: PathBuf, create: bool },
    Pipe { path: PathBuf, create: bool },
    Socket { path: PathBuf },
}

/// What an operator writes to name a model artifact. Resolution is admin's and
/// readability is the SPU's; this crate answers well-formed only.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtifactRef(pub String);

/// An unsigned device number, so a negative one is a parse error rather than a
/// check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceOrdinal(pub u32);

/// A tool's name. A name today, gaining its element type with the tool
/// workflow, because it elects from `tool-trait`, which the traits charter
/// holds blocked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolName(pub String);

/// A config field's name, as the operator spells it, kebab-case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldName(pub String);

/// A refused parse, typed: the field it names where one names itself, and what
/// went wrong.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigError {
    pub field: Option<FieldName>,
    pub kind: ConfigErrorKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigErrorKind {
    Malformed,
    MissingField,
    UnknownField,
    BadValue,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            ConfigErrorKind::Malformed => "malformed document",
            ConfigErrorKind::MissingField => "missing required field",
            ConfigErrorKind::UnknownField => "unknown field",
            ConfigErrorKind::BadValue => "bad value",
        };
        match &self.field {
            Some(FieldName(name)) => write!(f, "{kind}: {name}"),
            None => write!(f, "{kind}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// A total parse into a typed value: the whole config or a typed refusal, and
/// nothing partial, so a half-valid config is unrepresentable rather than
/// merely refused.
///
/// The signature is the pin (`types-config-parse-total`): the crate exposes
/// this and no builder and no field-by-field accessor.
///
/// ```
/// let _: fn(&str) -> Result<weaver_types::AgentConfig, weaver_types::ConfigError> =
///     weaver_types::parse;
/// ```
#[cfg(feature = "config")]
pub fn parse(source: &str) -> Result<AgentConfig, ConfigError> {
    let config: AgentConfig =
        serde_yaml_ng::from_str(source).map_err(|e| classify_yaml_error(&e.to_string()))?;
    if config.model_binding.devices.is_empty() {
        return Err(ConfigError {
            field: Some(FieldName("model-binding".to_string())),
            kind: ConfigErrorKind::BadValue,
        });
    }
    Ok(config)
}

/// Sorts a serde_yaml_ng error message into the typed kinds. The messages are
/// serde's and their shapes are stable across the 1.x derive: `missing field
/// `name``, `unknown field `name``, `invalid type`, `unknown variant`.
#[cfg(feature = "config")]
fn classify_yaml_error(message: &str) -> ConfigError {
    fn backticked(message: &str, after: &str) -> Option<FieldName> {
        let rest = message.split(after).nth(1)?;
        let name = rest.split('`').nth(1)?;
        Some(FieldName(name.to_string()))
    }
    if message.contains("missing field") {
        ConfigError {
            field: backticked(message, "missing field"),
            kind: ConfigErrorKind::MissingField,
        }
    } else if message.contains("unknown field") {
        ConfigError {
            field: backticked(message, "unknown field"),
            kind: ConfigErrorKind::UnknownField,
        }
    } else if message.contains("invalid type")
        || message.contains("invalid value")
        || message.contains("unknown variant")
    {
        ConfigError {
            field: None,
            kind: ConfigErrorKind::BadValue,
        }
    } else {
        ConfigError {
            field: None,
            kind: ConfigErrorKind::Malformed,
        }
    }
}
