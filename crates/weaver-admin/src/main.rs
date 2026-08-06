//! conforms: admin-no-library-surface
//! conforms: admin-one-floor-link-types-config
//! conforms: admin-no-direct-traits-line
//! conforms: admin-no-runtime-no-bus-no-logging
//! conforms: admin-descriptors-owned-types
//! conforms: admin-publishes-only-on-ready
//! conforms: admin-validate-starts-no-process
//! conforms: admin-inventory-one-function
//!
//! `weaver-admin`: the supervisor. One binary and no library surface, per
//! `weaver-admin-Spec` section 1 - nothing links admin, and a library target
//! would be an API for a consumer the topology forbids.
//!
//! Entry and wiring, and nothing else: each obligation lives in its own
//! module, and this file composes them.

mod channel;
mod inventory;
mod log;
mod sink;
mod surface;
mod unit;
mod verbs;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use weaver_types::{AgentName, LifecycleAnswer, LifecycleDirective, LifecycleRefusal};

/// Admin's own operator-installed configuration, per Spec section 9: the
/// operator socket's path, the coordination directory, the log directory, the
/// unit template, and the fleet allow-list. **These are deployment facts of
/// the service** - not the agent config, crossing no seam, read by no other
/// crate, and **none of them discovered at runtime by searching**.
///
/// The file's format and field list are an open election of section 11. What
/// this struct fixes is that the values exist and are the operator's to place.
struct ServiceConfig {
    operator_socket: PathBuf,
    coordination_directory: PathBuf,
    log_path: PathBuf,
    agent_config_directory: PathBuf,
    unit: unit::UnitTemplate,
    allow_list: inventory::AllowList,
    admin_gid: u32,
    provisioned: Vec<String>,
}

/// What the supervisor holds while it serves.
struct Supervisor {
    config: ServiceConfig,
    fleet: verbs::Fleet,
    /// The coordination channel of each loaded agent, held across verbs
    /// because a stop is conveyed over the channel the load accepted and a
    /// leave is directed over the same one.
    channels: Mutex<BTreeMap<String, channel::Coordination>>,
}

fn main() {
    let config = match load_service_config() {
        Ok(config) => config,
        Err(reason) => {
            eprintln!("weaver-admin: {reason}");
            std::process::exit(2);
        }
    };
    let fleet = verbs::Fleet::new(config.provisioned.clone());
    let supervisor = Arc::new(Supervisor {
        config,
        fleet,
        channels: Mutex::new(BTreeMap::new()),
    });

    let listener = match surface::bind(&supervisor.config.operator_socket) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("weaver-admin: the operator socket could not be bound: {e}");
            std::process::exit(2);
        }
    };

    // The surface's policy: the allow set is the admin group and the deny set
    // is every agent uid the fleet names, denial winning over permission.
    let policy = surface::SurfacePolicy::new(
        supervisor.config.admin_gid,
        supervisor
            .config
            .provisioned
            .iter()
            .filter_map(|name| agent_uid(&inventory::identity_for(&AgentName(name.clone())))),
    );

    // **One thread per accepted connection, requests served serially within
    // it.** The threads are the standard library's: nothing here needs an
    // executor, the surface's traffic being operator-paced.
    loop {
        let connection = match surface::accept_cloexec(&listener) {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("weaver-admin: accept failed: {e}");
                continue;
            }
        };
        let supervisor = Arc::clone(&supervisor);
        let policy = policy.clone();
        std::thread::spawn(move || {
            let _ = surface::serve_connection(connection, &policy, |directive| {
                supervisor.dispatch(directive)
            });
        });
    }
}

impl Supervisor {
    /// One directive, judged and answered. Every verb stops at the seam:
    /// **admin authorizes and does not execute**.
    fn dispatch(&self, directive: LifecycleDirective) -> Result<LifecycleAnswer, LifecycleRefusal> {
        match directive {
            LifecycleDirective::Validate { agent } => self.validate(&agent),
            LifecycleDirective::Load { agent } => self.load(&agent),
            LifecycleDirective::Unload { agent } => self.unload(&agent),
            // **The floor's `Stop` carries no agent name**, so a stop
            // arriving on the operator surface names no target for admin to
            // convey it to. The relay itself is written and tested in
            // `verbs::relay_stop_answer`; what is missing is the routing, and
            // the honest answer is a refusal rather than a guess at which
            // agent the operator meant. Filed for the contract's next
            // opening, beside the harness's own two vocabulary notes.
            LifecycleDirective::Stop => Err(LifecycleRefusal::Malformed),
            LifecycleDirective::Show { agent } => self
                .fleet
                .state_of(&agent)
                .map(|state| LifecycleAnswer::State { state })
                .ok_or(LifecycleRefusal::NoSuchAgent),
            LifecycleDirective::List => Ok(LifecycleAnswer::Agents {
                agents: self.fleet.summaries(),
            }),
            // Every other case belongs to a seam this surface does not carry:
            // the organ directives cross the coordination channel, not here.
            _ => Err(LifecycleRefusal::OutOfOrder),
        }
    }

    /// **`validate` is the load's front half**, and it stops at the report:
    /// it touches no seam and starts no process, which is what makes
    /// `Validated` mean an outcome rather than a transition.
    fn validate(&self, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
        let _claim = self.fleet.claim(agent)?;
        self.take_inventory(agent)?;
        Ok(LifecycleAnswer::Validated)
    }

    /// The one inventory, called by both `validate` and `load`, so the two
    /// cannot drift.
    fn take_inventory(&self, agent: &AgentName) -> Result<inventory::Inventory, LifecycleRefusal> {
        // **The name is authorized before it reaches the filesystem.**
        // `AgentName` wraps a bare string, so a name carrying `/` or `..`
        // would otherwise be interpolated into a config path and traverse out
        // of the directory the operator placed. The inventory consults the
        // allow-list again, being the one function both callers share; this
        // check is what keeps an unlisted name from being spelled into a path
        // first.
        if !self.config.allow_list.admits(agent)
            || agent.0.is_empty()
            || !agent
                .0
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(LifecycleRefusal::NoSuchAgent);
        }
        let identity = inventory::identity_for(agent);
        let source_path = self
            .config
            .agent_config_directory
            .join(format!("{}.yaml", agent.0));
        let source =
            std::fs::read_to_string(&source_path).map_err(|_| LifecycleRefusal::NoSuchAgent)?;
        // The home comes from the account database rather than from a
        // constructed path: an operator who placed the agent elsewhere would
        // otherwise have the boundary checked against a directory that is not
        // the agent's.
        let user = nix::unistd::User::from_name(&identity)
            .ok()
            .flatten()
            .ok_or(LifecycleRefusal::BoundaryUnverified)?;
        let boundary = inventory::Boundary {
            agent_uid: user.uid.as_raw(),
            // The admin principal is this process: custody is held by whoever
            // opens the sink, and that is this crate under its own account.
            admin_uid: nix::unistd::getuid().as_raw(),
            agent_gids: agent_gids(&identity),
            home: user.dir.clone(),
        };
        inventory::take_inventory(agent, &source, &self.config.allow_list, &boundary)
    }

    /// `load` runs the charter's steps with the channel's acts interleaved,
    /// and **publishes only on a ready aggregate**: every other outcome enters
    /// the rollback carrying the step's name and publishes nothing.
    fn load(&self, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
        let claim = self.fleet.claim(agent)?;
        let mut operations = log::OperationsLog::open(&self.config.log_path)
            .map_err(|_| LifecycleRefusal::BoundaryUnverified)?;
        let mut standing = verbs::Standing::default();

        let outcome = self.run_load(agent, &mut standing);
        match outcome {
            Ok(()) => {
                claim.publish_loaded();
                let _ = operations.record(&log::Act {
                    verb: "load",
                    agent: agent.0.clone(),
                    outcome: "ready".into(),
                    undone: None,
                });
                Ok(LifecycleAnswer::State {
                    state: weaver_types::AgentState::Idle,
                })
            }
            Err(refusal) => {
                // **Rollback walks what stands**, and its account is logged.
                let account = verbs::rollback(
                    &standing,
                    || true,
                    || unit::stop(&self.config.unit, &agent.0).is_ok(),
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
        &self,
        agent: &AgentName,
        standing: &mut verbs::Standing,
    ) -> Result<(), LifecycleRefusal> {
        let inventory = self.take_inventory(agent)?;
        let sink = sink::open(&inventory.config.trace_sink)?;
        standing.sink_opened = true;

        // Bind and listen stands as its own act **before the unit**, because
        // the worker connects at its start and a name not yet listening is a
        // race.
        let (listener, socket_path) =
            channel::bind_and_listen(&self.config.coordination_directory, &agent.0)
                .map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;

        unit::start(
            &self.config.unit,
            &inventory.identity,
            &agent.0,
            &socket_path,
        )
        .map_err(|_| LifecycleRefusal::BindFailed)?;
        standing.unit_started = true;

        let expected =
            agent_uid(&inventory.identity).ok_or(LifecycleRefusal::BoundaryUnverified)?;
        let mut coordination = listener
            .accept_once(expected)
            .map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;

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

        let outcome = match coordination.recv() {
            Ok(answer) => match answer.payload {
                weaver_types::Payload::Answer(LifecycleAnswer::Ready) => Ok(()),
                weaver_types::Payload::Refusal(refusal) => Err(refusal),
                _ => Err(LifecycleRefusal::Malformed),
            },
            Err(_) => Err(LifecycleRefusal::NoResidency),
        };
        if outcome.is_ok() {
            self.channels
                .lock()
                .expect("channels")
                .insert(agent.0.clone(), coordination);
        }
        outcome
    }

    /// `unload` directs leave, stops the unit, and publishes
    /// provisioned-and-unloaded. A refusal on leave returns unchanged and
    /// publishes nothing.
    fn unload(&self, agent: &AgentName) -> Result<LifecycleAnswer, LifecycleRefusal> {
        let claim = self.fleet.claim(agent)?;
        // The leave is directed over the channel the load accepted, and the
        // channel is released with the run.
        // **The channel is taken only once the leave has been directed
        // successfully.** Removing it first would drop the run's one route on
        // a failure, leaving nothing to retry the leave over, and publishing
        // after a failed leave would contradict this verb's own contract.
        {
            let mut channels = self.channels.lock().expect("channels");
            if let Some(channel) = channels.get_mut(&agent.0) {
                let ordinal = channel.next_ordinal();
                channel
                    .send_directive(ordinal, LifecycleDirective::Leave)
                    .map_err(|_| LifecycleRefusal::NoResidency)?;
            }
        }
        unit::stop(&self.config.unit, &agent.0).map_err(|_| LifecycleRefusal::BindFailed)?;
        self.channels.lock().expect("channels").remove(&agent.0);

        // **Worker death is observed, not reported**, and the unit's status is
        // consulted through the same interface for the report the log carries.
        let mut operations = log::OperationsLog::open(&self.config.log_path)
            .map_err(|_| LifecycleRefusal::BoundaryUnverified)?;
        let status = unit::status(&self.config.unit, &agent.0);
        let _ = operations.record(&log::Act {
            verb: "unload",
            agent: agent.0.clone(),
            outcome: match &status {
                Ok(s) if s.success() => "unit-active".into(),
                Ok(_) => "unit-inactive".into(),
                Err(_) => "unit-status-unavailable".into(),
            },
            undone: None,
        });
        claim.publish_unloaded();
        Ok(LifecycleAnswer::State {
            state: weaver_types::AgentState::Unloaded,
        })
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
        operator_socket: PathBuf::from(read("operator-socket")?),
        coordination_directory: PathBuf::from(read("coordination-directory")?),
        log_path: PathBuf::from(read("log-path")?),
        agent_config_directory: PathBuf::from(read("agent-config-directory")?),
        unit: unit::UnitTemplate {
            run_tool: read("run-tool")?,
            properties: read("unit-properties")
                .unwrap_or_default()
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect(),
            worker: PathBuf::from(read("worker-binary")?),
        },
        allow_list: inventory::AllowList::new(provisioned.clone()),
        admin_gid: read("admin-gid")?
            .parse()
            .map_err(|_| "admin-gid is not a group id".to_string())?,
        provisioned,
    })
}

/// Resolves a provisioned identity's uid. The identity is built from the
/// validated name at the one constructing site, so this never takes a
/// caller-supplied string.
fn agent_uid(identity: &str) -> Option<u32> {
    nix::unistd::User::from_name(identity)
        .ok()
        .flatten()
        .map(|user| user.uid.as_raw())
}

fn agent_gids(identity: &str) -> Vec<u32> {
    nix::unistd::User::from_name(identity)
        .ok()
        .flatten()
        .map(|user| vec![user.gid.as_raw()])
        .unwrap_or_default()
}
