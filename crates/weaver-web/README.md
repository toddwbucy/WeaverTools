# Weaver-Web

The frontend of the WeaverTools suite: the user's and operator's surface
over running weaver agents. It is the suite's first consumer application  - 
built against the framework's public boundary, not inside it.

One Rust binary serves three surfaces:

- **Channel** - multi-party chat: humans and local weaver agents (and
  eventually upstream models) in shared channels. Agents are first-class
  participants, invoked by `@mention`, answering whole turns with their
  run/turn labels linking each answer to the trace that produced it.
- **Lifecycle** *(admin)* - the operator's verbs (`validate` / `load` /
  `unload`) over declared agents, each answer rendered verbatim, load
  state shown from the gate socket's existence and labeled as the
  inference it is.
- **Trace** *(admin)* - a live, turn-bracketed view over each agent's
  NDJSON record, with field selection and search, fault events
  prominent, and discontinuities marked, never smoothed.

## What it talks to

Weaver-Web reaches the agent the way any outside consumer does: over
Unix sockets and by running its binaries. It links no crate of this
workspace. Its build surface is two contract documents under
`docs/crates/contracts/`:

- **The gate contract** (`weaver-gate-world-contract.md`) - the client
  boundary. How work enters a loaded agent and how answers return.
- **The operator contract** (`weaver-admin-operator-contract.md`) - the
  operator boundary. How the trace, the program's one output, leaves it.

Nothing else is API. The frontend never links the framework's crates,
never imports its internals, and never parses anything the contracts call
opaque. When a contract lacks something the frontend needs, the need is
filed as an ask through the contract's own change protocol rather than
worked around.

## Architecture, in one paragraph

The browser is a display engine - a hard constraint, not a preference.
Server-rendered HTML (askama) with htmx and SSE vendored into the binary;
no SPA, no node toolchain, no client-side state beyond a session cookie
and an SSE cursor. All processing is server-side Rust: a Postgres-backed
channel log with a single writer task, per-agent single-flight queues
that batch queued mentions into one turn, a mention router that routes
agent-to-agent mentions with self-invocation suppressed, a
dial-per-turn gate adapter, and per-agent trace
tailers. The user surface (gate boundary) and admin surface (operator
boundary) are separate modules behind separate route prefixes with a
role gate, so the coming IAM act attaches authentication to standing
roles instead of rearchitecting.

## Documents

- [`weaver-web-PRD.md`](../../docs/crates/weaver-web/weaver-web-PRD.md)  - 
  what and why: the surfaces, the rulings (display-engine constraint,
  scope, identity and responsibility), the roadmap, and the asks filed
  upstream.
- [`weaver-web-Spec.md`](../../docs/crates/weaver-web/weaver-web-Spec.md)  - 
  how: every representation election, cited from the code that implements
  it.
- [`deploy/`](deploy/) - the sudoers fragment, example config, and the
  verified agent-setup runbook for this box.

## Running

```sh
cargo build
weaver-web --config /etc/weaver-web/config.toml
```

Requires Postgres (local socket, peer auth; database named in config)
and, for lifecycle verbs, the scoped sudoers fragment from `deploy/`.
Agents are declared in the config and reached at their gate sockets when
loaded. See `deploy/agent-setup.md` for adding an agent to the box.

## Status

v1 is built and live-proven: real turns against real local agents,
including two agents answering one message concurrently from separate
GPUs. v1 trusts its LAN - no authentication or TLS by deliberate,
documented deferral (PRD section 6 and roadmap item 2: passkeys and TLS
land together, triggered by the surfaces being stable in daily use).

## License

Apache 2.0. See [LICENSE](LICENSE).
