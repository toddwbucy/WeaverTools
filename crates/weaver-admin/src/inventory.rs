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
    LifecycleRefusal, TraceSink,
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
        (BindingKind::Serving, Some(gate_instruction)) => EnterBinding::Serving { gate_instruction },
        (BindingKind::Diagnostic, None) => EnterBinding::Diagnostic,
        _ => {
            return Err(LifecycleRefusal::ConfigInvalid {
                field: Some(FieldName("gate-instruction".into())),
            });
        }
    };

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

    Ok(Inventory {
        config,
        identity,
        binding,
    })
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
        }
    }

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
                "    allowed-uids: [1000]\n",
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
                "    allowed-uids: [1000]\n",
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
