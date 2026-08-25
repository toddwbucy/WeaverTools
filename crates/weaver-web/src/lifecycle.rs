//! The verb invocation (Spec section 11): sudo weaver-admin, one JSON
//! object on stdout, rendered verbatim; failure never swallowed.

use serde::Serialize;
use tokio::process::Command;

const ADMIN_BIN: &str = "/usr/local/libexec/weaver/weaver-admin";
const ADMIN_CONFIG: &str = "/etc/weaver/config";
pub const VERBS: [&str; 3] = ["validate", "load", "unload"];

/// Generous ceiling on a verb invocation: load blocks until the
/// interior is idle (tens of seconds for the 35B admit), so this only
/// catches a hang, never a slow success.
const VERB_TIMEOUT_SECS: u64 = 300;

#[derive(Debug, Clone, Serialize)]
pub struct VerbOutcome {
    pub verb: String,
    pub agent: String,
    pub exit_code: Option<i32>,
    /// stdout parsed as one JSON object, when it is one.
    pub answer: Option<serde_json::Value>,
    /// raw stdout, kept when parsing failed so nothing is swallowed.
    pub raw_stdout: Option<String>,
    pub stderr: Option<String>,
    /// True when the invocation hit the timeout and was killed.
    pub timed_out: bool,
}

pub async fn run_verb(verb: &str, agent: &str) -> anyhow::Result<VerbOutcome> {
    if !VERBS.contains(&verb) {
        anyhow::bail!("'{verb}' is not a lifecycle verb");
    }
    if agent.is_empty()
        || !agent.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        anyhow::bail!("'{agent}' is not a well-formed agent name");
    }

    let fut = Command::new("sudo")
        .arg("-n")
        .arg(format!("WEAVER_ADMIN_CONFIG={ADMIN_CONFIG}"))
        .arg(ADMIN_BIN)
        .arg(verb)
        .arg(agent)
        .kill_on_drop(true)
        .output();
    let output = match tokio::time::timeout(
        std::time::Duration::from_secs(VERB_TIMEOUT_SECS),
        fut,
    )
    .await
    {
        Ok(res) => res?,
        Err(_) => {
            return Ok(VerbOutcome {
                verb: verb.to_string(),
                agent: agent.to_string(),
                exit_code: None,
                answer: None,
                raw_stdout: None,
                stderr: Some(format!(
                    "invocation exceeded {VERB_TIMEOUT_SECS}s and was killed"
                )),
                timed_out: true,
            });
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let answer: Option<serde_json::Value> = serde_json::from_str(stdout.trim()).ok();

    Ok(VerbOutcome {
        verb: verb.to_string(),
        agent: agent.to_string(),
        exit_code: output.status.code(),
        raw_stdout: if answer.is_none() && !stdout.is_empty() { Some(stdout) } else { None },
        answer,
        stderr: if stderr.is_empty() { None } else { Some(stderr) },
        timed_out: false,
    })
}
