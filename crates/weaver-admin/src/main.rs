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

/// The one inventory, called by both `validate` and `load`, so the two cannot
/// drift.
fn take_inventory(
    config: &ServiceConfig,
    agent: &AgentName,
) -> Result<inventory::Inventory, LifecycleRefusal> {
    // **The name is authorized before it reaches the filesystem.** `AgentName`
    // wraps a bare string, so a name carrying `/` or `..` would otherwise be
    // interpolated into a config path and traverse out of the directory the
    // operator placed.
    if !config.allow_list.admits(agent)
        || agent.0.is_empty()
        || !agent
            .0
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(LifecycleRefusal::NoSuchAgent);
    }
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
        agent_gids: agent_gids(&identity),
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
                || true,
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
    unit::start(&config.unit, &inventory.identity, &agent.0)
        .map_err(|_| LifecycleRefusal::BindFailed)?;
    standing.unit_started = true;

    // **The dial races the worker's bind and the bound covers it.** A bound
    // exceeded is not an absent residency, so the refusal consults the unit's
    // state before returning and carries what the manager said.
    let socket_path = config.coordination_socket(&agent.0);
    let coordination = match channel::dial(&socket_path) {
        Ok(coordination) => coordination,
        Err(_) => return Err(refusal_for_absent_worker(config, &agent.0)),
    };
    let mut coordination = coordination;

    let ordinal = coordination.next_ordinal();
    let envelope = weaver_types::OrganEnvelope {
        exchange: weaver_types::ExchangeId {
            opener: weaver_types::Opener::Admin,
            ordinal,
        },
        position: weaver_types::Position::Open,
        payload: weaver_types::Payload::Directive(LifecycleDirective::Enter {
            payload: weaver_types::EnterPayload {
                session: weaver_types::SessionId(format!("{}-{ordinal}", agent.0)),
                run_ordinal: ordinal,
                model_binding: inventory.config.model_binding.clone(),
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
        unit::Residency::Failed => LifecycleRefusal::BindFailed,
        _ => LifecycleRefusal::NoResidency,
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
    match stopped {
        Ok(status) if status.success() => {}
        _ => return Err(LifecycleRefusal::BindFailed),
    }
    if residency == unit::Residency::Active {
        return Err(LifecycleRefusal::ActivityNotAtRest);
    }
    Ok(LifecycleAnswer::State {
        state: weaver_types::AgentState::Unloaded,
    })
}

/// `stop` is a conveyance and its answer is a relay. Admin holds no opinion
/// about which fate the harness reports: authorizing a stop and deciding what
/// a stop found are different acts, and the second is the harness's.
fn stop(config: &ServiceConfig, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
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
        },
        allow_list: inventory::AllowList::new(provisioned),
    })
}

fn agent_gids(identity: &str) -> Vec<u32> {
    nix::unistd::User::from_name(identity)
        .ok()
        .flatten()
        .map(|user| vec![user.gid.as_raw()])
        .unwrap_or_default()
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
