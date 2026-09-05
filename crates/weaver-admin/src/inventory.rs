//! conforms: admin-inventory-one-function
//! conforms: admin-existence-checks-repair-nothing
//! conforms: admin-boundary-denies-agent-traversal
//! conforms: admin-checks-no-device
//! conforms: admin-identity-from-validated-name
//! conforms: admin-kind-mismatch-refused-at-inventory
//!
//! The inventory, per `weaver-admin-Spec` section 4: **one function**, called
//! by `validate` and by `load`'s steps 2 and 3, refusing at the first failure
//! with the field or check named. The two callers cannot drift, which is the
//! charter's one-code-path-entered-two-ways rule made structural.

use std::path::Path;

use weaver_types::{
    AgentConfig, AgentName, BindingKind, ConfigErrorKind, EnterBinding, FieldName,
    LifecycleRefusal, StoreEngine, TraceSink,
};

/// What the boundary check needs to know about the host, supplied by the
/// caller rather than discovered here, so a test can present a boundary
/// without provisioning one.
#[derive(Debug, Clone)]
pub struct Boundary {
    /// The agent's uid, resolved from the validated name.
    pub agent_uid: u32,
    /// The admin principal's uid, for the custody half of the sink boundary.
    pub admin_uid: u32,
    /// The agent's groups, for the search-bit reasoning below.
    pub agent_gids: Vec<u32>,
    /// The agent's home, which must exist.
    pub home: std::path::PathBuf,
    /// The state member's binary where the box carries one beside the
    /// worker's, per `weaver-admin-Spec` section 4 as of 2026-09-04: every
    /// election but `none` requires it, so its absence is the box's fault
    /// and never an absent member.
    pub member_binary: Option<std::path::PathBuf>,
    /// The directory the store's socket stands in, from this crate's own
    /// configuration, read under the service engine alone.
    pub store_socket: std::path::PathBuf,
}

/// The fleet's allow-list: the names the operator delegated, per charter
/// section 7.
#[derive(Debug, Clone)]
pub struct AllowList {
    names: Vec<String>,
}

impl AllowList {
    pub fn new(names: impl IntoIterator<Item = String>) -> Self {
        AllowList {
            names: names.into_iter().collect(),
        }
    }

    pub fn admits(&self, name: &AgentName) -> bool {
        self.names.iter().any(|n| n == &name.0)
    }
}

/// The one identity-constructing site.
///
/// The constructed identity is `weaver-<name>` **from the validated name,
/// never from a caller-supplied string**, which is the argless-grant
/// discipline landing at the one site that constructs. It is the one site
/// because the same validated name is what the unit template interpolates, so
/// the delegated authority has one origin.
pub fn identity_for(name: &AgentName) -> String {
    format!("weaver-{}", name.0)
}

/// The report a completed inventory yields: what was read, for the caller that
/// proceeds to a load and for the verb that stops here.
#[derive(Debug, Clone)]
pub struct Inventory {
    pub config: AgentConfig,
    pub identity: String,
    /// The declaration file's digest as this inventory read it, per
    /// `weaver-admin-harness-contract` section 5 as of 2026-09-05: supplied
    /// on the enter so the run and the record name what they were built
    /// from, and this crate is the file's one reader.
    pub declaration: String,
    /// The binding kind resolved, per `weaver-admin-Spec` section 7: the one
    /// inventory function resolves it, so the verb and the load cannot
    /// resolve differently, and what the enter carries is this value.
    pub binding: EnterBinding,
}

/// The one inventory function.
///
/// The allow-list is consulted before anything else is touched. The parse is
/// the floor's - `weaver_types::parse` yields a whole config or a typed error,
/// and this crate adds no partial reader. The existence checks are admin's,
/// and **each is a look rather than an ask**: nothing is repaired and nothing
/// is built.
///
/// **The devices the binding assigns are not checked here**, and the absence
/// is deliberate: whether they exist, have room, or can reach each other are
/// questions about hardware, and admin reasons about the device at no point.
/// An admin that verified the GPU would be a second arbitrator reintroduced as
/// a convenience.
pub fn take_inventory(
    name: &AgentName,
    source: &str,
    allow_list: &AllowList,
    boundary: &Boundary,
) -> Result<Inventory, LifecycleRefusal> {
    take_inventory_against(name, source, allow_list, boundary, None)
}

/// The whole of `take_inventory` with the host's answer about the agent's
/// group supplied rather than read.
///
/// **The seam exists so the reachability call site is watched.** The judgment
/// `unreachable_peer_against` makes was already testable this way and the
/// wiring, whether `take_inventory` calls it and refuses on what it says, was
/// not: its only watch first searched the box for a provisioned agent group
/// excluding the caller, found none on a developer box and none on CI, and
/// returned early - so it skipped in both environments the suite runs in
/// while its record claimed a perturbation. A watch that runs nowhere is the
/// thing this program calls representation.
///
/// `None` reads the host, which is every caller outside the tests.
fn take_inventory_against(
    name: &AgentName,
    source: &str,
    allow_list: &AllowList,
    boundary: &Boundary,
    group: Option<&ResolvedGroup>,
) -> Result<Inventory, LifecycleRefusal> {
    if !allow_list.admits(name) {
        return Err(LifecycleRefusal::NoSuchAgent);
    }
    let identity = identity_for(name);

    let config = weaver_types::parse(source).map_err(|e| match e.kind {
        ConfigErrorKind::MissingField
        | ConfigErrorKind::UnknownField
        | ConfigErrorKind::BadValue
        | ConfigErrorKind::Malformed => LifecycleRefusal::ConfigInvalid { field: e.field },
    })?;

    // The kind conditions the gate instruction's presence, per
    // `weaver-types-Spec` section 2: a serving declaration requires it and a
    // diagnostic declaration excludes it. The parse checks each field alone,
    // so the cross-field rule lands here, before any look at the filesystem
    // and before any unit starts. Absence of the kind means serving, per
    // `weaver-types-PRD` section 2.1, so a declaration written before the
    // member existed resolves as it always meant.
    let binding = match (
        config.binding_kind.clone().unwrap_or(BindingKind::Serving),
        config.gate_instruction.clone(),
    ) {
        (BindingKind::Serving, Some(gate_instruction)) => {
            EnterBinding::Serving { gate_instruction }
        }
        (BindingKind::Diagnostic, None) => EnterBinding::Diagnostic,
        _ => {
            return Err(LifecycleRefusal::ConfigInvalid {
                field: Some(FieldName("gate-instruction".into())),
            });
        }
    };

    // **The store election is judged here**, per `weaver-admin-Spec` section
    // 4 as of 2026-09-04, the declaration's half first, before any look at
    // the box. The parse checks each field alone, so the cross-field rules
    // land here: `none` declines the member and so refuses a state election
    // beside it, an election with no member to receive it being malformed
    // rather than surplus, and `database` and `role` belong to the service
    // engine exactly, absent under every other engine and present under it.
    let store = config.state_store.clone().unwrap_or_default();
    let store_invalid = |field: &str| LifecycleRefusal::ConfigInvalid {
        field: Some(FieldName(field.into())),
    };
    match store.engine {
        StoreEngine::None if config.state_election.is_some() => {
            return Err(store_invalid("state-election"));
        }
        StoreEngine::None | StoreEngine::Sqlite => {
            if store.database.is_some() {
                return Err(store_invalid("state-store.database"));
            }
            if store.role.is_some() {
                return Err(store_invalid("state-store.role"));
            }
        }
        StoreEngine::Postgres => {
            if store.database.is_none() {
                return Err(store_invalid("state-store.database"));
            }
            if store.role.is_none() {
                return Err(store_invalid("state-store.role"));
            }
        }
    }

    // A granted permission member is refused, per `weaver-admin-Spec`
    // section 7: the member is this crate's to set from the resolved kind at
    // the construction, and it parses with `default` because one type
    // serves the declaration and the seam, so the refusal lands here, where
    // the file is seen whole, rather than at a parse that would also refuse
    // the instruction this crate authors.
    if config.spu_instruction.decoder.refeed_permission {
        return Err(LifecycleRefusal::ConfigInvalid {
            field: Some(FieldName(
                "spu-instruction.decoder.refeed-permission".into(),
            )),
        });
    }
    if config.spu_instruction.decoder.column_permission {
        return Err(LifecycleRefusal::ConfigInvalid {
            field: Some(FieldName(
                "spu-instruction.decoder.column-permission".into(),
            )),
        });
    }

    // The model artifact resolves to something readable. Nothing is repaired.
    if !artifact_readable(&config.spu_instruction.decoder.model_binding.artifact.0) {
        return Err(LifecycleRefusal::ArtifactUnresolvable);
    }

    // The agent's home exists.
    if !boundary.home.is_dir() {
        return Err(LifecycleRefusal::BoundaryUnverified);
    }

    // The sink exists or its creation flag is set.
    if !sink_present_or_creatable(&config.trace_sink) {
        return Err(LifecycleRefusal::BoundaryUnverified);
    }

    // **The sink boundary has two halves and neither implies the other.**
    //
    // Denial: the containing directory denies the agent uid the search bit, so
    // the kernel refuses the lookup before any mode on the file is consulted.
    //
    // Custody: the directory is owned by root or by the admin principal. A
    // directory owned by some third principal at mode 0700 satisfies the
    // denial and defeats the custody, because that owner can chmod, replace,
    // or remove the file admin opened - and custody of where the record leaves
    // the system is admin's by charter. Ownership does not by itself deny a
    // traversal that mode grants, and denial does not by itself establish
    // custody, so both are checked.
    let directory = sink_directory(&config.trace_sink);
    if agent_can_traverse(directory, boundary) {
        return Err(LifecycleRefusal::BoundaryUnverified);
    }
    if !admin_holds_custody(directory, boundary) {
        return Err(LifecycleRefusal::BoundaryUnverified);
    }

    // **The store's box half**, per `weaver-admin-Spec` section 4 as of
    // 2026-09-04. Every election but `none` stands a member and so requires
    // the member's binary: the leg's standing is the declaration's fact and
    // not the directory's, per `weaver-state-PRD` section 4 and issue #381,
    // so a box lacking the binary refuses rather than running without a leg
    // the declaration never declined. The service engine further requires
    // the store's socket under the configured directory, and the walk asks
    // the store the two questions the charter's two gates pose: that this
    // account, the member's, maps to the declared role, and that the agent's
    // uid maps to none. Each is `BoundaryUnverified` and never
    // `ConfigInvalid`, for the reason the group case below gives: the
    // declaration is well formed and the fault is the provisioning's.
    if store.engine != StoreEngine::None && boundary.member_binary.is_none() {
        eprintln!("boundary unverified: no weaver-state binary beside the worker's");
        return Err(LifecycleRefusal::BoundaryUnverified);
    }
    if store.engine == StoreEngine::Postgres {
        let (Some(database), Some(role)) = (store.database.as_deref(), store.role.as_deref())
        else {
            unreachable!("the declaration's half required both");
        };
        let socket = boundary.store_socket.join(STORE_SOCKET_LEAF);
        if !std::fs::metadata(&socket)
            .map(|m| std::os::unix::fs::FileTypeExt::is_socket(&m.file_type()))
            .unwrap_or(false)
        {
            eprintln!(
                "boundary unverified: no store socket at {}",
                socket.display()
            );
            return Err(LifecycleRefusal::BoundaryUnverified);
        }
        match store_admits(&boundary.store_socket, database, role) {
            Ok(true) => {}
            Ok(false) => {
                eprintln!(
                    "boundary unverified: the store does not map this account to role {role:?} \
                     on database {database:?}"
                );
                return Err(LifecycleRefusal::BoundaryUnverified);
            }
            Err(e) => {
                eprintln!("boundary unverified: the store could not be asked: {e}");
                return Err(LifecycleRefusal::BoundaryUnverified);
            }
        }
        match store_admits_as(boundary, database, role) {
            Ok(false) => {}
            Ok(true) => {
                eprintln!(
                    "boundary unverified: the store maps the agent's uid {} to role {role:?}",
                    boundary.agent_uid
                );
                return Err(LifecycleRefusal::BoundaryUnverified);
            }
            Err(e) => {
                eprintln!("boundary unverified: the store could not be asked as the agent: {e}");
                return Err(LifecycleRefusal::BoundaryUnverified);
            }
        }
    }

    // **The access rule is checked against the mode that will carry it**, per
    // `weaver-admin-Spec` section 6 and the operator's ruling of 2026-08-28.
    //
    // **Last of the walks**, so it preempts none of them: an artifact or a
    // boundary that refuses is the older and narrower fact, and a check
    // running ahead of them made three inventory tests depend on whether the
    // box happened to carry the agent's group.
    //
    // The gate's socket is `0770` owned by the agent's group, per
    // `weaver-gate-Spec` section 3, so reaching it takes membership in that
    // group. A rule admitting a uid outside it names a peer the filesystem
    // turns away at `connect(2)` before the credential check ever runs: the
    // dialer sees `Permission denied`, the driver reports the socket never
    // stood, and the gate logs nothing because the peer never reached
    // `accept`. That diagnosis cost two runs on 2026-08-27 while it was an
    // unprovisioned box, and the mode makes it the designed behaviour unless
    // the two are made to agree.
    //
    // **The two locks may narrow the same set and may not contradict.** So a
    // rule the mode would silently defeat refuses here, named, before any
    // unit starts, rather than at a `connect` no layer reports.
    //
    // **The rule half is the serving binding's and the group half is every
    // binding's.** `start_arguments` emits `--property=Group={identity}` for
    // every unit, so a box carrying the agent user and not its group fails
    // `systemd-run` with the opaque credential error whatever the binding is,
    // and gating the whole check on `Serving` let a diagnostic declaration
    // pass validate clean and fail at load. A diagnostic binding is asked
    // with an empty rule, which reaches the group arm and names no peer.
    let empty_rule = weaver_types::AccessRule {
        allowed_uids: Default::default(),
        allowed_gids: Default::default(),
        denied_uids: Default::default(),
    };
    let rule = match &binding {
        EnterBinding::Serving { gate_instruction } => &gate_instruction.access_rule,
        _ => &empty_rule,
    };
    if let Some(unreachable) = match group {
        Some(known) => unreachable_peer_against(&identity, rule, known),
        None => unreachable_peer(&identity, rule),
    } {
        // **`BoundaryUnverified` and not `ConfigInvalid`.** The declaration is
        // well formed and the fault is the box's: the operator wrote a uid
        // that ought to reach the socket and the provisioning has not put it
        // in the agent's group. `ConfigInvalid` names the YAML, and a
        // deployer following that reading deletes the uid from
        // `allowed-uids` - which makes validate pass and breaks the connector
        // for good, the credential check then denying it at `accept` with
        // nothing saying why. This is the same fault an unprovisioned home or
        // sink is, and it answers as they do.
        //
        // **The remedy comes from the case and not from this call site.** The
        // two refusals want different instructions, and a single sentence
        // here told a deployer whose group was missing to run `gpasswd`,
        // which is the command that fails on exactly that box.
        eprintln!(
            "boundary unverified: {}: {}",
            unreachable.field(&identity),
            unreachable.remedy(&identity)
        );
        return Err(LifecycleRefusal::BoundaryUnverified);
    }

    Ok(Inventory {
        config,
        identity,
        declaration: declaration_digest(source),
        binding,
    })
}

/// The service engine's conventional socket directory, standing where this
/// crate's configuration names none.
pub const STORE_SOCKET_DIRECTORY: &str = "/run/postgresql";

/// The leaf the store's socket carries under its directory, at the engine's
/// default port.
pub const STORE_SOCKET_LEAF: &str = ".s.PGSQL.5432";

/// The name of the environment variable under which this binary, re-executed
/// as the agent's uid, answers the store's second question instead of
/// serving a verb: no verb of the operator's surface is added for a question
/// the surface never asks.
pub const PROBE_STORE_VARIABLE: &str = "WEAVER_ADMIN_PROBE_STORE";

/// **Does the store admit this process as `role` on `database`?** Asked in
/// the engine's own startup handshake over the unix socket, with no client
/// library: one startup message carrying the role and database, and one
/// answer read, an authentication-complete meaning the store's peer
/// authentication mapped this account to the role, and anything else meaning
/// it did not. An authentication method this process cannot answer, a
/// password or a challenge, counts as not admitted, because the member
/// authenticates by peer credential and nothing else. The error names a
/// socket that could not be spoken to, never a refusal.
pub fn store_admits(socket_dir: &Path, database: &str, role: &str) -> std::io::Result<bool> {
    use std::io::{Read, Write};
    let mut stream = std::os::unix::net::UnixStream::connect(socket_dir.join(STORE_SOCKET_LEAF))?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    let mut body = Vec::new();
    body.extend_from_slice(&196_608u32.to_be_bytes());
    for (key, value) in [("user", role), ("database", database)] {
        body.extend_from_slice(key.as_bytes());
        body.push(0);
        body.extend_from_slice(value.as_bytes());
        body.push(0);
    }
    body.push(0);
    let mut message = ((body.len() + 4) as u32).to_be_bytes().to_vec();
    message.extend_from_slice(&body);
    stream.write_all(&message)?;
    let mut head = [0u8; 5];
    stream.read_exact(&mut head)?;
    let length = u32::from_be_bytes([head[1], head[2], head[3], head[4]]) as usize;
    let admitted = if head[0] == b'R' && length >= 8 {
        let mut code = [0u8; 4];
        stream.read_exact(&mut code)?;
        u32::from_be_bytes(code) == 0
    } else {
        false
    };
    let _ = stream.write_all(&[b'X', 0, 0, 0, 4]);
    Ok(admitted)
}

/// **Does the store admit the agent's uid as `role`?** The question the
/// charter's second gate poses, and the one this process cannot ask as
/// itself: peer authentication reads the connecting uid, so this binary
/// re-executes itself under the agent's uid and primary gid with
/// [`PROBE_STORE_VARIABLE`] set, and the child asks [`store_admits`] and
/// exits zero for admitted and one for refused. Any other exit is the probe
/// failing to run, which is an error and not an answer.
pub fn store_admits_as(boundary: &Boundary, database: &str, role: &str) -> std::io::Result<bool> {
    use std::os::unix::process::CommandExt;
    let mut probe = std::process::Command::new(std::env::current_exe()?);
    probe
        .env_clear()
        .env(PROBE_STORE_VARIABLE, "1")
        .arg(&boundary.store_socket)
        .arg(database)
        .arg(role)
        .uid(boundary.agent_uid)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    if let Some(gid) = boundary.agent_gids.first() {
        probe.gid(*gid);
    }
    match probe.status()?.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        other => Err(std::io::Error::other(format!(
            "the probe under uid {} ended with {other:?}",
            boundary.agent_uid
        ))),
    }
}

/// The declaration's digest, sha256 of the file's bytes as read, hex.
pub fn declaration_digest(source: &str) -> String {
    use sha2::Digest;
    let digest = sha2::Sha256::digest(source.as_bytes());
    let mut hex = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write;
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// The names the allow-list admits, in the operator's order, for the verbs
/// that answer for every agent at once.
impl AllowList {
    pub fn names(&self) -> &[String] {
        &self.names
    }
}

/// The first peer an access rule admits that the socket's mode will turn
/// away, named by its field, or `None` where the two agree.
///
/// **The mode and the rule are two locks on one door and may not
/// contradict.** The gate binds at `0770` owned by the agent's group, so a
/// peer reaches `accept` only through that group. A rule admitting a uid
/// outside it is a rule that does nothing, and does nothing invisibly: the
/// refusal lands at `connect(2)` where the gate cannot see it and no layer
/// above reports it as what it is.
///
/// **Only `allowed_uids` is judged.** A gid names no particular peer, and
/// whether one holding it reaches the socket turns on that peer's own
/// memberships rather than on the gid, so a rule admitting a gid is left to
/// the credential check. Uid 0 is skipped for a different reason: root
/// reaches a `0770` socket whatever group it holds.
///
/// **Unresolvable is not the same as unreachable.** Where this cannot read
/// the group at all it answers `None` rather than refusing a declaration on
/// a fact it could not establish, the load then failing later and loudly
/// rather than here and wrongly.
///
/// conforms: admin-access-rule-reaches-the-socket
fn unreachable_peer(identity: &str, rule: &weaver_types::AccessRule) -> Option<Unreachable> {
    unreachable_peer_against(identity, rule, &resolved_group(identity))
}

/// Why a declaration's rule cannot be met by the socket it will meet.
///
/// **Two cases and not one string.** The field a refusal names and the
/// instruction an operator needs differ between them, and an earlier form
/// returned one sentence for both - which `take_inventory` then spliced into
/// a field-path slot, telling a deployer to run `gpasswd` for a missing
/// group, where `gpasswd` is what fails when the group is what is absent.
#[derive(Debug, PartialEq, Eq)]
enum Unreachable {
    /// The rule admits a uid outside the agent's group, so it is turned away
    /// at `connect` before the credential check.
    UidOutsideGroup(u32),
    /// The agent user exists and its group does not, so the unit's `Group=`
    /// cannot resolve and the start fails opaquely.
    GroupMissing,
}

impl Unreachable {
    /// The field a refusal names, where one applies.
    fn field(&self, identity: &str) -> String {
        match self {
            Unreachable::UidOutsideGroup(uid) => {
                format!("gate-instruction.access-rule.allowed-uids.{uid}")
            }
            Unreachable::GroupMissing => format!("the {identity} group"),
        }
    }

    /// What the operator does about it. The two remedies are different and
    /// naming the wrong one costs a deployer the diagnosis.
    fn remedy(&self, identity: &str) -> String {
        match self {
            Unreachable::UidOutsideGroup(uid) => format!(
                "uid {uid} is outside the {identity} group, which the gate's \
                 socket mode admits by. Run `gpasswd -a <user> {identity}` \
                 rather than deleting the uid from the rule, which would make \
                 validate pass and leave the credential check denying it."
            ),
            Unreachable::GroupMissing => format!(
                "the {identity} user exists and the {identity} group does \
                 not, so the unit's Group= cannot resolve and the start \
                 fails. Run `groupadd {identity} && usermod -g {identity} \
                 {identity}`."
            ),
        }
    }
}

/// What the host says about an agent's group: its gid and members, or which
/// of the two absences it is.
enum ResolvedGroup {
    /// The group exists, with this gid and these member names.
    Present { gid: u32, members: Vec<String> },
    /// Neither the group nor the user: no agent is provisioned here.
    NoAgent,
    /// The user exists and the group does not, which fails the unit start.
    UserWithoutGroup,
    /// The lookup itself failed, so nothing about the group is known.
    Unresolvable,
}

fn resolved_group(identity: &str) -> ResolvedGroup {
    match nix::unistd::Group::from_name(identity) {
        Ok(Some(group)) => ResolvedGroup::Present {
            gid: group.gid.as_raw(),
            members: group.mem,
        },
        // **A lookup that failed is not a group that is absent.** A transient
        // `getgrnam_r` error would otherwise read as `UserWithoutGroup` on a
        // box where the user resolves, and refuse a load on a fact never
        // established - which is the rule the rest of this function follows.
        Err(_) => ResolvedGroup::Unresolvable,
        Ok(None) => {
            // **A missing group is a fault only where the agent user
            // exists.** `Group={identity}` is a hard start-time requirement,
            // so a box that provisioned the user and not the group fails the
            // unit start with an opaque credential error. That state is
            // nameable and is named. A box carrying neither - a bare
            // checkout, or CI - has provisioned no agent at all and is not
            // refused on a fact about an agent it does not have.
            match nix::unistd::User::from_name(identity) {
                Ok(Some(_)) => ResolvedGroup::UserWithoutGroup,
                _ => ResolvedGroup::NoAgent,
            }
        }
    }
}

/// The judgment, against a group already resolved.
///
/// **Separated from the lookup so the watch can run anywhere.** Reading the
/// host's group table made the test that guards this vacuous on both a
/// development box, where the operator is in every agent group, and on CI,
/// where no agent group exists - so the record claimed a watch that ran in
/// neither place.
///
/// conforms: admin-access-rule-reaches-the-socket
fn unreachable_peer_against(
    _identity: &str,
    rule: &weaver_types::AccessRule,
    group: &ResolvedGroup,
) -> Option<Unreachable> {
    let (gid, members) = match group {
        ResolvedGroup::Present { gid, members } => (*gid, members),
        ResolvedGroup::NoAgent | ResolvedGroup::Unresolvable => return None,
        ResolvedGroup::UserWithoutGroup => return Some(Unreachable::GroupMissing),
    };
    let members: std::collections::BTreeSet<&String> = members.iter().collect();

    for uid in &rule.allowed_uids {
        // **Root is not bound by the mode.** `CAP_DAC_OVERRIDE` reaches a
        // `0770` socket whatever group it holds, so refusing `allowed-uids:
        // [0]` would refuse a rule that works, on a premise false for that
        // one uid.
        if *uid == 0 {
            continue;
        }
        // **A uid the rule denies is not a peer this asks about.**
        // `weaver_types::authorized` gives `denied_uids` precedence over
        // `allowed_uids`, so a uid in both is refused at `accept` whatever
        // the mode does, and naming it unreachable would refuse a
        // declaration over a peer that was never going to be admitted.
        if rule.denied_uids.contains(uid) {
            continue;
        }
        let user = match nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(*uid)) {
            Ok(Some(user)) => user,
            // **A uid with no passwd entry is unreachable and not exempt.**
            // Group membership is recorded by name, so a uid carrying no
            // name is in no group and cannot hold the agent's: it reaches a
            // `0770` socket it does not own by no route. Continuing here was
            // an undeclared third exemption beside `allowed-gids` and uid 0,
            // and it passed exactly the declaration this check exists to
            // catch - the dialer turned away at `connect(2)` while the
            // driver reports the socket never stood.
            Ok(None) => return Some(Unreachable::UidOutsideGroup(*uid)),
            // A lookup that failed establishes nothing, which is the rule
            // the group resolution follows too.
            Err(_) => continue,
        };
        // The agent's own group reached either as a primary or a secondary.
        if user.gid.as_raw() == gid || members.contains(&user.name) {
            continue;
        }
        return Some(Unreachable::UidOutsideGroup(*uid));
    }
    // **`allowed_gids` is not judged here, and the omission is the finding.**
    // A gid names no particular peer, and whether a peer holding it reaches
    // the socket turns on that peer's own memberships: a rule admitting gid
    // 1000 works where the operator holding it is a supplementary member of
    // the agent's group, which is the shape one of these boxes has. An arm
    // refusing every gid but the agent's own refused that working
    // declaration. Deciding it properly means enumerating who holds the gid,
    // which this crate has no reason to do, so the rule's gid half rests on
    // the credential check alone and this says so rather than guessing.
    None
}

fn artifact_readable(artifact: &str) -> bool {
    let path = Path::new(artifact);
    // An artifact named by an absolute path must be readable; a bare name is
    // the operator's own reference and resolution beyond this crate's reach.
    if path.is_absolute() {
        return path.exists();
    }
    !artifact.is_empty()
}

pub(crate) fn sink_directory(sink: &TraceSink) -> &Path {
    let path = match sink {
        TraceSink::File { path, .. }
        | TraceSink::Pipe { path, .. }
        | TraceSink::Socket { path } => path,
    };
    path.parent().unwrap_or(Path::new("/"))
}

fn sink_present_or_creatable(sink: &TraceSink) -> bool {
    match sink {
        TraceSink::File { path, create } | TraceSink::Pipe { path, create } => {
            *create || path.exists()
        }
        // No creation flag exists for a socket sink: something of the
        // operator's must already be listening.
        TraceSink::Socket { path } => path.exists(),
    }
}

/// Whether the sink directory is held by a principal whose custody the charter
/// recognizes: root, or the admin principal itself. Any other owner controls
/// the file admin opened, whatever the mode currently reads.
/// **The window between this check and the sink's open is not closed here, and
/// the reason is the trust model rather than an oversight.** The path is
/// resolved twice, once for this validation and once when the sink is opened,
/// so a party able to write in the directory could replace the target or
/// redirect it by symlink between them. Every such party is root or the admin
/// principal, because the check below is what refuses any other owner, and the
/// charter secures the agent against reaching its own record while securing
/// nothing against the operator, who is trusted by construction. Descriptor
/// relative resolution would close the window against an untrusted writer that
/// this check has already excluded.
pub fn admin_holds_custody(directory: &Path, boundary: &Boundary) -> bool {
    use std::os::unix::fs::MetadataExt;
    let Ok(meta) = std::fs::metadata(directory) else {
        // A directory this crate cannot resolve establishes no custody, the
        // same reading the traversal half takes for the same reason.
        return false;
    };
    meta.uid() == 0 || meta.uid() == boundary.admin_uid
}

/// Whether the agent uid could traverse into the directory: it is the owner,
/// holds a group search bit through some membership, or the other bits carry
/// search. Any of those is a boundary the operator has not drawn.
pub fn agent_can_traverse(directory: &Path, boundary: &Boundary) -> bool {
    use std::os::unix::fs::MetadataExt;
    // **A path this crate cannot resolve yields no boundary evidence**, so it
    // is treated as traversable rather than as safe. A relative sink path's
    // parent is the empty path, whose metadata lookup fails, and reading that
    // failure as "not traversable" would admit a configuration nothing
    // verified.
    if !directory.is_absolute() {
        return true;
    }
    let Ok(meta) = std::fs::metadata(directory) else {
        return true;
    };
    let mode = meta.mode();
    // **An agent that owns the directory can restore the search bit itself**,
    // so ownership defeats the boundary whatever the mode currently reads.
    if meta.uid() == boundary.agent_uid {
        return true;
    }
    if boundary.agent_gids.contains(&meta.gid()) && mode & 0o010 != 0 {
        return true;
    }
    mode & 0o001 != 0
}

#[cfg(test)]
mod tests {
    //! The second walk, run from inside the crate because this crate publishes
    //! no library surface.

    use super::*;

    /// **A rule admitting a peer the socket's mode turns away is named.**
    ///
    /// The gate binds `0770` owned by the agent's group, so a uid outside it
    /// is refused at `connect(2)` - before `accept`, so the credential check
    /// never runs and the gate records nothing. Two locks on one door may
    /// narrow the same set and may not contradict.
    ///
    /// **Judged against a constructed table, so this runs everywhere.**
    /// Reading the host's made it vacuous on a development box, where the
    /// operator is in every agent group, and on CI, where no agent group
    /// exists - a watch that ran in neither place while its record claimed
    /// otherwise.
    ///
    /// Perturbation: return `None` unconditionally from
    /// `unreachable_peer_against` and this fails. The call site is watched
    /// separately below.
    ///
    /// conforms: admin-access-rule-reaches-the-socket
    #[test]
    fn a_rule_the_mode_would_defeat_is_named_rather_than_left_to_connect() {
        let outsider = 4242;
        let rule = weaver_types::AccessRule {
            allowed_uids: [outsider].into_iter().collect(),
            allowed_gids: Default::default(),
            denied_uids: Default::default(),
        };
        let excludes = ResolvedGroup::Present {
            gid: 6000,
            members: vec!["someone-else".to_string()],
        };
        // A uid the host does resolve, so this case is about the group and
        // not about the passwd entry: this process's own, which is in
        // neither the gid nor the member list above.
        let me = nix::unistd::Uid::current().as_raw();
        let mine = weaver_types::AccessRule {
            allowed_uids: [me].into_iter().collect(),
            allowed_gids: Default::default(),
            denied_uids: Default::default(),
        };
        if me != 0 {
            assert_eq!(
                unreachable_peer_against("weaver-x", &mine, &excludes),
                Some(Unreachable::UidOutsideGroup(me)),
                "a uid outside the group is named by its field"
            );
        }

        // Root is skipped: `CAP_DAC_OVERRIDE` reaches a `0770` socket
        // whatever group it holds.
        let root = weaver_types::AccessRule {
            allowed_uids: [0].into_iter().collect(),
            allowed_gids: Default::default(),
            denied_uids: Default::default(),
        };
        assert_eq!(unreachable_peer_against("weaver-x", &root, &excludes), None);

        // A gid is not judged at all, whether it is the group's or not.
        let by_gid = weaver_types::AccessRule {
            allowed_uids: Default::default(),
            allowed_gids: [1000, 6000].into_iter().collect(),
            denied_uids: Default::default(),
        };
        assert_eq!(
            unreachable_peer_against("weaver-x", &by_gid, &excludes),
            None
        );

        // **A uid the host cannot resolve is named rather than skipped**, and
        // this assertion said the opposite until 2026-08-29. Skipping it was
        // an undeclared third exemption beside `allowed-gids` and uid 0: a
        // uid with no passwd entry is in no group, membership being recorded
        // by name, so it reaches a `0770` socket it does not own by no route.
        assert_eq!(
            unreachable_peer_against("weaver-x", &rule, &excludes),
            Some(Unreachable::UidOutsideGroup(4242)),
            "a uid with no passwd entry is unreachable, not exempt"
        );

        // Membership by the group's own gid, and by the member list.
        let includes = ResolvedGroup::Present {
            gid: 6000,
            members: vec![
                nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(me))
                    .expect("this uid resolves")
                    .expect("this uid has a passwd entry")
                    .name,
            ],
        };
        assert_eq!(unreachable_peer_against("weaver-x", &mine, &includes), None);
    }

    /// **A half-provisioned box is named**, the user existing without its
    /// group being the state that fails the unit start with an opaque
    /// credential error. A box carrying neither is not refused on a fact
    /// about an agent it does not have.
    #[test]
    fn a_user_without_its_group_is_named_and_a_bare_box_is_not() {
        let rule = weaver_types::AccessRule {
            allowed_uids: Default::default(),
            allowed_gids: Default::default(),
            denied_uids: Default::default(),
        };
        assert_eq!(
            unreachable_peer_against("weaver-x", &rule, &ResolvedGroup::UserWithoutGroup),
            Some(Unreachable::GroupMissing),
            "the missing group is its own case and not a uid's"
        );
        // **The remedy is the one that works on that box.** A single sentence
        // covering both cases told this operator to run `gpasswd`, which is
        // what fails when the group is what is absent.
        let remedy = Unreachable::GroupMissing.remedy("weaver-x");
        assert!(
            remedy.contains("groupadd") && !remedy.contains("gpasswd"),
            "the missing-group remedy creates the group: {remedy}"
        );
        assert!(
            Unreachable::UidOutsideGroup(7)
                .remedy("weaver-x")
                .contains("gpasswd"),
            "the outside-uid remedy adds the member"
        );
        // And the field slot carries a field rather than a sentence.
        assert_eq!(
            Unreachable::UidOutsideGroup(7).field("weaver-x"),
            "gate-instruction.access-rule.allowed-uids.7"
        );
        assert_eq!(
            unreachable_peer_against("weaver-x", &rule, &ResolvedGroup::NoAgent),
            None
        );

        // A group this caller is not in, so the walk actually runs.
        let outside = ResolvedGroup::Present {
            gid: u32::MAX - 1,
            members: Vec::new(),
        };

        // **A uid with no passwd entry is unreachable, not exempt.** Group
        // membership is recorded by name, so a uid carrying no name holds no
        // group and reaches a `0770` socket it does not own by no route.
        // Continuing past it was an undeclared third exemption beside
        // `allowed-gids` and uid 0, and it passed exactly the declaration
        // this check exists to catch.
        let nameless = 4242;
        assert!(
            nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(nameless))
                .ok()
                .flatten()
                .is_none(),
            "the fixture uid must carry no passwd entry on this box"
        );
        let orphan = weaver_types::AccessRule {
            allowed_uids: [nameless].into_iter().collect(),
            allowed_gids: Default::default(),
            denied_uids: Default::default(),
        };
        assert_eq!(
            unreachable_peer_against("weaver-x", &orphan, &outside),
            Some(Unreachable::UidOutsideGroup(nameless)),
            "a uid with no passwd entry is named rather than skipped"
        );

        // **And a uid the rule itself denies is not asked about.**
        // `weaver_types::authorized` gives `denied_uids` precedence, so a uid
        // in both sets never reaches `accept` whatever the mode does, and
        // naming it would refuse a declaration over a peer already refused.
        let both = weaver_types::AccessRule {
            allowed_uids: [nameless].into_iter().collect(),
            allowed_gids: Default::default(),
            denied_uids: [nameless].into_iter().collect(),
        };
        assert_eq!(
            unreachable_peer_against("weaver-x", &both, &outside),
            None,
            "a denied uid is not a peer the mode is asked about"
        );
    }

    /// A group this cannot read is not a contradiction it may assert, so the
    /// check answers `None` rather than refusing on a fact it never
    /// established. The load then fails later and loudly.
    #[test]
    fn an_unreadable_group_is_not_read_as_a_contradiction() {
        let rule = weaver_types::AccessRule {
            allowed_uids: [1000].into_iter().collect(),
            allowed_gids: Default::default(),
            denied_uids: Default::default(),
        };
        assert_eq!(unreachable_peer("weaver-no-such-agent-here", &rule), None);
    }

    /// **The group half of the check covers every binding, not the serving
    /// one.**
    ///
    /// `start_arguments` emits `--property=Group={identity}` for every unit,
    /// so a box carrying the agent user and not its group fails
    /// `systemd-run` with an opaque credential error whatever the binding is.
    /// Gating the whole check on `EnterBinding::Serving` let a diagnostic
    /// declaration pass validate clean and fail at load, which is exactly the
    /// failure the `GroupMissing` arm exists to preempt.
    ///
    /// The other half of the rule still holds: a box carrying **neither** the
    /// user nor the group has provisioned no agent and is refused on nothing,
    /// which is what lets a bare checkout and CI run this at all.
    ///
    /// Perturbation: returning the group check to inside the `Serving` arm
    /// leaves the diagnostic half `Ok`. Watched failing 2026-08-29.
    ///
    /// conforms: admin-access-rule-reaches-the-socket
    #[test]
    fn a_diagnostic_declaration_meets_the_group_check_too() {
        let root = scratch("diagnostic-group");
        let sink_dir = root.join("sink");
        std::fs::create_dir_all(&sink_dir).expect("sink dir");
        let home = root.join("home");
        std::fs::create_dir_all(&home).expect("home");
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o700)).expect("mode");
        let allow = AllowList::new(["karl".to_string()]);
        let name = AgentName("karl".into());
        let bound = boundary(&home, 65533);

        // Diagnostic: the kind its binding requires, with the gate
        // instruction its kind excludes removed. No access rule is present,
        // so nothing but the group can refuse it.
        let source = format!(
            "{}binding-kind: diagnostic\n",
            config_source(&sink_dir).replace(
                concat!(
                    "gate-instruction:\n",
                    "  access-rule:\n",
                    "    allowed-uids: [0]\n",
                    "    allowed-gids: []\n",
                    "    denied-uids: [1701]\n"
                ),
                ""
            )
        );

        let refused = take_inventory_against(
            &name,
            &source,
            &allow,
            &bound,
            Some(&ResolvedGroup::UserWithoutGroup),
        );
        assert!(
            matches!(refused, Err(LifecycleRefusal::BoundaryUnverified)),
            "a diagnostic declaration meets the missing group: {refused:?}"
        );

        // And a box that provisioned no agent at all is refused on nothing.
        assert!(
            take_inventory_against(
                &name,
                &source,
                &allow,
                &bound,
                Some(&ResolvedGroup::NoAgent)
            )
            .is_ok(),
            "an unprovisioned box is refused on no fact about an agent"
        );
    }

    /// The wiring: `take_inventory` calls the reachability check and refuses
    /// on what it says.
    ///
    /// **Runs everywhere, which the form it replaced did not.** That form
    /// searched the box for a provisioned agent group, so it skipped on a
    /// developer box with no `weaver-*` group and skipped on CI for the same
    /// reason, while its own record claimed a perturbation was watched. The
    /// group is supplied here instead of found, so the only host fact left is
    /// the caller's uid.
    ///
    /// Perturbation: dropping the `unreachable_peer` block from
    /// `take_inventory_against` leaves this returning `Ok`, watched failing
    /// 2026-08-29 - and now on this box rather than on a hypothetical one.
    ///
    /// conforms: admin-access-rule-reaches-the-socket
    #[test]
    fn a_rule_the_mode_would_defeat_refuses_at_the_inventory() {
        let me = nix::unistd::Uid::current().as_raw();
        if me == 0 {
            eprintln!("SKIP: the check skips root by design");
            return;
        }
        let root = scratch("reach");
        let sink_dir = root.join("sink");
        std::fs::create_dir_all(&sink_dir).expect("sink dir");
        let home = root.join("home");
        std::fs::create_dir_all(&home).expect("home");
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o700)).expect("mode");
        let allow = AllowList::new(["karl".to_string()]);
        let name = AgentName("karl".into());
        let bound = boundary(&home, 65533);

        // A group this caller is not in and cannot be: no members, and a gid
        // no real group carries. The rule then admits a uid the socket's
        // `0770` turns away at `connect`, which is the contradiction.
        let group = ResolvedGroup::Present {
            gid: u32::MAX - 1,
            members: Vec::new(),
        };
        let source = config_source(&sink_dir).replace(
            "    allowed-uids: [0]\n",
            &format!("    allowed-uids: [{me}]\n"),
        );
        let refused = take_inventory_against(&name, &source, &allow, &bound, Some(&group));
        assert!(
            matches!(refused, Err(LifecycleRefusal::BoundaryUnverified)),
            "the unreachable uid refuses as a boundary fault: {refused:?}"
        );

        // And the same walk admits the rule the group does reach, so the
        // refusal is the contradiction and not the walk running at all.
        let reachable = ResolvedGroup::Present {
            gid: u32::MAX - 1,
            members: vec![
                nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(me))
                    .ok()
                    .flatten()
                    .map(|user| user.name)
                    .unwrap_or_default(),
            ],
        };
        assert!(
            take_inventory_against(&name, &source, &allow, &bound, Some(&reachable)).is_ok(),
            "a uid inside the group passes the same walk"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    use std::os::unix::fs::PermissionsExt;

    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "weaver-admin-inv-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    fn boundary(home: &std::path::Path, agent_uid: u32) -> Boundary {
        Boundary {
            agent_uid,
            admin_uid: nix::unistd::getuid().as_raw(),
            // The agent is deliberately NOT in the directory's group: an agent
            // that were a member would be admitted by the group search bit,
            // and correctly so - the boundary the operator draws is exactly
            // that the agent holds no membership reaching this directory.
            agent_gids: vec![65533],
            home: home.to_path_buf(),
            // The fixtures elect no store, which resolves to the embedded
            // engine and so requires a member binary: this process's own
            // stands in for it, present on every box the suite runs on.
            member_binary: Some(std::path::PathBuf::from("/proc/self/exe")),
            store_socket: std::path::PathBuf::from(STORE_SOCKET_DIRECTORY),
        }
    }

    /// **The fixture's rule names uid 0**, which the reachability check
    /// skips for root's `CAP_DAC_OVERRIDE`, so no test drawing this fixture
    /// depends on the host's group table. It named 1000 until 2026-08-28,
    /// which made three boundary tests refuse for the new reason instead of
    /// the one they assert wherever the box carried the agent's group without
    /// that uid in it - the suite reading one box's provisioning as a
    /// property of the code. A test that wants the check judging something
    /// writes its own rule.
    fn config_source(sink_dir: &std::path::Path) -> String {
        format!(
            concat!(
                "session: s-1\n",
                "spu-instruction:\n",
                "  decoder:\n",
                "    model-binding:\n",
                "      artifact: qwen3-4b-instruct\n",
                "      devices: [0]\n",
                "    residual-readout-election: false\n",
                "    identity: []\n",
                "    tunable-values: {{}}\n",
                "tool-set: []\n",
                "permission-mode: ask\n",
                "gate-instruction:\n",
                "  access-rule:\n",
                "    allowed-uids: [0]\n",
                "    allowed-gids: []\n",
                "    denied-uids: [1701]\n",
                "trace-sink:\n",
                "  kind: file\n",
                "  path: {}/trace.ndjson\n",
                "  create: true\n"
            ),
            sink_dir.display()
        )
    }

    /// The fixture with a store election appended, the rest unchanged.
    fn config_source_electing(sink_dir: &std::path::Path, store: &str) -> String {
        format!("{}state-store:\n{store}", config_source(sink_dir))
    }

    /// **The store election's declaration half**, per `weaver-admin-Spec`
    /// section 4 as of 2026-09-04: `none` beside a state election refuses
    /// naming `state-election`, `database` and `role` belong to the service
    /// engine exactly, and each refusal is `ConfigInvalid` naming the field.
    /// Perturbation: remove any one arm of the match and its case below
    /// passes the inventory, each case naming the arm that catches it.
    #[test]
    fn the_store_election_is_judged_before_the_box() {
        let allow = AllowList::new(["alpha".to_string()]);
        let name = AgentName("alpha".into());
        let root = std::env::temp_dir().join(format!("wt-store-decl-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let home = root.join("home");
        std::fs::create_dir_all(&home).expect("home");
        let boundary = boundary(&home, 65533);
        let cases: [(&str, &str); 5] = [
            (
                "  engine: none\nstate-election:\n  all-kinds: true\n  keys: []\n",
                "state-election",
            ),
            ("  engine: none\n  database: d\n", "state-store.database"),
            ("  engine: sqlite\n  role: r\n", "state-store.role"),
            ("  engine: postgres\n  role: r\n", "state-store.database"),
            ("  engine: postgres\n  database: d\n", "state-store.role"),
        ];
        for (store, field) in cases {
            let source = config_source_electing(&home, store);
            let refused = take_inventory(&name, &source, &allow, &boundary);
            match refused {
                Err(LifecycleRefusal::ConfigInvalid { field: Some(ref f) }) if f.0 == field => {}
                other => panic!("{store:?} should refuse naming {field}, got {other:?}"),
            }
        }
        let _ = std::fs::remove_dir_all(&root);
    }

    /// **Every election but `none` requires the member's binary**, and the
    /// service engine requires the store's socket, each `BoundaryUnverified`.
    /// Perturbation: drop the binary check and the first case passes the
    /// inventory or fails later on the sink instead, and drop the socket
    /// check and the second reaches the store's handshake against a
    /// directory holding no socket, failing as an ask rather than a look.
    #[test]
    fn every_election_but_none_requires_the_member_and_postgres_its_socket() {
        let allow = AllowList::new(["alpha".to_string()]);
        let name = AgentName("alpha".into());
        let root = std::env::temp_dir().join(format!("wt-store-box-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let home = root.join("home");
        std::fs::create_dir_all(&home).expect("home");
        let sink_dir = root.join("sink");
        std::fs::create_dir_all(&sink_dir).expect("sink");
        // The sink is admin's, mode-locked, so the walks before the store's
        // pass and the store's own is what refuses.
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o700)).expect("mode");

        let mut without_binary = boundary(&home, 65533);
        without_binary.member_binary = None;
        let embedded = config_source(&sink_dir);
        assert!(
            matches!(
                take_inventory(&name, &embedded, &allow, &without_binary),
                Err(LifecycleRefusal::BoundaryUnverified)
            ),
            "an absent election is the embedded engine and requires the member"
        );
        let declined = config_source_electing(&sink_dir, "  engine: none\n");
        assert!(
            take_inventory(&name, &declined, &allow, &without_binary).is_ok(),
            "none declines the member and requires nothing"
        );

        let mut without_socket = boundary(&home, 65533);
        without_socket.store_socket = root.join("no-such-store");
        let service =
            config_source_electing(&sink_dir, "  engine: postgres\n  database: d\n  role: r\n");
        assert!(
            matches!(
                take_inventory(&name, &service, &allow, &without_socket),
                Err(LifecycleRefusal::BoundaryUnverified)
            ),
            "the service engine requires the store's socket under the configured directory"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// **A declaration whose gate instruction disagrees with its kind is
    /// refused at the inventory, before any look at the filesystem.** The
    /// refusal names the field, so the operator learns which member to move.
    ///
    /// Perturbation: remove the cross-field check from `take_inventory` and
    /// the diagnostic-with-instruction case sails through to a load whose
    /// binding admin cannot construct honestly. Watched under exactly that
    /// removal. The boundary handed in is deliberately broken - a missing
    /// home would refuse later - so a pass through to a boundary refusal is
    /// the perturbation showing.
    #[test]
    fn a_kind_gate_disagreement_refuses_at_the_inventory() {
        let root = scratch("kind");
        let allow = AllowList::new(["alpha".to_string()]);
        let name = AgentName("alpha".to_string());
        let bound = boundary(&root.join("absent-home"), 65533);

        // Diagnostic, carrying the instruction its kind excludes.
        let source = format!("{}binding-kind: diagnostic\n", config_source(&root));
        let refused = take_inventory(&name, &source, &allow, &bound);
        assert!(
            matches!(
                refused,
                Err(LifecycleRefusal::ConfigInvalid { field: Some(ref f) }) if f.0 == "gate-instruction"
            ),
            "the diagnostic instruction refuses naming the field: {refused:?}"
        );

        // Serving, with the instruction its kind requires removed.
        let source = config_source(&root).replace(
            concat!(
                "gate-instruction:\n",
                "  access-rule:\n",
                "    allowed-uids: [0]\n",
                "    allowed-gids: []\n",
                "    denied-uids: [1701]\n"
            ),
            "",
        );
        let refused = take_inventory(&name, &source, &allow, &bound);
        assert!(
            matches!(
                refused,
                Err(LifecycleRefusal::ConfigInvalid { field: Some(ref f) }) if f.0 == "gate-instruction"
            ),
            "the serving omission refuses naming the field: {refused:?}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// **A declaration granting the re-feed permission is refused.** The
    /// member is this crate's to set from the resolved kind at the
    /// construction, per `weaver-admin-Spec` section 7, and it parses with
    /// `default` because one type serves the declaration and the seam, so
    /// this refusal is the declaration's whole guard.
    ///
    /// Perturbation: remove the permission check from the inventory and the
    /// granting declaration loads. Watched under exactly that removal.
    ///
    /// conforms: admin-granted-permission-refused-at-inventory
    #[test]
    fn a_declaration_granting_the_permission_refuses_at_the_inventory() {
        let root = scratch("permission");
        let allow = AllowList::new(["alpha".to_string()]);
        let name = AgentName("alpha".to_string());
        let bound = boundary(&root.join("absent-home"), 65533);

        let source = config_source(&root).replace(
            "    residual-readout-election: false\n",
            "    residual-readout-election: false\n    refeed-permission: true\n",
        );
        assert_ne!(
            source,
            config_source(&root),
            "the grant landed in the source"
        );
        let refused = take_inventory(&name, &source, &allow, &bound);
        assert!(
            matches!(
                refused,
                Err(LifecycleRefusal::ConfigInvalid { field: Some(ref f) })
                    if f.0 == "spu-instruction.decoder.refeed-permission"
            ),
            "the granting declaration refuses naming the field: {refused:?}"
        );
        // The sibling, on the same terms.
        let source = config_source(&root).replace(
            "    residual-readout-election: false\n",
            "    residual-readout-election: false\n    column-permission: true\n",
        );
        let refused = take_inventory(&name, &source, &allow, &bound);
        assert!(
            matches!(
                refused,
                Err(LifecycleRefusal::ConfigInvalid { field: Some(ref f) })
                    if f.0 == "spu-instruction.decoder.column-permission"
            ),
            "the granting declaration refuses naming the sibling: {refused:?}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// **The second walk: the agent reaches the sink by path.** The
    /// containing directory must deny the agent uid the search bit, so the
    /// kernel refuses the lookup before any mode on the file is consulted.
    ///
    /// Perturbation: remove the traversal check from the inventory and the
    /// world-searchable boundary loads. Watched under exactly that removal.
    #[test]
    fn a_traversable_sink_directory_refuses_the_load() {
        let root = scratch("traverse");
        let sink_dir = root.join("sink");
        std::fs::create_dir_all(&sink_dir).expect("sink dir");
        let home = root.join("home");
        std::fs::create_dir_all(&home).expect("home");
        let allow = AllowList::new(["alpha".to_string()]);
        let name = AgentName("alpha".to_string());
        // An agent uid that is nobody here, so ownership is not the route.
        let bound = boundary(&home, 65533);

        // World-searchable: the kernel would let the agent traverse.
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o755)).expect("mode");
        let refused = take_inventory(&name, &config_source(&sink_dir), &allow, &bound);
        assert!(
            matches!(refused, Err(LifecycleRefusal::BoundaryUnverified)),
            "a traversable sink directory refuses, got {refused:?}"
        );

        // Admin-owned and unsearchable by anyone else: the boundary the
        // operator is required to draw.
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o750)).expect("mode");
        let admitted = take_inventory(&name, &config_source(&sink_dir), &allow, &bound);
        assert!(
            admitted.is_ok(),
            "an unsearchable boundary admits: {admitted:?}"
        );
    }

    /// A path this crate cannot resolve yields no boundary evidence, and an
    /// agent that owns the directory can restore the search bit itself.
    ///
    /// Perturbation: read an unresolvable parent as not-traversable, or admit
    /// an agent-owned directory whose mode currently denies search, and both
    /// assertions below fail.
    #[test]
    fn unresolvable_and_agent_owned_directories_are_not_boundaries() {
        let root = scratch("evidence");
        let bound = boundary(&root, nix::unistd::getuid().as_raw());
        assert!(
            agent_can_traverse(std::path::Path::new(""), &bound),
            "a relative sink path's empty parent is no evidence of a boundary"
        );
        // This process owns the scratch directory, so presenting our own uid as
        // the agent's is how the test presents the ownership case.
        // The search bit is cleared first, which is what makes the watch
        // reachable: with it set, a check that also required the bit would
        // pass and the test could not tell the two apart.
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o600))
            .expect("clear the search bit");
        assert!(
            agent_can_traverse(&root, &bound),
            "an agent that owns the directory can restore the search bit itself"
        );
        let _ = std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700));
    }

    /// **The sink boundary has two halves and neither implies the other.** A
    /// directory owned by a third principal at mode 0700 denies the agent its
    /// traversal and still defeats custody, because that owner controls the
    /// file admin opened.
    ///
    /// Perturbation: drop the custody check from the inventory and the
    /// third-party case loads. Watched under exactly that removal.
    #[test]
    fn a_third_party_owned_sink_directory_refuses_the_load() {
        // **Unreachable as root, and skipped rather than asserted anyway.**
        // A directory this test creates is owned by whoever runs it, and the
        // custody check admits root by name, so under root the third-party
        // case cannot be constructed without a second real uid to chown to.
        // Asserting through it would be a watch that reports on the runner
        // rather than on the check.
        if nix::unistd::getuid().is_root() {
            eprintln!(
                "SKIP a_third_party_owned_sink_directory_refuses_the_load: run as root, \
                 so a created directory is root-owned and holds custody by name"
            );
            return;
        }
        let root = scratch("custody");
        let sink_dir = root.join("sink");
        std::fs::create_dir_all(&sink_dir).expect("sink dir");
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o750)).expect("mode");
        let home = root.join("home");
        std::fs::create_dir_all(&home).expect("home");
        let allow = AllowList::new(["alpha".to_string()]);
        let name = AgentName("alpha".to_string());

        // This process owns the directory, so it holds custody and the load
        // is admitted: the denial half already passes at 0750.
        let held = boundary(&home, 65533);
        assert!(
            take_inventory(&name, &config_source(&sink_dir), &allow, &held).is_ok(),
            "an admin-owned directory holds custody"
        );

        // Now present the same directory to a boundary whose admin principal
        // is somebody else. The agent still cannot traverse it - nothing about
        // the directory changed - and custody is now a third party's, which is
        // exactly the case the traversal check alone admits.
        let third_party = Boundary {
            agent_uid: 65533,
            admin_uid: 65532,
            agent_gids: vec![65531],
            home: home.clone(),
            member_binary: Some(std::path::PathBuf::from("/proc/self/exe")),
            store_socket: std::path::PathBuf::from(STORE_SOCKET_DIRECTORY),
        };
        assert!(
            !agent_can_traverse(&sink_dir, &third_party),
            "the denial half still passes, which is what makes this case reachable"
        );
        let refused = take_inventory(&name, &config_source(&sink_dir), &allow, &third_party);
        assert!(
            matches!(refused, Err(LifecycleRefusal::BoundaryUnverified)),
            "a directory a third principal owns refuses the load, got {refused:?}"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The allow-list is consulted before anything else is touched, and the
    /// identity is built from the validated name rather than from any
    /// caller-supplied string.
    #[test]
    fn an_unlisted_name_refuses_before_anything_is_read() {
        let root = scratch("allow");
        let allow = AllowList::new(["alpha".to_string()]);
        let bound = boundary(&root, 65533);
        // The source is not even valid YAML: if the allow-list were consulted
        // second, the parse error would surface instead.
        let refused = take_inventory(&AgentName("beta".into()), "%%%", &allow, &bound);
        assert!(matches!(refused, Err(LifecycleRefusal::NoSuchAgent)));
        assert_eq!(identity_for(&AgentName("alpha".into())), "weaver-alpha");
    }

    /// The inventory repairs nothing: a missing home refuses rather than being
    /// created.
    #[test]
    fn the_inventory_repairs_nothing() {
        let root = scratch("repair");
        let sink_dir = root.join("sink");
        std::fs::create_dir_all(&sink_dir).expect("sink dir");
        std::fs::set_permissions(&sink_dir, std::fs::Permissions::from_mode(0o750)).expect("mode");
        let absent_home = root.join("no-such-home");
        let allow = AllowList::new(["alpha".to_string()]);
        let bound = boundary(&absent_home, 65533);
        let refused = take_inventory(
            &AgentName("alpha".into()),
            &config_source(&sink_dir),
            &allow,
            &bound,
        );
        assert!(matches!(refused, Err(LifecycleRefusal::BoundaryUnverified)));
        assert!(!absent_home.exists(), "nothing was built");
    }
}
