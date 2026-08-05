//! conforms: admin-publishes-only-on-ready
//! conforms: admin-validate-starts-no-process
//! conforms: admin-stop-answer-relayed-unchanged
//! conforms: admin-one-exchange-in-flight-per-agent
//!
//! The verbs, the fleet state, and rollback, per `weaver-admin-Spec` section
//! 3. **Admin authorizes and does not execute**: what a run does after the
//! enter directive is the harness's, and every verb here stops at the seam.

use std::collections::BTreeMap;
use std::sync::Mutex;

#[cfg(test)]
use weaver_types::LifecycleAnswer;
use weaver_types::{AgentName, AgentState, AgentSummary, LifecycleRefusal};

/// One agent's entry: the published state and whether a transition holds it.
#[derive(Debug, Clone)]
struct Entry {
    published: AgentState,
    in_flight: bool,
}

/// The fleet state: a locked map, one entry per provisioned agent.
///
/// **Loaded-and-idle is published only on a ready aggregate and never
/// earlier**, and a failed anything publishes nothing. This map is where that
/// holds: one write of the published state per transition and none on any
/// other path.
///
/// **One exchange in flight per agent**, by this lock. The channel's layer
/// permits concurrency and this crate's contract forbids a second transition
/// for the same agent, so the serialization is admin's discipline at the fleet
/// state rather than a property claimed of the wire.
#[derive(Debug, Default)]
pub struct Fleet {
    entries: Mutex<BTreeMap<String, Entry>>,
}

/// A transition's claim on an agent, released when it drops.
pub struct Claim<'fleet> {
    fleet: &'fleet Fleet,
    agent: String,
}

impl Fleet {
    pub fn new(provisioned: impl IntoIterator<Item = String>) -> Self {
        let entries = provisioned
            .into_iter()
            .map(|name| {
                (
                    name,
                    Entry {
                        published: AgentState::Unloaded,
                        in_flight: false,
                    },
                )
            })
            .collect();
        Fleet {
            entries: Mutex::new(entries),
        }
    }

    /// Takes the agent's entry for a transition, or refuses because one is
    /// already in flight. A second directive for the same agent refuses rather
    /// than queueing.
    pub fn claim(&self, agent: &AgentName) -> Result<Claim<'_>, LifecycleRefusal> {
        let mut entries = self.entries.lock().expect("fleet state");
        let entry = entries
            .get_mut(&agent.0)
            .ok_or(LifecycleRefusal::NoSuchAgent)?;
        if entry.in_flight {
            return Err(LifecycleRefusal::OutOfOrder);
        }
        entry.in_flight = true;
        Ok(Claim {
            fleet: self,
            agent: agent.0.clone(),
        })
    }

    /// Publishes a state. The only caller is a completed transition.
    fn publish(&self, agent: &str, state: AgentState) {
        let mut entries = self.entries.lock().expect("fleet state");
        if let Some(entry) = entries.get_mut(agent) {
            entry.published = state;
        }
    }

    pub fn state_of(&self, agent: &AgentName) -> Option<AgentState> {
        let entries = self.entries.lock().expect("fleet state");
        entries.get(&agent.0).map(|e| e.published.clone())
    }

    pub fn summaries(&self) -> Vec<AgentSummary> {
        let entries = self.entries.lock().expect("fleet state");
        entries
            .iter()
            .map(|(name, entry)| AgentSummary {
                name: AgentName(name.clone()),
                state: entry.published.clone(),
            })
            .collect()
    }
}

impl Claim<'_> {
    /// Publishes loaded-and-idle. **Called only on a ready aggregate**: every
    /// other outcome leaves the claim to drop with nothing published.
    pub fn publish_loaded(&self) {
        self.fleet.publish(&self.agent, AgentState::Idle);
    }

    /// Publishes provisioned-and-unloaded, after a completed unload.
    pub fn publish_unloaded(&self) {
        self.fleet.publish(&self.agent, AgentState::Unloaded);
    }
}

impl Drop for Claim<'_> {
    fn drop(&mut self) {
        let mut entries = self.fleet.entries.lock().expect("fleet state");
        if let Some(entry) = entries.get_mut(&self.agent) {
            entry.in_flight = false;
        }
    }
}

/// What a load left standing, walked in reverse by the rollback.
#[derive(Debug, Default, Clone)]
pub struct Standing {
    pub entered: bool,
    pub unit_started: bool,
    pub sink_opened: bool,
}

/// One rollback act's outcome, for the account the log carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Undone {
    pub act: &'static str,
    pub succeeded: bool,
}

/// **Rollback is the reap plus one directive, as data.** It walks what stands:
/// direct leave where a run was entered, stop the unit where a unit started,
/// close the sink where one opened. Each act's outcome enters the account, the
/// rollback reports what it could not undo, and **no state is published on any
/// partial outcome**.
pub fn rollback<L, U, S>(
    standing: &Standing,
    mut direct_leave: L,
    mut stop_unit: U,
    mut close_sink: S,
) -> Vec<Undone>
where
    L: FnMut() -> bool,
    U: FnMut() -> bool,
    S: FnMut() -> bool,
{
    let mut account = Vec::new();
    if standing.entered {
        account.push(Undone {
            act: "leave",
            succeeded: direct_leave(),
        });
    }
    if standing.unit_started {
        account.push(Undone {
            act: "unit-stop",
            succeeded: stop_unit(),
        });
    }
    if standing.sink_opened {
        account.push(Undone {
            act: "sink-close",
            succeeded: close_sink(),
        });
    }
    account
}

/// **`stop` is a conveyance and its answer is a relay.**
///
/// Reachable only from this module's tests today, because the floor's `Stop`
/// carries no agent name and the operator surface therefore has no target to
/// convey it to - the routing gap named at the composition root. The relay's
/// rule is written and watched here so it does not have to be rediscovered
/// when the contract's next opening restores the route.
/// The harness's answer returns to the operator as received: admin holds no
/// opinion about which, because authorizing a stop and deciding what a stop
/// found are different acts, and the second is the harness's. A relay that
/// translated the answer would be admin ruling on a run it does not conduct.
#[cfg(test)]
pub fn relay_stop_answer(from_harness: LifecycleAnswer) -> LifecycleAnswer {
    from_harness
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The rollback's account:** a load failed after each step leaves
    /// exactly what the charter names, and the account records what was undone
    /// and what could not be.
    ///
    /// Perturbation: move the account off the rollback path and it goes
    /// silent, which the emptiness assertion catches.
    #[test]
    fn the_rollback_walks_only_what_stands() {
        // Nothing stood up: nothing is undone.
        let empty = rollback(&Standing::default(), || true, || true, || true);
        assert!(
            empty.is_empty(),
            "a load that stood nothing up undoes nothing"
        );

        // A sink opened and a unit started, but no run entered.
        let partial = rollback(
            &Standing {
                entered: false,
                unit_started: true,
                sink_opened: true,
            },
            || panic!("no leave is directed where no run entered"),
            || true,
            || true,
        );
        assert_eq!(
            partial,
            vec![
                Undone {
                    act: "unit-stop",
                    succeeded: true
                },
                Undone {
                    act: "sink-close",
                    succeeded: true
                },
            ]
        );

        // Everything stood, and the leave could not be undone: the account
        // reports what it could not do rather than going quiet.
        let full = rollback(
            &Standing {
                entered: true,
                unit_started: true,
                sink_opened: true,
            },
            || false,
            || true,
            || true,
        );
        assert_eq!(
            full[0],
            Undone {
                act: "leave",
                succeeded: false
            }
        );
        assert_eq!(full.len(), 3, "the reverse order is walked whole");
    }

    /// A second transition for the same agent refuses rather than queueing,
    /// and the claim releases when it drops.
    #[test]
    fn one_transition_in_flight_per_agent() {
        let fleet = Fleet::new(["alpha".to_string(), "beta".to_string()]);
        let name = AgentName("alpha".to_string());
        let claim = fleet.claim(&name).expect("first claim");
        assert!(
            matches!(fleet.claim(&name), Err(LifecycleRefusal::OutOfOrder)),
            "a second directive for the same agent refuses"
        );
        // A different agent is unaffected: the lock is per entry.
        let other = fleet.claim(&AgentName("beta".into())).expect("other agent");
        drop(other);
        drop(claim);
        assert!(fleet.claim(&name).is_ok(), "the claim released on drop");
    }

    /// **Loaded-and-idle is published only on a ready aggregate.** A claim
    /// that drops without publishing leaves the state as it was.
    #[test]
    fn a_failed_transition_publishes_nothing() {
        let fleet = Fleet::new(["alpha".to_string()]);
        let name = AgentName("alpha".to_string());
        assert_eq!(fleet.state_of(&name), Some(AgentState::Unloaded));
        {
            let _claim = fleet.claim(&name).expect("claim");
            // The transition fails: nothing is published.
        }
        assert_eq!(
            fleet.state_of(&name),
            Some(AgentState::Unloaded),
            "a failed anything publishes nothing"
        );
        {
            let claim = fleet.claim(&name).expect("claim");
            claim.publish_loaded();
        }
        assert_eq!(fleet.state_of(&name), Some(AgentState::Idle));
    }

    /// The stop answer is relayed unchanged: admin holds no opinion about what
    /// a stop found.
    #[test]
    fn the_stop_answer_is_relayed_unchanged() {
        let aborted = LifecycleAnswer::TurnAborted {
            turn: weaver_types::TurnKey("t-1".into()),
        };
        assert_eq!(relay_stop_answer(aborted.clone()), aborted);
        assert_eq!(
            relay_stop_answer(LifecycleAnswer::AtRest),
            LifecycleAnswer::AtRest
        );
    }
}
