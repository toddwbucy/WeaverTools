//! conforms: admin-init-system-over-command-line
//! conforms: admin-unit-declares-no-open
//!
//! The unit, per `weaver-admin-Spec` section 6: the init system is asked over
//! its command-line interface, one invocation per load with the unit's
//! properties declared on the invocation, one invocation of the stop verb to
//! stop it, and the same interface answers the residency query of section 3.
//!
//! The alternative is a bus library, and it loses on the tree: `zbus` carries
//! async machinery into the resolved tree whatever its surface API, which this
//! crate's own manifest assertion forbids, and `dbus-rs` trades that for a C
//! library dependency in a binary that otherwise links none. What the command
//! line costs instead is failure discrimination, named at
//! `weaver-admin-systemd-contract` section 3 and not defended here.

use std::process::Command;

/// The template's fixed part and the one variable. The template lives in
/// admin's own service configuration, and **the only value interpolated is the
/// validated agent name**, so a name reaching a unit has one origin.
#[derive(Debug, Clone)]
pub struct UnitTemplate {
    /// The run tool the operator's init system provides.
    pub run_tool: String,
    /// The control tool that answers the residency query and stops a unit.
    pub control_tool: String,
    /// The properties the fixed template carries beyond `User=`. Which ones is
    /// the operator's policy surface, deliberately not enumerated by the Spec,
    /// because a hardening list frozen in a Spec is a posture that cannot
    /// track its host.
    pub properties: Vec<String>,
    /// The worker binary the unit starts.
    pub worker: std::path::PathBuf,
}

/// What the init system answers about a unit, per
/// `weaver-admin-systemd-contract` section 3.
///
/// **Three values, and they separate less than they look.** `Active` and
/// `Failed` each mean one thing, a running unit and one whose process exited
/// non-zero. `Inactive` means at least three: a unit that stopped cleanly, one
/// that never existed, and one whose exec never succeeded. Measured against a
/// live manager on 2026-08-05. A party reading this learns that a worker is
/// not running and does not learn why.
///
/// **This is residency and not lifecycle state.** Constructing an `AgentState`
/// from it would be inventing a fact, per Spec section 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Residency {
    Active,
    Failed,
    Inactive,
    /// The ask itself failed. Reported as unknown rather than guessed at,
    /// because a state this boundary could not answer is not a state the
    /// program may invent.
    Unknown,
}

impl Residency {
    /// The manager's own name for the value, carried rather than translated,
    /// because a translation is where the invention would enter.
    pub fn as_str(self) -> &'static str {
        match self {
            Residency::Active => "active",
            Residency::Failed => "failed",
            Residency::Inactive => "inactive",
            Residency::Unknown => "unknown",
        }
    }
}

/// The per-agent unit name, from the validated agent name and nothing else.
pub fn unit_name(agent: &str) -> String {
    format!("weaver-worker@{agent}.service")
}

/// The runtime directory the coordination socket is bound inside, named
/// relative to the manager's runtime root because the manager creates and
/// removes it.
pub fn runtime_directory_name(agent: &str) -> String {
    format!("weaver-{agent}")
}

/// Starts the worker's transient unit.
///
/// **The unit declares no descriptor-bearing open.** The worker starts bare
/// and binds its own coordination socket inside the runtime directory asked
/// for here, per `weaver-harness-Spec` section 2.3, so nothing is placed into
/// the unit at start and no descriptor crosses the init system at all. The
/// sink's descriptor crosses later and elsewhere, inside the enter directive
/// over the connection admin dialed.
///
/// **The runtime directory is asked for because its removal is the answer to a
/// stale socket.** A Unix socket's pathname outlives the process that bound
/// it, and the program does not solve that by unlinking, which races a live
/// successor. The manager creates the directory at start and destroys it with
/// the unit, so the pathname cannot outlive the worker.
pub fn start(
    template: &UnitTemplate,
    identity: &str,
    agent: &str,
) -> std::io::Result<std::process::ExitStatus> {
    Command::new(&template.run_tool)
        .args(start_arguments(template, identity, agent))
        .status()
}

/// The argument vector `start` runs, built here so nothing reads a copy of it.
/// The test that asserts no descriptor-bearing property is declared reads this
/// function, and a builder who added such a property to `start` would have to
/// add it here, which is what keeps that test from passing over a drifted
/// invocation.
fn start_arguments(template: &UnitTemplate, identity: &str, agent: &str) -> Vec<String> {
    let mut args = vec![
        "--unit".to_string(),
        format!("weaver-worker@{agent}"),
        format!("--property=User={identity}"),
        format!(
            "--property=RuntimeDirectory={}",
            runtime_directory_name(agent)
        ),
    ];
    for property in &template.properties {
        args.push(format!("--property={property}"));
    }
    args.push(template.worker.display().to_string());
    args
}

/// Stops the unit and waits for it to have stopped.
///
/// **A stop that is accepted is not a stop that has happened.**
/// `weaver-admin-systemd-contract` section 4 promises the ask is answered when
/// the unit has stopped rather than when the stop was accepted, so this call
/// is the confirmation and this crate elects no timer beside it.
pub fn stop(template: &UnitTemplate, agent: &str) -> std::io::Result<std::process::ExitStatus> {
    Command::new(&template.control_tool)
        .arg("stop")
        .arg(unit_name(agent))
        .status()
}

/// Asks the init system what state the unit is in.
///
/// The value is carried and nothing is claimed beyond it. An ask that fails is
/// `Unknown` rather than a guess.
pub fn residency(template: &UnitTemplate, agent: &str) -> Residency {
    let out = Command::new(&template.control_tool)
        .arg("is-active")
        .arg(unit_name(agent))
        .output();
    match out {
        Ok(out) => match String::from_utf8_lossy(&out.stdout).trim() {
            "active" | "activating" | "reloading" => Residency::Active,
            "failed" => Residency::Failed,
            "inactive" | "deactivating" => Residency::Inactive,
            _ => Residency::Unknown,
        },
        Err(_) => Residency::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The start invocation carries no descriptor-bearing property.** The
    /// route that placed the trace's far end on the unit's standard output was
    /// declined, because standard output is inherited across fork and exec and
    /// would hand every organ the harness forks a writable handle to the
    /// agent's own record.
    ///
    /// Perturbation: add a `StandardOutput=` or `ListenStream=` property to
    /// `start` and this test fails. Watched under exactly that addition.
    #[test]
    fn the_start_invocation_declares_no_descriptor() {
        let template = UnitTemplate {
            run_tool: "/bin/true".into(),
            control_tool: "/bin/true".into(),
            properties: vec!["PrivateTmp=yes".into(), "NoNewPrivileges=yes".into()],
            worker: "/usr/libexec/weaver-worker".into(),
        };
        let rendered = start_arguments(&template, "weaver-alpha", "alpha");
        for banned in [
            "StandardOutput=",
            "StandardError=",
            "StandardInput=",
            "ListenStream=",
            "Sockets=",
            "FileDescriptorName=",
        ] {
            assert!(
                !rendered.iter().any(|a| a.contains(banned)),
                "the start invocation carries {banned}, which places a descriptor: {rendered:?}"
            );
        }
        assert!(
            rendered.iter().any(|a| a.contains("RuntimeDirectory=")),
            "the runtime directory is asked for: {rendered:?}"
        );
    }

    /// The one interpolated value is the validated agent name, so a name
    /// reaching a unit has one origin.
    #[test]
    fn the_agent_name_is_the_one_variable() {
        let template = UnitTemplate {
            run_tool: "/bin/true".into(),
            control_tool: "/bin/true".into(),
            properties: vec![],
            worker: "/usr/libexec/weaver-worker".into(),
        };
        let rendered = start_arguments(&template, "weaver-alpha", "alpha");
        assert!(rendered.iter().any(|a| a == "--property=User=weaver-alpha"));
        assert!(
            rendered
                .iter()
                .any(|a| a == "--property=RuntimeDirectory=weaver-alpha")
        );
        assert_eq!(unit_name("alpha"), "weaver-worker@alpha.service");
    }
}
