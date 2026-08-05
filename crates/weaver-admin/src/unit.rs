//! conforms: admin-init-system-over-command-line
//! conforms: admin-unit-declares-one-open
//!
//! The unit, per `weaver-admin-Spec` section 6: the init system is asked over
//! its command-line interface, one invocation per load with the unit's
//! properties declared on the invocation, and one invocation of the stop verb
//! to stop it.
//!
//! The alternative is a bus library, and it loses on the tree: a D-Bus crate
//! brings an async runtime or its own event loop into a binary that otherwise
//! needs neither, for two invocations per lifecycle that are neither hot nor
//! latency-bound.

use std::path::Path;
use std::process::Command;

/// The template's fixed part and the one variable. The template lives in
/// admin's own service configuration, and **the only value interpolated is the
/// validated agent name**, so the delegated authority stays bounded by the
/// allow-list.
#[derive(Debug, Clone)]
pub struct UnitTemplate {
    /// The run tool the operator's init system provides.
    pub run_tool: String,
    /// The properties the fixed template carries beyond `User=`. Which ones is
    /// the operator's policy surface, deliberately not enumerated by the Spec,
    /// because a hardening list frozen in a Spec is a posture that cannot
    /// track its host.
    pub properties: Vec<String>,
    /// The worker binary the unit starts.
    pub worker: std::path::PathBuf,
}

/// Starts the worker's transient unit.
///
/// **The unit declares exactly one open**: the coordination socket's path. The
/// init system connects it at the worker's start and the worker receives the
/// connected end at its first instruction. The sink's descriptor does not
/// travel this way - it crosses inside the enter directive as ancillary data -
/// so a second declaration would be a breach.
pub fn start(
    template: &UnitTemplate,
    identity: &str,
    agent: &str,
    coordination_socket: &Path,
) -> std::io::Result<std::process::ExitStatus> {
    let mut command = Command::new(&template.run_tool);
    command
        .arg("--unit")
        .arg(format!("weaver-worker@{agent}"))
        .arg(format!("--property=User={identity}"));
    for property in &template.properties {
        command.arg(format!("--property={property}"));
    }
    // The one declared open.
    command.arg(format!(
        "--property=Sockets=weaver-coordination@{agent}.socket"
    ));
    command.arg(format!(
        "--property=ListenStream={}",
        coordination_socket.display()
    ));
    command.arg(&template.worker);
    command.status()
}

/// Stops the unit through the same interface.
pub fn stop(template: &UnitTemplate, agent: &str) -> std::io::Result<std::process::ExitStatus> {
    Command::new(&template.run_tool)
        .arg("stop")
        .arg(format!("weaver-worker@{agent}"))
        .status()
}

/// Consults the unit's status for the report the log carries. **Worker death
/// is observed, not reported**, and admin repairs nothing on either.
pub fn status(template: &UnitTemplate, agent: &str) -> std::io::Result<std::process::ExitStatus> {
    Command::new(&template.run_tool)
        .arg("is-active")
        .arg(format!("weaver-worker@{agent}"))
        .status()
}
