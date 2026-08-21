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
use crate::wire::SessionId;

/// The declared surface of an agent, six fields under five keys, every one
/// required.
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
    pub session: SessionId,
    pub spu_instruction: SpuInstruction,
    pub tool_set: Vec<ToolName>,
    pub permission_mode: weaver_traits::PermissionMode,
    pub gate_instruction: GateInstruction,
    pub trace_sink: TraceSink,
    /// The tee's election, per `weaver-types-Spec` section 2: the one
    /// optional field, optional by the required-field rule's own exception
    /// because `weaver-state-PRD` section 4 rules what absence means - the
    /// default election, the envelope of every kind. Admin resolves the
    /// absence at inventory, so the worker never re-derives it.
    #[serde(default)]
    pub state_election: Option<StateElection>,
    /// The agent's loop file, per `weaver-types-Spec` section 2 and the
    /// ruling of 2026-08-20 on issue #243: the loop is a member of this
    /// agent's harness and unique to it. The second optional field, by the
    /// required-field rule's own exception because `weaver-harness-PRD`
    /// section 2 rules what absence means - the worker's own default loop.
    /// It reaches the worker on the unit's argument vector, never in an
    /// exchange, because no exchange carries a path.
    #[serde(default)]
    pub loop_file: Option<PathBuf>,
}

/// The probability field's election, per `weaver-types-Spec` section 2: how
/// many candidates are ranked and reported at each decode position.
///
/// **The depth has a floor and the floor is the sampling cutoff**, judged
/// by the SPU at admit per `weaver-spu-Spec` section 7.5 rather than here.
/// The sampler truncates before it draws, so top-k puts a real wall in the
/// field where the reported depth is an artifact of the reporting, and
/// telling them apart requires reporting past the wall.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct FieldElection {
    pub depth: u32,
}

/// The operator's election of payload key paths for the state tee, per
/// `weaver-types-Spec` section 2. The resolved default's spelling is fixed
/// there so two resolvers cannot disagree: `all_kinds` true and `keys`
/// empty. The empty list is only the default's spelling: `keys` stays
/// meaningful beside `all_kinds` true, each named kind adding payload
/// paths on top of the envelope every kind already crosses with. When the
/// block is present in the file, both members are required, the
/// required-field discipline resuming inside it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct StateElection {
    pub all_kinds: bool,
    pub keys: Vec<ElectedKindConfig>,
}

impl Default for StateElection {
    fn default() -> Self {
        StateElection {
            all_kinds: true,
            keys: Vec::new(),
        }
    }
}

/// One kind's election: the kind as the canonical form spells it, and the
/// payload key paths elected for it, dotted from the payload root.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ElectedKindConfig {
    pub kind: String,
    pub paths: Vec<String>,
}

/// The SPU's section of the declaration, per `weaver-types-Spec` section 2:
/// an organ's fields are named together and cross together, the pattern
/// `gate_instruction` already takes. The operator writes it, admin validates
/// it, the harness carries it uninterpreted, and the SPU consumes it at
/// admit.
///
/// `decoder` names a role rather than a slot. The organ's domain is every
/// semantic operation in the text modality, and the decode role is the one
/// whose seam stands, so an embedder key arrives in the act that builds an
/// embedder rather than being carried empty here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SpuInstruction {
    pub decoder: DecoderInstruction,
    /// The classify role, the section's second key, per `weaver-types-Spec`
    /// section 2 as of the classifier act: optional by presence, absence
    /// being the operator's declaration that the agent runs no classifier,
    /// per `weaver-spu-PRD` section 15.3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classify: Option<ClassifyInstruction>,
}

/// The classify role's declaration: the model binding at the smaller size,
/// per `weaver-types-Spec` section 2. Every field of a present section is
/// required, the no-defaulting rule untouched by the section's own option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ClassifyInstruction {
    pub model_binding: ModelBinding,
}

/// The decode role's declaration: which model serves it and whether the
/// residual readout is elected for the load. Absence of either field refuses
/// the parse at any depth, the no-defaulting rule of `weaver-types-Spec`
/// section 2 surviving the nesting untouched.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct DecoderInstruction {
    pub model_binding: ModelBinding,
    pub residual_readout_election: bool,
    /// The probability field's election, per `weaver-types-Spec` section 2
    /// and `weaver-spu-PRD` section 13.11. Optional because the election is
    /// what makes the field exist and its absence is the ordinary posture
    /// rather than a value withheld, which is why it takes no default and
    /// its absence is not read as one.
    ///
    /// **It stands beside the readout's election and is never merged with
    /// it.** Each diagnostic election stands alone and none is bundled
    /// under a name for a set: a named set drifts as members join it, and
    /// every record already carrying the name becomes a record of
    /// something else without any event saying so.
    #[serde(default)]
    pub field_election: Option<FieldElection>,
    /// The session's identity material: the canonical messages the identity
    /// prefix is rendered from, configuration rather than history, per
    /// `weaver-types-Spec` section 2. Required like every field, and an
    /// empty list is a declaration the operator made where an absent field
    /// is a file unfinished.
    pub identity: Vec<weaver_traits::Message>,
    /// Values for whatever parameters this deployment's SPU left tunable, per
    /// `weaver-types-Spec` section 2 and `weaver-spu-Spec` section 8. This is
    /// the route `Disposition::OperatorTunable` names, keyed by the
    /// parameter's name.
    ///
    /// **A map rather than a field per parameter**, because which parameters a
    /// binary leaves tunable is that binary's election and moves with a
    /// recompile: a floor type enumerating them would move with every
    /// deployment that changed its mind. Required like every field and may be
    /// empty, an empty map being a declaration that supplies nothing. A name
    /// this binary froze is ignored where it appears, frozen meaning compiled
    /// in and never carried, so a declaration cannot move what a deployment
    /// locked.
    pub tunable_values: std::collections::BTreeMap<String, f64>,
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
/// uninterpreted, and the gate consumes it beside the socket the raise
/// directive carries, per `weaver-gate-Spec` section 3.
///
/// **The socket is not here, and that is the ruling of 2026-08-15.** Where a
/// door stands is the program's and only who may pass is the operator's, per
/// `weaver-gate-PRD` section 2. A pathname the operator wrote could sit
/// outside the unit's runtime directory, where it outlives its worker and
/// refuses the next bind, so the program places it inside and the hazard
/// becomes unreachable rather than checked for.
///
/// The group survives with one field rather than collapsing to a bare rule,
/// on the pattern `spu_instruction` takes, so a field the gate workflow adds
/// later has somewhere to land.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct GateInstruction {
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
    if config
        .spu_instruction
        .decoder
        .model_binding
        .devices
        .is_empty()
    {
        return Err(ConfigError {
            field: Some(FieldName("model-binding".to_string())),
            kind: ConfigErrorKind::BadValue,
        });
    }
    check_tunable_values(&config.spu_instruction.decoder.tunable_values)?;
    check_trace_sink_surface(source, &config.trace_sink)?;
    Ok(config)
}

/// **Finiteness is the whole of what this crate can judge about a tunable
/// value, and the boundary is deliberate.** A `NaN` compares false against
/// every bound and an infinity is one no filter clamps, and neither is
/// specific to which parameter carried it, so the check belongs here.
///
/// Whether a value suits its parameter is not this crate's to say. Which names
/// are counts and which are reals is the SPU's election, held in its
/// dispositions and moving with its recompile, so a list of them here would be
/// the same fact in two places with no authority named, which G5 files as a
/// defect rather than resolves. The SPU judges that at resolve, before any
/// device work, per `weaver-spu-Spec` section 8.
#[cfg(feature = "config")]
fn check_tunable_values(
    values: &std::collections::BTreeMap<String, f64>,
) -> Result<(), ConfigError> {
    for (name, value) in values {
        if !value.is_finite() {
            return Err(ConfigError {
                field: Some(FieldName(format!("tunable-values.{name}"))),
                kind: ConfigErrorKind::BadValue,
            });
        }
    }
    Ok(())
}

/// The fixed-surface check the derive cannot carry: serde's
/// `deny_unknown_fields` does not compose with an internally tagged enum, so an
/// unknown key inside `trace-sink` would be silently discarded by the typed
/// parse alone, which is the vanishing-declaration failure the refusal exists
/// to prevent. The raw mapping is read back and its keys are judged against the
/// selected variant's surface: `file` and `pipe` carry `kind`, `path`, and
/// `create`, and `socket` carries `kind` and `path`.
#[cfg(feature = "config")]
fn check_trace_sink_surface(source: &str, sink: &TraceSink) -> Result<(), ConfigError> {
    let malformed = || ConfigError {
        field: None,
        kind: ConfigErrorKind::Malformed,
    };
    let document: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(source).map_err(|_| malformed())?;
    let mapping = document
        .get("trace-sink")
        .and_then(|v| v.as_mapping())
        .ok_or_else(malformed)?;
    let allowed: &[&str] = match sink {
        TraceSink::File { .. } | TraceSink::Pipe { .. } => &["kind", "path", "create"],
        TraceSink::Socket { .. } => &["kind", "path"],
    };
    for key in mapping.keys() {
        let key = key.as_str().ok_or_else(malformed)?;
        if !allowed.contains(&key) {
            return Err(ConfigError {
                field: Some(FieldName(format!("trace-sink.{key}"))),
                kind: ConfigErrorKind::UnknownField,
            });
        }
    }
    Ok(())
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
