//! conforms: admin-no-library-surface
//! conforms: admin-one-floor-link-types-config
//! conforms: admin-no-direct-traits-line
//! conforms: admin-no-runtime-no-bus-no-logging
//! conforms: admin-descriptors-owned-types
//! conforms: admin-runs-as-root-or-performs-nothing
//! conforms: admin-answer-and-exit-status-agree
//! conforms: admin-residency-is-not-lifecycle-state
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
        // **`show` and `list` refuse rather than construct an `AgentState`.**
        // The two fitting answer cases both carry one, and this crate has no
        // source for it: the party that knows an agent's lifecycle state is
        // the harness, and no chartered exchange asks it. What admin can read
        // is residency, which is a different fact, so the refusal is the
        // smallest honest answer available, per Spec section 3.
        surface::Request::Show(_) | surface::Request::List => {
            Err(LifecycleRefusal::StateNotObservable)
        }
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
        // From the user already resolved above rather than a second lookup of
        // the same identity.
        agent_gids: vec![user.gid.as_raw()],
        home: user.dir.clone(),
    };
    inventory::take_inventory(agent, &source, &config.allow_list, &boundary)
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
            })
        }
        Err(refusal) => {
            // **Rollback walks what stands**, and its account is logged.
            let account = verbs::rollback(
                &standing,
                // **No leave is directed on this path**, so the account says
                // so rather than claiming an act nobody performed. A failed
                // load that entered leaves a run the harness unwinds when its
                // channel closes, and a rollback that reported a leave it
                // never sent would put a false act in the one record of the
                // privileged half of the lifecycle.
                || false,
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
    ))?;
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
                spu_instruction: inventory.config.spu_instruction.clone(),
                gate_instruction: inventory.config.gate_instruction.clone(),
            },
        }),
    };
    use std::os::fd::AsFd;
    coordination
        .send_with_sink(&envelope, sink.as_fd())
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

/// **A failed dial is followed by a state ask, so a refusal names the right
/// thing.** A start ask can succeed over a unit that never runs, so the dial's
/// bound is what proves liveness, and the bound alone would report an absent
/// residency where the truth is a unit that is not running.
///
/// What the ask yields is a state and never a reason. `Failed` means one
/// thing; `Inactive` covers a unit that stopped cleanly, one that never
/// existed, and one whose exec never succeeded, so nothing is claimed beyond
/// the value.
fn refusal_for_absent_worker(config: &ServiceConfig, agent: &str) -> LifecycleRefusal {
    match unit::residency(&config.unit, agent) {
        // The unit ran and exited non-zero: the worker is not there to bind.
        unit::Residency::Failed => LifecycleRefusal::BindFailed,
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
        }),
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
        },
        allow_list: inventory::AllowList::new(provisioned),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A configuration whose values are never read by the arm under test.
    /// `dispatch`'s observation arm touches no field, which is what lets this
    /// stand without a filesystem.
    fn unread_config() -> ServiceConfig {
        ServiceConfig {
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
    fn show_and_list_construct_no_agent_state() {
        let config = unread_config();
        for request in [
            surface::Request::Show(AgentName("alpha".into())),
            surface::Request::List,
        ] {
            let answered = dispatch(&config, request.clone());
            assert_eq!(
                answered,
                Err(LifecycleRefusal::StateNotObservable),
                "{request:?} refuses as not observable rather than answering"
            );
        }
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
                state: weaver_types::AgentState::Unloaded
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

    /// A unit that ran and exited non-zero is not running, which is the
    /// outcome the step asks for. The log carries the residency's own name, so
    /// the failure is reported rather than swallowed.
    #[test]
    fn a_failed_unit_answers_unloaded() {
        assert!(matches!(
            unload_answer(unit::Residency::Failed),
            Ok(LifecycleAnswer::State {
                state: weaver_types::AgentState::Unloaded
            })
        ));
    }
}
