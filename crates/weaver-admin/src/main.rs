//! conforms: admin-no-library-surface
//! conforms: admin-one-floor-link-types-config
//! conforms: admin-no-direct-traits-line
//! conforms: admin-no-runtime-no-bus-no-logging
//! conforms: admin-descriptors-owned-types
//! conforms: admin-runs-as-root-or-performs-nothing
//! conforms: admin-answer-and-exit-status-agree
//! conforms: admin-residency-is-not-lifecycle-state
//! conforms: admin-preload-name-follows-the-kind
//! conforms: admin-cloexec-atomic-at-creation
//! conforms: admin-publishes-only-on-ready
//! conforms: admin-validate-starts-no-process
//! conforms: admin-inventory-one-function
//! conforms: admin-unload-answers-after-confirmed-stop
//! conforms: admin-failed-dial-consults-unit-state
//!
//! `weaver-admin`: the lifecycle tool the admin role runs with root. One
//! binary and no library surface, per `weaver-admin-Spec` section 1 - nothing
//! links admin, and a library target would be an API for a consumer the
//! topology forbids.
//!
//! **One invocation, one verb, then exit.** The operator socket, its
//! accept-time predicate, and the fleet map retired with the service account
//! on 2026-08-05, and what replaces them is the process boundary the operating
//! system already draws around an executed program, per section 2.

mod channel;
mod inventory;
mod log;
mod sink;
mod surface;
mod unit;
mod verbs;

use std::path::PathBuf;

use weaver_types::{AgentName, LifecycleAnswer, LifecycleDirective, LifecycleRefusal};

/// Admin's own operator-installed configuration, per Spec section 9: the
/// coordination socket's per-agent name, the log directory, the unit template,
/// the agent config directory, and the allow-list. **These are deployment
/// facts the operator installs** - not the agent config, crossing no seam, and
/// **none of them discovered at runtime by searching**.
///
/// The operator socket's path left this list with the socket on 2026-08-05.
/// The coordination name stayed and changed hands: the operator places it, the
/// harness binds it, and admin dials it.
struct ServiceConfig {
    coordination_root: PathBuf,
    log_path: PathBuf,
    agent_config_directory: PathBuf,
    unit: unit::UnitTemplate,
    /// The directory the store's unix socket stands in, per Spec section 9
    /// as of 2026-09-04: read under a service election and otherwise idle.
    /// Optional in the file, the engine's conventional directory standing
    /// where the file is silent.
    state_store_socket: PathBuf,
    allow_list: inventory::AllowList,
}

impl ServiceConfig {
    /// The per-agent socket the harness binds inside the unit's runtime
    /// directory. Admin resolves the same name to dial it, which is the one
    /// value that reaches two crates and the reason the operator's file is
    /// where they agree.
    fn coordination_socket(&self, agent: &str) -> PathBuf {
        self.coordination_root
            .join(unit::runtime_directory_name(agent))
            .join("coordination.sock")
    }
}

fn main() {
    // **The store's second question is answered by this binary under the
    // agent's uid**, per `weaver-admin-Spec` section 4 as of 2026-09-04: the
    // parent sets the variable and three arguments, and the child asks the
    // store and exits with the answer. This is not a verb of the surface,
    // and an invocation that carries the variable serves no verb.
    if std::env::var_os(inventory::PROBE_STORE_VARIABLE).is_some() {
        let arguments: Vec<String> = std::env::args().skip(1).collect();
        let admitted = match arguments.as_slice() {
            [socket, database, role] => {
                inventory::store_admits(std::path::Path::new(socket), database, role)
            }
            _ => Err(std::io::Error::other(
                "the probe takes socket, database, role",
            )),
        };
        std::process::exit(match admitted {
            Ok(true) => 0,
            Ok(false) => 1,
            Err(_) => 2,
        });
    }
    let outcome = run();
    // **The answer is one JSON object on standard output and the exit status
    // agrees with it.** Zero exits an answer and a non-zero status exits a
    // refusal, so a shell reads the status and a tool reads the object and the
    // two never disagree.
    match outcome {
        Ok(answer) => {
            println!("{}", surface::render_answer(&answer));
            std::process::exit(0);
        }
        Err(refusal) => {
            println!("{}", surface::render_refusal(&refusal));
            std::process::exit(surface::EXIT_REFUSED);
        }
    }
}

fn run() -> Result<LifecycleAnswer, LifecycleRefusal> {
    // **Authorization is the kernel's, and what this crate checks is the
    // name.** The invocation runs as root or performs nothing: no predicate,
    // no allow set, and no deny set, the operator being the party the kernel
    // already admitted. This refusal is enacted before any verb touches
    // anything.
    if !surface::running_as_root() {
        return Err(LifecycleRefusal::Unauthorized);
    }
    let request = surface::parse_arguments(std::env::args().skip(1))?;
    let config =
        load_service_config().map_err(|_| LifecycleRefusal::ConfigInvalid { field: None })?;
    dispatch(&config, request)
}

fn dispatch(
    config: &ServiceConfig,
    request: surface::Request,
) -> Result<LifecycleAnswer, LifecycleRefusal> {
    match request {
        surface::Request::Validate(agent) => validate(config, &agent),
        surface::Request::Load(agent) => load(config, &agent),
        surface::Request::Unload(agent) => unload(config, &agent),
        surface::Request::Stop(agent) => stop(config, &agent),
        // **`show` and `list` answer through the observation exchange**, per
        // Spec section 3 as of 2026-09-05: the harness's own word where a
        // worker answers the dial, and `Unloaded` from the absence where
        // none does, which is the one place residency is read and it is
        // read as the absence of a worker and not as a state.
        surface::Request::Show(agent) => show(config, &agent),
        surface::Request::List => list(config),
    }
}

/// **`validate` is the load's front half**, and it stops at the report: it
/// touches no seam and starts no process, which is what makes `Validated`
/// mean an outcome rather than a transition.
fn validate(
    config: &ServiceConfig,
    agent: &AgentName,
) -> Result<LifecycleAnswer, LifecycleRefusal> {
    take_inventory(config, agent)?;
    Ok(LifecycleAnswer::Validated)
}

/// **The name is authorized before it reaches the filesystem**, and every verb
/// that builds a path from it calls this.
///
/// `AgentName` wraps a bare string, so a name carrying `/` or `..` would
/// otherwise be interpolated into a config path or a socket path and traverse
/// out of the directory the operator placed. The allow-list is consulted here
/// too, so what a verb may name has one answer rather than one per verb.
fn admissible(config: &ServiceConfig, agent: &AgentName) -> Result<(), LifecycleRefusal> {
    if !config.allow_list.admits(agent)
        || agent.0.is_empty()
        || !agent
            .0
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(LifecycleRefusal::NoSuchAgent);
    }
    Ok(())
}

/// The one inventory, called by both `validate` and `load`, so the two cannot
/// drift.
/// Stands the state member for this load, per `weaver-harness-state-contract`
/// as ruled 2026-08-26 and `weaver-state-Spec` section 2: the custodian's
/// territory sits on the operator's side of the wall the worker's identity
/// cannot cross, so the party that opens the trace sink is the party that
/// stands the member, and that party now creates the first door's transport
/// too. This crate makes the socketpair, arms the member's end onto the fixed
/// number in the spawn path itself, and returns the harness's end for the
/// enter directive to courier, speaking on neither, per `weaver-admin-PRD`
/// section 2. The binary is discovered beside the worker's own, so a
/// deployment without it simply has no leg. Every failure here is absorbed:
/// the leg is optional by presence, a load is never refused over its
/// derivative, and a `None` return is the leg not standing.
fn stand_state_member(
    config: &ServiceConfig,
    inventory: &inventory::Inventory,
) -> Option<std::os::fd::OwnedFd> {
    // `none` declines the member, per `weaver-state-PRD` section 4 as of
    // 2026-09-04: nothing is stood, no territory is made, and the harness's
    // end is absent from the enter by the declaration's own word.
    let store = inventory.config.state_store.clone().unwrap_or_default();
    if store.engine == weaver_types::StoreEngine::None {
        return None;
    }
    let binary_directory = config.unit.worker.parent()?;
    let binary = binary_directory.join("weaver-state");
    if !binary.exists() {
        return None;
    }
    let territory_root = inventory::sink_directory(&inventory.config.trace_sink);
    let territory = territory_root.join("state");
    // The territory's custody mirrors the sink's: this uid owns it, the
    // sink directory's group may look, and the worker's identity holds
    // neither and cannot enter, which is the wall of `weaver-state-PRD`
    // section 4 enforced at the filesystem. The mode rides the creation
    // itself, so the directory never stands a moment wider than it ends.
    use std::os::unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt};
    if std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o750)
        .create(&territory)
        .is_err()
        && !territory.is_dir()
    {
        return None;
    }
    let _ = std::fs::set_permissions(&territory, std::fs::Permissions::from_mode(0o750));
    if let Ok(parent) = std::fs::metadata(territory_root) {
        let _ = std::os::unix::fs::chown(&territory, None, Some(parent.gid()));
    }
    // **The first door is a socketpair this crate creates and speaks on
    // never**, per the operator's ruling of 2026-08-26: both ends
    // close-on-exec atomically at creation like every descriptor this crate
    // holds, and the member's end is re-armed onto the fixed number in the
    // spawn path itself, the one deliberate gift the third walk of
    // `weaver-admin-Spec` section 10 names.
    let (harness_end, member_end) = nix::sys::socket::socketpair(
        nix::sys::socket::AddressFamily::Unix,
        nix::sys::socket::SockType::Stream,
        None,
        nix::sys::socket::SockFlag::SOCK_CLOEXEC,
    )
    .ok()?;
    let log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(territory.join("state.log"));
    let mut member = std::process::Command::new(&binary);
    member
        .args(member_vector(
            &territory,
            &inventory.binding,
            &store,
            &config.state_store_socket,
        ))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null());
    if let Ok(log) = log {
        member.stderr(log);
    } else {
        member.stderr(std::process::Stdio::null());
    }
    // The member's end arrives at the fixed number `weaver-state` adopts.
    // Relocated first where it sits at or below the arming target, because
    // the spawn places the standard streams onto 0 through 2 before any
    // pre-exec closure runs, and the arming itself carries the
    // equal-number corner `weaver-harness-Spec` section 2.2 records,
    // repaired inside `arm_member_end`.
    let member_end = {
        use std::os::fd::AsRawFd;
        if member_end.as_raw_fd() <= 3 {
            match nix::fcntl::fcntl(&member_end, nix::fcntl::FcntlArg::F_DUPFD_CLOEXEC(4)) {
                Ok(raw) => {
                    // SAFETY: F_DUPFD_CLOEXEC answered a fresh descriptor
                    // this process owns, adopted exactly once, and the low
                    // original drops here.
                    unsafe {
                        use std::os::fd::FromRawFd;
                        std::os::fd::OwnedFd::from_raw_fd(raw)
                    }
                }
                Err(_) => return None,
            }
        } else {
            member_end
        }
    };
    let raw_member_end = {
        use std::os::fd::AsRawFd;
        member_end.as_raw_fd()
    };
    unsafe {
        use std::os::unix::process::CommandExt;
        member.pre_exec(move || arm_member_end(raw_member_end));
    }
    let spawned = member.spawn().is_ok();
    // This side's copy of the member's end closes either way: the member
    // holds the armed number, and a spawn that failed leaves no holder, the
    // harness end below then reading as the closed pair it is.
    drop(member_end);
    if !spawned {
        return None;
    }
    Some(harness_end)
}

/// **The arming, the one deliberate gift**, per `weaver-admin-Spec` section
/// 6: `dup2` onto the member's fixed number, then an unconditional
/// close-on-exec clear, because `dup2` onto the same number is a no-op that
/// leaves the flag standing - the corner `weaver-harness-Spec` section 2.2
/// records - and a cleared flag on the armed number alone is what makes the
/// inheritance an act at one site while the atomic flag stands everywhere
/// else. Async-signal-safe throughout, per the pre-exec contract.
fn arm_member_end(raw_member_end: std::os::fd::RawFd) -> std::io::Result<()> {
    if raw_member_end != 3 && unsafe { nix::libc::dup2(raw_member_end, 3) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let flags = unsafe { nix::libc::fcntl(3, nix::libc::F_GETFD) };
    if flags < 0 {
        return Err(std::io::Error::last_os_error());
    }
    if unsafe { nix::libc::fcntl(3, nix::libc::F_SETFD, flags & !nix::libc::FD_CLOEXEC) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

/// **The member's vector, both directions this crate's alone**, per
/// `weaver-admin-Spec` section 6 as ruled 2026-08-26: the territory, and the
/// preload socket path exactly where the resolved kind is diagnostic, derived
/// as the territory with a fixed leaf so no invocation input composes it. One
/// value on a serving load and two on a diagnostic one, the first door riding
/// no argument at all, its end inherited at the fixed number the spawn arms.
fn member_vector(
    territory: &std::path::Path,
    binding: &weaver_types::EnterBinding,
    store: &weaver_types::StateStore,
    store_socket: &std::path::Path,
) -> Vec<std::ffi::OsString> {
    // **The engine leads**, so the member knows which port to stand before
    // it reads a positional, per Spec section 6 as of 2026-09-04, and under
    // the service engine the socket, the database, and the role follow it.
    let mut vector: Vec<std::ffi::OsString> = vec!["--engine".into()];
    vector.push(
        match store.engine {
            weaver_types::StoreEngine::None => "none",
            weaver_types::StoreEngine::Sqlite => "sqlite",
            weaver_types::StoreEngine::Postgres => "postgres",
        }
        .into(),
    );
    if store.engine == weaver_types::StoreEngine::Postgres {
        vector.push("--store-socket".into());
        vector.push(store_socket.as_os_str().to_owned());
        for (flag, value) in [("--database", &store.database), ("--role", &store.role)] {
            if let Some(value) = value {
                vector.push(flag.into());
                vector.push(value.into());
            }
        }
    }
    vector.push(territory.as_os_str().to_owned());
    if matches!(binding, weaver_types::EnterBinding::Diagnostic) {
        vector.push(territory.join("preload.sock").into_os_string());
    }
    vector
}

fn take_inventory(
    config: &ServiceConfig,
    agent: &AgentName,
) -> Result<inventory::Inventory, LifecycleRefusal> {
    admissible(config, agent)?;
    let identity = inventory::identity_for(agent);
    let source_path = config
        .agent_config_directory
        .join(format!("{}.yaml", agent.0));
    let source =
        std::fs::read_to_string(&source_path).map_err(|_| LifecycleRefusal::NoSuchAgent)?;
    // The home comes from the account database rather than from a constructed
    // path: an operator who placed the agent elsewhere would otherwise have
    // the boundary checked against a directory that is not the agent's.
    let user = nix::unistd::User::from_name(&identity)
        .ok()
        .flatten()
        .ok_or(LifecycleRefusal::BoundaryUnverified)?;
    let boundary = inventory::Boundary {
        agent_uid: user.uid.as_raw(),
        // Custody is held by whoever opens the sink, and that is this
        // invocation under root, the role's principal.
        admin_uid: nix::unistd::getuid().as_raw(),
        // **Every gid the worker will actually hold, not the passwd primary
        // alone.** The unit sets `Group={identity}`, so the running egid is
        // `weaver-<agent>` whatever passwd says - and where the operator
        // provisioned a shared primary group, which is the case the mode's
        // own justification cites, the two disagree. A denial walk reading
        // only passwd would pass a sink at `root:weaver-<agent>` mode `0710`
        // that the worker can traverse into and rewrite the record admin
        // holds custody of.
        //
        // Over-approximated on purpose: this asks what the agent could reach,
        // so a gid too many refuses a boundary that might have held, and a
        // gid too few admits one that does not.
        agent_gids: agent_gids(&user),
        home: user.dir.clone(),
        // The two box facts the store rules read, per Spec section 4 as of
        // 2026-09-04: the member's binary beside the worker's, and the
        // store's socket directory from this crate's own file.
        member_binary: config
            .unit
            .worker
            .parent()
            .map(|directory| directory.join("weaver-state"))
            // A binary and not a path: a directory or an unexecutable file
            // under that name would pass an existence look and fail at the
            // spawn, the leg then down under a declaration that never
            // declined it, which is the #381 class this rule exists to
            // refuse at the inventory.
            .filter(|binary| {
                use std::os::unix::fs::PermissionsExt;
                std::fs::metadata(binary)
                    .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                    .unwrap_or(false)
            }),
        store_socket: config.state_store_socket.clone(),
    };
    inventory::take_inventory(agent, &source, &config.allow_list, &boundary)
}

/// Every gid the worker may run under: the group the unit sets, the passwd
/// primary, and the supplementary memberships the user holds.
///
/// `unreachable_peer` reads `Group::from_name(identity)` for the same
/// boundary, and the denial walk follows it rather than reading passwd alone.
///
/// conforms: admin-boundary-reads-every-gid-the-worker-holds
fn agent_gids(user: &nix::unistd::User) -> Vec<u32> {
    // The gid `--property=Group={identity}` gives the unit, which is what the
    // worker's egid actually is.
    let named = match nix::unistd::Group::from_name(&user.name) {
        Ok(Some(group)) => Some(group.gid.as_raw()),
        // A lookup that failed leaves the primary and the supplementary set,
        // which is the narrower answer and so the one that refuses more.
        _ => None,
    };
    // And whatever else the user is a member of, a supplementary group being
    // reachable by the running process too.
    let supplementary: Vec<u32> = std::ffi::CString::new(user.name.as_str())
        .ok()
        .and_then(|name| nix::unistd::getgrouplist(&name, user.gid).ok())
        .map(|groups| groups.into_iter().map(|gid| gid.as_raw()).collect())
        .unwrap_or_default();
    merge_gids(user.gid.as_raw(), named, &supplementary)
}

/// The three sources joined, deduped, with the passwd primary first.
///
/// **Separate from the host lookups so the join is watched.** The failure the
/// denial walk cannot survive is a gid dropped on the floor, and that is a
/// property of this merge rather than of `getgrnam_r`. Reading the host
/// inside the same function would have made any watch on it vacuous wherever
/// no agent is provisioned, which is this box and CI both.
fn merge_gids(primary: u32, named: Option<u32>, supplementary: &[u32]) -> Vec<u32> {
    let mut gids = vec![primary];
    for gid in named.into_iter().chain(supplementary.iter().copied()) {
        if !gids.contains(&gid) {
            gids.push(gid);
        }
    }
    gids
}

#[cfg(test)]
mod gid_tests {
    use super::merge_gids;

    /// **The group the unit sets is in the set, and the passwd primary alone
    /// is not the set.**
    ///
    /// This is the custody hole olympus found on 2026-08-29: the unit forces
    /// `Group={identity}` so the worker's egid is `weaver-<agent>`, while the
    /// denial walk read the passwd primary. Where the operator provisioned a
    /// shared primary - `users`, `nogroup` - the two disagree, and a sink at
    /// `root:weaver-karl` mode `0710` passes a walk the running worker can
    /// then traverse to rewrite the trace admin holds custody of.
    ///
    /// Perturbation: returning `vec![primary]` fails this. Watched failing
    /// 2026-08-29.
    ///
    /// conforms: admin-boundary-reads-every-gid-the-worker-holds
    #[test]
    fn the_walk_reads_the_group_the_unit_sets_and_not_passwd_alone() {
        // The shared-primary case the mode's own justification cites.
        let gids = merge_gids(100, Some(2001), &[100]);
        assert!(
            gids.contains(&2001),
            "the egid the unit sets is walked: {gids:?}"
        );
        assert!(gids.contains(&100), "and the passwd primary too: {gids:?}");

        // Supplementary memberships reach the sink as well.
        let gids = merge_gids(100, Some(2001), &[100, 27, 998]);
        assert!(
            [100, 2001, 27, 998].iter().all(|gid| gids.contains(gid)),
            "every gid the worker holds is walked: {gids:?}"
        );

        // Deduped, and the primary stays first: the walk asks `contains`, so
        // a repeat is only noise, but a set that grows per call is a leak.
        assert_eq!(merge_gids(100, Some(100), &[100, 100]), vec![100]);
        assert_eq!(merge_gids(100, None, &[7]), vec![100, 7]);
    }
}

/// `load` runs the charter's seven steps in order, and answers only on a ready
/// aggregate: every other outcome enters the rollback carrying the step's name
/// and answers nothing.
fn load(config: &ServiceConfig, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
    let mut operations = log::OperationsLog::open(&config.log_path)
        .map_err(|_| LifecycleRefusal::BoundaryUnverified)?;
    let mut standing = verbs::Standing::default();

    let outcome = run_load(config, agent, &mut standing);
    match outcome {
        Ok(()) => {
            let _ = operations.record(&log::Act {
                verb: "load",
                agent: agent.0.clone(),
                outcome: "ready".into(),
                undone: None,
            });
            // Idle is honest here and only here: the enter aggregate came back
            // ready, which is the interior serving and at rest. It is not read
            // from the unit, which could not tell idle from active.
            Ok(LifecycleAnswer::State {
                state: weaver_types::AgentState::Idle,
                load: None,
            })
        }
        Err(refusal) => {
            // **Rollback walks what stands**, and its account is logged.
            let account = verbs::rollback(
                &standing,
                // **The leave is directed where a run was entered**, per
                // charter section 5 and Spec section 3. A refused fan-out
                // leaves the harness holding a partial run it will unwind
                // along the seams it fanned out on, and the directive is what
                // asks it to. The account reports whether the ask landed, so
                // a leave that could not be sent is recorded as held rather
                // than claimed.
                || direct_leave(config, agent).is_ok(),
                || unit::stop(&config.unit, &agent.0).is_ok(),
                || true,
            );
            for undone in &account {
                let _ = operations.record(&log::Act {
                    verb: "rollback",
                    agent: agent.0.clone(),
                    outcome: format!("{refusal:?}"),
                    undone: Some(format!(
                        "{}:{}",
                        undone.act,
                        if undone.succeeded { "undone" } else { "held" }
                    )),
                });
            }
            Err(refusal)
        }
    }
}

fn run_load(
    config: &ServiceConfig,
    agent: &AgentName,
    standing: &mut verbs::Standing,
) -> Result<(), LifecycleRefusal> {
    let inventory = take_inventory(config, agent)?;
    let sink = sink::open(&inventory.config.trace_sink)?;
    standing.sink_opened = true;

    // The unit starts bare and binds its own coordination socket as its first
    // act, so nothing is placed into it here.
    // **A start ask that returns non-zero started nothing**, and the status is
    // what carries that. Unit-name uniqueness is the concurrency guard
    // `weaver-admin-systemd-contract` section 5 relies on, so a second load of
    // a live agent fails here, and a discarded status would let it proceed to
    // dial the first load's worker.
    // The socket path is derived here from the same validated name the unit's
    // own name carries, and the worker binds what it is told, so the two
    // agree by construction rather than by two readings of one convention.
    let socket_path = config.coordination_socket(&agent.0);
    started(unit::start(
        &config.unit,
        &inventory.identity,
        &agent.0,
        &socket_path,
        inventory.config.loop_file.as_deref(),
    ))
    .map_err(|from_status| refusal_for_failed_start(config, &agent.0, from_status))?;
    standing.unit_started = true;

    // **The dial races the worker's bind and the bound covers it.** A bound
    // exceeded is not an absent residency, so the refusal consults the unit's
    // state before returning and carries what the manager said. The path is
    // the one the start ask carried, so admin dials exactly what it told the
    // worker to bind.
    let mut coordination = match channel::dial(&socket_path) {
        Ok(coordination) => coordination,
        Err(_) => return Err(refusal_for_absent_worker(config, &agent.0)),
    };

    // The state member stands after the worker's bind and before the enter,
    // and nothing is waited on: the pair exists before the member does, per
    // the ruling of 2026-08-26. Best-effort whole, per the contract's
    // dead-peer clause - an absent binary or a failed spawn leaves the leg
    // down and the load unrefused, the harness's end below then absent from
    // the enter and the directive carrying the sink alone.
    let state_end = stand_state_member(config, &inventory);

    let ordinal = coordination.next_ordinal();
    // **The session is read and the run is minted**, per Spec section 7. The
    // session is the operator's, declared in the config and carried
    // uninterpreted. The reference is this crate's, and a load that cannot
    // read the randomness it rests on refuses rather than carrying a
    // reference that looks like the others and is not guaranteed.
    let run_reference =
        channel::mint_run_reference(&agent.0).ok_or(LifecycleRefusal::BoundaryUnverified)?;
    let envelope = weaver_types::OrganEnvelope {
        exchange: weaver_types::ExchangeId {
            opener: weaver_types::Opener::Admin,
            ordinal,
        },
        position: weaver_types::Position::Open,
        payload: weaver_types::Payload::Directive(LifecycleDirective::Enter {
            payload: weaver_types::EnterPayload {
                session: inventory.config.session.clone(),
                run: run_reference,
                // The permission member is written from the resolved kind,
                // per `weaver-admin-Spec` section 7: granted under a
                // diagnostic enter and cleared under a serving one, never
                // read from the file, whose grant the inventory refused.
                spu_instruction: {
                    let mut instruction = inventory.config.spu_instruction.clone();
                    let diagnostic =
                        matches!(inventory.binding, weaver_types::EnterBinding::Diagnostic);
                    instruction.decoder.refeed_permission = diagnostic;
                    instruction.decoder.column_permission = diagnostic;
                    instruction
                },
                // The resolution happened at the inventory, the one site, so
                // what crosses is the kind decided with what it requires
                // riding inside it.
                binding: inventory.binding.clone(),
                // The resolution site the contract names: an absent
                // declaration becomes the ruled default here, so what
                // crosses is always the election whole and the worker
                // never re-derives an absence.
                state_election: inventory.config.state_election.clone().unwrap_or_default(),
                // The same resolution for the store, per `weaver-types-Spec`
                // section 4 as of 2026-09-04: an absent election is the
                // embedded engine, and the record names what was resolved.
                state_store: inventory.config.state_store.clone().unwrap_or_default(),
                declaration: inventory.declaration.clone(),
            },
        }),
    };
    use std::os::fd::AsFd;
    coordination
        .send_with_sink(
            &envelope,
            sink.as_fd(),
            state_end.as_ref().map(|end| end.as_fd()),
        )
        .map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;
    standing.entered = true;

    match coordination.recv() {
        Ok(answer) => match answer.payload {
            weaver_types::Payload::Answer(LifecycleAnswer::Ready) => Ok(()),
            weaver_types::Payload::Refusal(refusal) => Err(refusal),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }
    // The connection closes here with the verb: a per-invocation admin holds
    // no standing end.
}

/// Whether a start ask started anything.
///
/// **A start ask that returns non-zero started nothing**, and the status is
/// what carries that. Unit-name uniqueness is the concurrency guard
/// `weaver-admin-systemd-contract` section 5 relies on, so a second load of a
/// live agent fails here, and a discarded status would let that load proceed
/// to dial the first load's worker and direct a second enter at it.
///
/// It is a function rather than a match inside `run_load` so that the decision
/// is reachable by a test: discarding the status compiles, so nothing but a
/// watch catches it.
fn started(asked: std::io::Result<std::process::ExitStatus>) -> Result<(), LifecycleRefusal> {
    match asked {
        Ok(status) if status.success() => Ok(()),
        _ => Err(LifecycleRefusal::BindFailed),
    }
}

/// **A failed start ask is asked about rather than guessed at.**
///
/// The status cannot say which failure it was: `weaver-admin-systemd-contract`
/// section 3 measures a duplicate unit name and a malformed property failing
/// with the same status, differing only in prose this crate does not read. So
/// the refusal comes from the state ask, where `failed` covers one condition,
/// and every other reading keeps the answer the status alone could give.
///
/// It is a function rather than a branch inside `run_load` for the reason
/// `started` is: the decision is reachable by a test.
fn refusal_for_failed_start(
    config: &ServiceConfig,
    agent: &str,
    from_status: LifecycleRefusal,
) -> LifecycleRefusal {
    start_refusal_for_residency(unit::residency(&config.unit, agent), from_status)
}

/// The mapping alone, testable without a manager.
fn start_refusal_for_residency(
    residency: unit::Residency,
    from_status: LifecycleRefusal,
) -> LifecycleRefusal {
    match residency {
        unit::Residency::Failed => LifecycleRefusal::PriorUnitUnreaped,
        _ => from_status,
    }
}

/// **A failed dial is followed by a state ask, so a refusal names the right
/// thing.** A start ask can succeed over a unit that never runs, so the dial's
/// bound is what proves liveness, and the bound alone would report an absent
/// residency where the truth is a unit that is not running.
///
/// What the ask yields is a state and never a reason. `Failed` means one
/// thing; `Inactive` covers a unit that stopped cleanly, one that never
/// existed, and one whose exec never succeeded, so nothing is claimed beyond
/// the value.
/// **Observe one agent**, per `weaver-admin-harness-contract` section 3 as
/// of 2026-09-05: dial the coordination socket and open the exchange, and
/// carry the harness's answer whole. No socket, or no worker answering the
/// dial, is `Unloaded` with no load, read from the absence and never from
/// the unit.
fn observe(
    config: &ServiceConfig,
    agent: &AgentName,
) -> Result<(weaver_types::AgentState, Option<weaver_types::LoadFacts>), LifecycleRefusal> {
    let socket_path = config.coordination_socket(&agent.0);
    let Ok(mut coordination) = channel::dial(&socket_path) else {
        return Ok((weaver_types::AgentState::Unloaded, None));
    };
    let ordinal = coordination.next_ordinal();
    if coordination
        .send_directive(ordinal, LifecycleDirective::Observe)
        .is_err()
    {
        return Ok((weaver_types::AgentState::Unloaded, None));
    }
    match coordination.recv() {
        Ok(answer) => match answer.payload {
            weaver_types::Payload::Answer(LifecycleAnswer::State { state, load }) => {
                Ok((state, load))
            }
            weaver_types::Payload::Refusal(refusal) => Err(refusal),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Ok((weaver_types::AgentState::Unloaded, None)),
    }
}

fn show(config: &ServiceConfig, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
    admissible(config, agent)?;
    let (state, load) = observe(config, agent)?;
    Ok(LifecycleAnswer::State { state, load })
}

fn list(config: &ServiceConfig) -> Result<LifecycleAnswer, LifecycleRefusal> {
    let mut agents = Vec::new();
    for name in config.allow_list.names() {
        let agent = AgentName(name.clone());
        // The allow-list is the operator's file and its entries are judged
        // as every verb's argument is, so a malformed name never composes a
        // socket path outside the coordination root.
        admissible(config, &agent)?;
        let (state, load) = observe(config, &agent)?;
        agents.push(weaver_types::AgentSummary {
            name: agent,
            state,
            load,
        });
    }
    Ok(LifecycleAnswer::Agents { agents })
}

fn refusal_for_absent_worker(config: &ServiceConfig, agent: &str) -> LifecycleRefusal {
    refusal_for_residency(unit::residency(&config.unit, agent))
}

/// The mapping alone, separated from the ask for the reason `unload_answer` is:
/// a decision reachable by a test rather than only by a live manager.
fn refusal_for_residency(residency: unit::Residency) -> LifecycleRefusal {
    match residency {
        // A prior process exited non-zero and its name is still held, which
        // is what refuses a later start under it. Not a bind failure: the
        // state says a process exited non-zero and says nothing about whether
        // it bound, so naming a socket here would assert what the boundary
        // did not, per `weaver-admin-systemd-contract` section 3.
        unit::Residency::Failed => LifecycleRefusal::PriorUnitUnreaped,
        // The unit is running and its socket was not reachable, so what failed
        // is the bind rather than the residency. Reporting no residency here
        // would name the one thing the manager just said was present.
        unit::Residency::Active => LifecycleRefusal::BindFailed,
        unit::Residency::Inactive | unit::Residency::Unknown => LifecycleRefusal::NoResidency,
    }
}

/// `unload` directs leave, stops the unit, and answers only once the stop has
/// been confirmed.
fn unload(config: &ServiceConfig, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
    take_inventory(config, agent).map(|_| ()).or_else(|e| {
        // A name that does not resolve cannot be unloaded, but a config that
        // has since gone bad must not strand a running worker, so only the
        // name's own refusal stops this verb.
        if matches!(e, LifecycleRefusal::NoSuchAgent) {
            Err(e)
        } else {
            Ok(())
        }
    })?;

    let socket_path = config.coordination_socket(&agent.0);
    let mut coordination =
        channel::dial(&socket_path).map_err(|_| LifecycleRefusal::NoResidency)?;
    let ordinal = coordination.next_ordinal();
    coordination
        .send_directive(ordinal, LifecycleDirective::Leave)
        .map_err(|_| LifecycleRefusal::NoResidency)?;
    match coordination.recv() {
        Ok(answer) => match answer.payload {
            weaver_types::Payload::Answer(LifecycleAnswer::Left) => Ok(()),
            weaver_types::Payload::Refusal(refusal) => Err(refusal),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }?;
    drop(coordination);

    // **A stop that is accepted is not a stop that has happened**, and this
    // verb waits for the difference. An agent reported unloaded while its
    // worker still runs is the one report this verb must never produce.
    let stopped = unit::stop(&config.unit, &agent.0);
    let mut operations = log::OperationsLog::open(&config.log_path)
        .map_err(|_| LifecycleRefusal::BoundaryUnverified)?;
    let residency = unit::residency(&config.unit, &agent.0);
    let _ = operations.record(&log::Act {
        verb: "unload",
        agent: agent.0.clone(),
        outcome: residency.as_str().into(),
        undone: None,
    });
    // **The state ask decides and the stop ask's status does not**, per Spec
    // section 6. That status returns the same value for a unit that is still
    // there as for one already gone, and the second is the ordinary end of a
    // clean unload: the leave confirmed above causes the worker to exit and a
    // transient unit is collected the moment its main process does, so the
    // stop that follows names a unit the manager no longer knows. The status
    // is still read, because the log records what the ask returned.
    let _ = stopped;
    unload_answer(residency)
}

/// The unload's answer from the state ask alone, split out so the rule is
/// reachable by a test rather than reachable only by unloading a live agent.
///
/// **Two of the four cases refuse and the reason is one rule**: an agent
/// reported unloaded while its worker still runs is the report this verb must
/// never produce, per `weaver-admin-Spec` section 6. `Active` is that case
/// directly. `Unknown` is the ask itself having failed, so the verb cannot
/// tell, and answering unloaded over a state nobody could read would risk the
/// same report by another route. The Spec's clause names `active` because that
/// is the case it was written against, and the rule behind it is what carries
/// `Unknown` here.
///
/// `Inactive` and `Failed` both mean the worker is not running, which is the
/// outcome charter step 2 asks for. A unit that failed is reported by the log,
/// which carries the residency's own name, rather than by refusing an unload
/// that achieved what it directed.
fn unload_answer(residency: unit::Residency) -> Result<LifecycleAnswer, LifecycleRefusal> {
    match residency {
        unit::Residency::Active => Err(LifecycleRefusal::ActivityNotAtRest),
        unit::Residency::Unknown => Err(LifecycleRefusal::BindFailed),
        unit::Residency::Inactive | unit::Residency::Failed => Ok(LifecycleAnswer::State {
            state: weaver_types::AgentState::Unloaded,
            load: None,
        }),
    }
}

/// Direct one leave over a fresh dial, for the rollback path.
///
/// **Admin is per-invocation, so the connection the load used is gone by the
/// time the rollback runs.** Dialing again is what every verb does, and the
/// worker is still there to answer: a refused fan-out leaves the harness
/// holding the partial run rather than exiting, which is what makes the
/// directive reach something.
///
/// A dial that fails is not an error to report upward. It means the worker is
/// already gone, so the run it held is gone with it, and the rollback's
/// account records the leave as held rather than done. That is the truthful
/// reading, and the act after this one stops the unit either way.
fn direct_leave(config: &ServiceConfig, agent: &AgentName) -> Result<(), LifecycleRefusal> {
    let socket_path = config.coordination_socket(&agent.0);
    let mut coordination =
        channel::dial(&socket_path).map_err(|_| LifecycleRefusal::NoResidency)?;
    let ordinal = coordination.next_ordinal();
    coordination
        .send_directive(ordinal, LifecycleDirective::Leave)
        .map_err(|_| LifecycleRefusal::NoResidency)?;
    match coordination.recv() {
        Ok(answer) => match answer.payload {
            weaver_types::Payload::Answer(LifecycleAnswer::Left) => Ok(()),
            weaver_types::Payload::Refusal(refusal) => Err(refusal),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }
}

/// `stop` is a conveyance and its answer is a relay. Admin holds no opinion
/// about which fate the harness reports: authorizing a stop and deciding what
/// a stop found are different acts, and the second is the harness's.
fn stop(config: &ServiceConfig, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
    // This verb reaches no inventory, so the name check is called directly:
    // a path is built from the name below, and an unchecked name would
    // traverse out of the runtime root.
    admissible(config, agent)?;
    let socket_path = config.coordination_socket(&agent.0);
    let mut coordination =
        channel::dial(&socket_path).map_err(|_| LifecycleRefusal::NoResidency)?;
    let ordinal = coordination.next_ordinal();
    coordination
        .send_directive(ordinal, LifecycleDirective::Stop)
        .map_err(|_| LifecycleRefusal::NoResidency)?;
    match coordination.recv() {
        // The relay is a function rather than a bare return so that
        // "unchanged" is a property one place holds and one test reads.
        Ok(answer) => match answer.payload {
            weaver_types::Payload::Answer(answer) => Ok(verbs::relay_stop_answer(answer)),
            weaver_types::Payload::Refusal(refusal) => Err(refusal),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }
}

/// The service configuration is the operator's to place. Its shape is section
/// 11's open election, so this reads the minimum the modules need and refuses
/// rather than searching for a default.
fn load_service_config() -> Result<ServiceConfig, String> {
    let root = std::env::var("WEAVER_ADMIN_CONFIG")
        .map_err(|_| "WEAVER_ADMIN_CONFIG names the service configuration and is unset")?;
    let root = PathBuf::from(root);
    let read = |name: &str| -> Result<String, String> {
        std::fs::read_to_string(root.join(name))
            .map(|s| s.trim().to_string())
            .map_err(|_| format!("the service configuration has no {name}"))
    };
    let allow = read("allow-list")?;
    // An interior blank line would otherwise provision an agent named the
    // empty string, which every downstream check would then have to refuse.
    let provisioned: Vec<String> = allow
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Ok(ServiceConfig {
        coordination_root: PathBuf::from(read("coordination-root")?),
        log_path: PathBuf::from(read("log-path")?),
        agent_config_directory: PathBuf::from(read("agent-config-directory")?),
        unit: unit::UnitTemplate {
            run_tool: read("run-tool")?,
            control_tool: read("control-tool")?,
            properties: read("unit-properties")
                .unwrap_or_default()
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect(),
            worker: PathBuf::from(read("worker-binary")?),
            // Required rather than defaulted, on the same ground as every
            // other value here: a missing one refuses and names itself
            // rather than being searched for, per Spec section 9.
            spu: PathBuf::from(read("spu-binary")?),
            gate: PathBuf::from(read("gate-binary")?),
            // Optional, unlike the binaries above: an installation that states
            // no headroom leaves the organ's compiled default standing.
            headroom_bytes: read("headroom-bytes").ok().filter(|v| !v.is_empty()),
        },
        state_store_socket: read("state-store-socket")
            .ok()
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(inventory::STORE_SOCKET_DIRECTORY)),
        allow_list: inventory::AllowList::new(provisioned),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spawn `readlink` over the named child descriptors with the given
    /// pre-exec arming, and return what each resolved to - the socket's
    /// inode identity, or an empty string where the number held nothing.
    /// Identity rather than existence, because a child's own bookkeeping
    /// can land a descriptor on a number this test is watching.
    fn probe_child_fds<F>(arming: F, numbers: &[i32]) -> Vec<String>
    where
        F: FnMut() -> std::io::Result<()> + Send + Sync + 'static,
    {
        let mut probe = std::process::Command::new("/usr/bin/readlink");
        for number in numbers {
            probe.arg(format!("/proc/self/fd/{number}"));
        }
        probe
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null());
        unsafe {
            use std::os::unix::process::CommandExt;
            probe.pre_exec(arming);
        }
        let held = probe.output().expect("the probe runs");
        std::str::from_utf8(&held.stdout)
            .expect("fd targets are ascii")
            .lines()
            .map(str::to_string)
            .collect()
    }

    /// This process's own reading of a descriptor's identity, the string
    /// the child's probe must match for the same open file description.
    fn own_fd_identity(raw: i32) -> String {
        std::fs::read_link(format!("/proc/self/fd/{raw}"))
            .expect("own descriptor resolves")
            .to_string_lossy()
            .into_owned()
    }

    /// **The third walk's enumerate half, on the real arming path**, per
    /// `weaver-admin-Spec` section 10 as amended 2026-08-26: spawn a
    /// subprocess with the member's spawn arming, enumerate its descriptors,
    /// and confirm none of this process's crossed but the one the spawn
    /// deliberately arms. Two layouts, because the arming has two paths:
    /// the ordinary one, the end above the target and `dup2` moving it, and
    /// the equal-number corner, the end already at three with close-on-exec
    /// set, where `dup2` is a no-op and the unconditional flag clear is the
    /// whole repair, per `weaver-harness-Spec` section 2.2.
    ///
    /// Perturbation: remove the flag clear from `arm_member_end` and the
    /// corner layout fails, the armed number closing at exec. Remove the
    /// `dup2` and the ordinary layout fails.
    #[test]
    fn the_spawn_arms_the_gift_and_nothing_else_crosses() {
        // Layout one, ordinary: the end relocated above the child's own
        // low numbers, so the listing below cannot alias it.
        let (harness_end, member_end) = nix::sys::socket::socketpair(
            nix::sys::socket::AddressFamily::Unix,
            nix::sys::socket::SockType::Stream,
            None,
            nix::sys::socket::SockFlag::SOCK_CLOEXEC,
        )
        .expect("the pair");
        let relocate = |end: std::os::fd::OwnedFd| -> std::os::fd::OwnedFd {
            let raw = nix::fcntl::fcntl(&end, nix::fcntl::FcntlArg::F_DUPFD_CLOEXEC(16))
                .expect("relocates");
            // SAFETY: a fresh descriptor this process owns, adopted once,
            // the low original dropping with `end`.
            unsafe {
                use std::os::fd::FromRawFd;
                std::os::fd::OwnedFd::from_raw_fd(raw)
            }
        };
        let harness_end = relocate(harness_end);
        let member_end = relocate(member_end);
        let harness_raw = {
            use std::os::fd::AsRawFd;
            harness_end.as_raw_fd()
        };
        let member_raw = {
            use std::os::fd::AsRawFd;
            member_end.as_raw_fd()
        };
        let member_identity = own_fd_identity(member_raw);
        let listed = probe_child_fds(move || arm_member_end(member_raw), &[3, harness_raw]);
        assert_eq!(
            listed.first().map(String::as_str),
            Some(member_identity.as_str()),
            "the gift stands at the fixed number as the member's own end: {listed:?}"
        );
        assert_eq!(
            listed.len(),
            1,
            "this process's other end did not cross: {listed:?}"
        );
        drop(harness_end);
        drop(member_end);

        // Layout two, the corner: the end seated at the target with the
        // flag set before the arming runs, so `dup2` is a no-op and only
        // the unconditional clear keeps the gift alive across exec.
        let (own_end, corner_end) = nix::sys::socket::socketpair(
            nix::sys::socket::AddressFamily::Unix,
            nix::sys::socket::SockType::Stream,
            None,
            nix::sys::socket::SockFlag::SOCK_CLOEXEC,
        )
        .expect("the corner pair");
        let corner_raw = {
            use std::os::fd::AsRawFd;
            corner_end.as_raw_fd()
        };
        let corner_identity = own_fd_identity(corner_raw);
        let listed = probe_child_fds(
            move || {
                // Seat the end at three with close-on-exec set, which is
                // the layout an unlucky descriptor table hands the real
                // spawn.
                if unsafe { nix::libc::dup2(corner_raw, 3) } < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if unsafe { nix::libc::fcntl(3, nix::libc::F_SETFD, nix::libc::FD_CLOEXEC) } < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                arm_member_end(3)
            },
            &[3],
        );
        assert_eq!(
            listed.first().map(String::as_str),
            Some(corner_identity.as_str()),
            "the corner's gift survives exec on the flag clear alone: {listed:?}"
        );
        drop(own_end);
        drop(corner_end);
    }

    /// **The vector this crate composes, in both directions**, per
    /// `weaver-admin-Spec` section 6 as ruled 2026-08-26: a serving
    /// inventory puts one value on it and a diagnostic inventory puts two,
    /// the preload path the territory with a fixed leaf. Two watches rather
    /// than one, because one would not fail on both directions.
    ///
    /// Perturbation: make the arm that appends the preload path
    /// unconditional and the serving half fails on a load carrying two.
    /// Remove the arm and the diagnostic half fails on a load carrying one.
    #[test]
    fn the_vector_follows_the_kind_in_both_directions() {
        let territory = std::path::Path::new("/dbpool/agents/alpha/state");
        let embedded = weaver_types::StateStore::default();
        let socket = std::path::Path::new("/run/postgresql");
        let serving = member_vector(
            territory,
            &weaver_types::EnterBinding::Serving {
                gate_instruction: weaver_types::GateInstruction {
                    access_rule: weaver_types::AccessRule {
                        allowed_uids: Default::default(),
                        allowed_gids: Default::default(),
                        denied_uids: Default::default(),
                    },
                },
            },
            &embedded,
            socket,
        );
        assert_eq!(
            serving.len(),
            3,
            "a serving load carries the engine pair and the territory alone"
        );
        assert_eq!(serving[0], "--engine");
        assert_eq!(serving[1], "sqlite");
        assert_eq!(serving[2], territory.as_os_str());
        let diagnostic = member_vector(
            territory,
            &weaver_types::EnterBinding::Diagnostic,
            &embedded,
            socket,
        );
        assert_eq!(
            diagnostic.len(),
            4,
            "a diagnostic load carries the preload path"
        );
        assert_eq!(diagnostic[2], territory.as_os_str());
        assert_eq!(
            diagnostic[3],
            territory.join("preload.sock").into_os_string(),
            "the territory with the fixed leaf, no invocation input composing it"
        );
    }

    /// A configuration whose values are never read by the arm under test.
    /// `dispatch`'s observation arm touches no field, which is what lets this
    /// stand without a filesystem.
    fn unread_config() -> ServiceConfig {
        ServiceConfig {
            state_store_socket: PathBuf::from(inventory::STORE_SOCKET_DIRECTORY),
            coordination_root: PathBuf::from("/nonexistent"),
            log_path: PathBuf::from("/nonexistent/log"),
            agent_config_directory: PathBuf::from("/nonexistent/agents"),
            unit: unit::UnitTemplate {
                run_tool: "/bin/false".into(),
                control_tool: "/bin/false".into(),
                properties: vec![],
                worker: PathBuf::from("/bin/false"),
                spu: PathBuf::from("/bin/false"),
                gate: PathBuf::from("/bin/false"),
                headroom_bytes: None,
            },
            allow_list: inventory::AllowList::new(["alpha".to_string()]),
        }
    }

    /// **`show` and `list` construct no `AgentState`.** The party that knows an
    /// agent's lifecycle state is the harness and no chartered exchange asks
    /// it, so these verbs refuse rather than return a value read from
    /// residency, which is a different fact.
    ///
    /// This tests `dispatch` rather than the binary because the root guard
    /// answers first in an unprivileged suite, so the binary can never reach
    /// this arm. A test that ran the binary here would pass under the very
    /// invention it claims to forbid, which is the never-failing perturbation
    /// apex section 11 counts as worse than no test. That is not hypothetical:
    /// the binary-level form of this test was written first and passed under
    /// the substitution, which is how the flaw was found.
    ///
    /// Perturbation: make the `Show` arm answer with a state read from
    /// `unit::residency` and this test fails. Watched under exactly that
    /// substitution.
    /// **Every verb that builds a path from a name checks the name first.**
    /// `stop` reaches no inventory, so it calls the shared check directly: a
    /// name carrying a separator or `..` would otherwise be interpolated into
    /// the runtime path and traverse out of the root the operator placed.
    ///
    /// Perturbation: remove the `admissible` call from `stop` and the
    /// traversal names below reach `coordination_socket`. Watched under
    /// exactly that removal.
    /// **A start ask that returns non-zero started nothing.** The manager
    /// reports a duplicate unit name that way, which is the concurrency guard
    /// this crate leans on, so a discarded status would let a second load of a
    /// live agent dial the first load's worker.
    ///
    /// Perturbation: replace `started` with a discard and this test fails.
    /// Watched under exactly that removal, which compiles, which is why the
    /// decision is a function rather than a match.
    #[test]
    fn a_non_success_start_starts_nothing() {
        use std::os::unix::process::ExitStatusExt;
        let ok = std::process::ExitStatus::from_raw(0);
        assert_eq!(started(Ok(ok)), Ok(()));
        // The manager's own answer for a duplicate unit name is a non-zero
        // status, measured 2026-08-05.
        let refused = std::process::ExitStatus::from_raw(1 << 8);
        assert_eq!(started(Ok(refused)), Err(LifecycleRefusal::BindFailed));
        assert_eq!(
            started(Err(std::io::Error::other("no such tool"))),
            Err(LifecycleRefusal::BindFailed)
        );
    }

    #[test]
    fn stop_checks_the_name_before_building_a_path() {
        let config = unread_config();
        for hostile in ["../../etc", "alpha/../beta", "a/b", "..", ""] {
            let refused = dispatch(
                &config,
                surface::Request::Stop(AgentName(hostile.to_string())),
            );
            assert_eq!(
                refused,
                Err(LifecycleRefusal::NoSuchAgent),
                "{hostile:?} is refused before any path is built"
            );
        }
        // And a well-formed name off the allow-list is refused by the same
        // check, so what a verb may name has one answer.
        let refused = dispatch(&config, surface::Request::Stop(AgentName("beta".into())));
        assert_eq!(refused, Err(LifecycleRefusal::NoSuchAgent));
    }

    /// The shared check admits exactly what the allow-list carries and refuses
    /// every shape that could leave the directory.
    #[test]
    fn the_name_check_is_one_answer_for_every_verb() {
        let config = unread_config();
        assert_eq!(admissible(&config, &AgentName("alpha".into())), Ok(()));
        for bad in ["alpha/", "/alpha", "al pha", "alpha\u{0}", "ALPHA/../x"] {
            assert_eq!(
                admissible(&config, &AgentName(bad.to_string())),
                Err(LifecycleRefusal::NoSuchAgent),
                "{bad:?} is not admissible"
            );
        }
    }

    #[test]
    fn show_and_list_answer_the_absence_as_unloaded() {
        // **Where no worker answers the dial, the answer is `Unloaded` from
        // the absence and constructs no state from the unit**, per Spec
        // section 3 as of 2026-09-05. Perturbation: map the manager's
        // `active` onto `Idle` in `observe` and a box with the unit running
        // fails this, which is the invention the clause forbids. The
        // coordination root below does not exist, so no socket does.
        let config = unread_config();
        assert_eq!(
            dispatch(&config, surface::Request::Show(AgentName("alpha".into()))),
            Ok(LifecycleAnswer::State {
                state: weaver_types::AgentState::Unloaded,
                load: None
            }),
            "an admitted agent with no socket answers unloaded with no load"
        );
        match dispatch(&config, surface::Request::List) {
            Ok(LifecycleAnswer::Agents { agents }) => {
                assert_eq!(agents.len(), config.allow_list.names().len());
                assert!(
                    agents
                        .iter()
                        .all(|a| a.state == weaver_types::AgentState::Unloaded && a.load.is_none())
                );
            }
            other => panic!("list answers one summary per admitted agent, got {other:?}"),
        }
        assert_eq!(
            dispatch(&config, surface::Request::Show(AgentName("nobody".into()))),
            Err(LifecycleRefusal::NoSuchAgent),
            "a name the allow-list does not admit refuses as every verb does"
        );
    }
}

#[cfg(test)]
mod unload_answer_tests {
    use super::*;

    /// **A clean unload answers unloaded**, which is the defect this rule
    /// closes: the leave confirmed, the worker gone, and the transient unit
    /// collected with it, so the stop that followed named a unit the manager
    /// no longer knew and the verb refused over an agent that unloaded
    /// exactly as asked.
    ///
    /// Perturbation: let the stop ask's status decide again and every unload
    /// of a live agent refuses. Watched by running the verb, which is how the
    /// defect was found.
    #[test]
    fn a_stopped_unit_answers_unloaded() {
        assert!(matches!(
            unload_answer(unit::Residency::Inactive),
            Ok(LifecycleAnswer::State {
                state: weaver_types::AgentState::Unloaded,
                load: None,
            })
        ));
    }

    /// A unit still running refuses, which is the report this verb must never
    /// produce and the case the clause was written against.
    #[test]
    fn a_running_unit_refuses_not_at_rest() {
        assert!(matches!(
            unload_answer(unit::Residency::Active),
            Err(LifecycleRefusal::ActivityNotAtRest)
        ));
    }

    /// **A state nobody could read refuses too.** The ask having failed is not
    /// evidence the worker stopped, and answering unloaded over it would risk
    /// the same false report by another route.
    #[test]
    fn an_unreadable_state_refuses() {
        assert!(matches!(
            unload_answer(unit::Residency::Unknown),
            Err(LifecycleRefusal::BindFailed)
        ));
    }

    /// **The two conditions stop sharing a word**, per `weaver-admin-Spec`
    /// and the ruling of 2026-08-16. A unit the manager reports `failed` left
    /// a name held, which is what refuses a later start. A unit it reports
    /// `active` with an unreachable socket is a bind that failed.
    ///
    /// Perturbation: answer `BindFailed` for `Failed` and this test fails,
    /// which is the state the defect was found in.
    #[test]
    fn a_failed_prior_unit_is_not_a_bind_failure() {
        assert_eq!(
            refusal_for_residency(unit::Residency::Failed),
            LifecycleRefusal::PriorUnitUnreaped
        );
        assert_eq!(
            refusal_for_residency(unit::Residency::Active),
            LifecycleRefusal::BindFailed,
            "a running unit with an unreachable socket is still a bind failure"
        );
        // Both patterns of the shared arm, not one of them. They answer
        // together today and a later act splitting them would pass a test
        // that named only `Inactive`.
        for absent in [unit::Residency::Inactive, unit::Residency::Unknown] {
            assert_eq!(
                refusal_for_residency(absent),
                LifecycleRefusal::NoResidency,
                "neither reading claims more than an absent residency"
            );
        }
    }

    /// **A failed start ask is asked about rather than guessed at.** The
    /// status cannot say which failure it was, so the state decides, and every
    /// reading but `failed` keeps the answer the status alone could give.
    ///
    /// Perturbation: return `from_status` for every residency and the first
    /// case fails, which is how an operator came to read `bind_failed` over a
    /// healthy socket.
    #[test]
    fn a_failed_start_consults_the_state() {
        let from_status = LifecycleRefusal::BindFailed;
        assert_eq!(
            start_refusal_for_residency(unit::Residency::Failed, from_status.clone()),
            LifecycleRefusal::PriorUnitUnreaped
        );
        for other in [
            unit::Residency::Active,
            unit::Residency::Inactive,
            unit::Residency::Unknown,
        ] {
            assert_eq!(
                start_refusal_for_residency(other, from_status.clone()),
                LifecycleRefusal::BindFailed,
                "nothing but failed is claimed beyond the status"
            );
        }
    }

    /// A unit that ran and exited non-zero is not running, which is the
    /// outcome the step asks for. The log carries the residency's own name, so
    /// the failure is reported rather than swallowed.
    #[test]
    fn a_failed_unit_answers_unloaded() {
        assert!(matches!(
            unload_answer(unit::Residency::Failed),
            Ok(LifecycleAnswer::State {
                state: weaver_types::AgentState::Unloaded,
                load: None,
            })
        ));
    }
}
