# weaver-web - Spec

**Status:** MERGED into the set 2026-08-24 with its charter, per issue #292.
Authored under this workshop's discipline in its own tree and absorbed
rather than reauthored.

**Revised:** 2026-08-24, the absorption. The parent reference resolves
inside this tree, and three elections the absorption surfaces are recorded
at section 15 as held rather than settled: the crate pins edition 2021
against the workspace's 2024, it carries Apache-2.0 against the workspace's
UNLICENSED, and its store is the first database dependency in the tree.

**Date filed:** 2026-08-19
**Document ID:** `weaver-web-Spec`
**Parent:** `weaver-web-PRD`
**Editorial:** Per the Working Rules, as of the absorption.

This document pins representation: what the PRD's requirements become in
code. Where the PRD states a constraint, this document cites it rather
than restating its rationale. Elections still open are named in the final
section rather than left implicit.

---

## 1. Stack elections

- **Language and runtime:** Rust, async on **tokio**. The framework's own
  harness refuses tokio, but that is the framework's editorial rule for
  its own crates. weaver-web is an outside consumer and elects its own
  runtime. axum is tokio-native and the choice follows from that.
- **HTTP server:** **axum**, one process, one listener.
- **Templating:** **askama** - compile-time checked HTML templates, so a
  template referencing a field that no longer exists is a build failure
  rather than a blank region at runtime.
- **Store:** **Postgres** via **sqlx**, one database, `weaver_web`, owned
  by the operator's role. Elected on the operator's ruling of 2026-08-19:
  the suite trajectory (weaver-store, weaver-train) makes Postgres the
  engine the satellites converge on, and Postgres-to-Postgres movement is
  a dump where SQLite-to-Postgres is a migration project. Relational over
  NDJSON-per-channel because the registry, sessions, and channel logs are
  relational the moment a message references a participant. The
  connection is the local Unix socket with peer authentication - identity
  by OS uid, the box's own idiom - so no credential exists in config.
  Sharing is of the engine, never the schema: a later satellite gets its
  own database and role on the same cluster, and integration happens at
  contract boundaries, never by reading another satellite's tables.
  Single writer task regardless (section 5).
- **Browser assets:** htmx and its SSE extension **vendored into the
  binary** (`include_bytes!`), no CDN and no node toolchain. The box is
  LAN-only, a CDN reference being both a privacy leak and a broken page.
- **Serialization:** serde + serde_json everywhere a JSON boundary
  exists (gate lines, verb answers, trace events, config is TOML).

## 2. Crate layout

One binary crate, `weaver-web`, per the one-binary discipline. Modules,
not a workspace - the project splits into crates only when a second
consumer of some module exists:

    src/
      main.rs          startup: config, store open, adapters, router, axum
      config.rs        TOML config: agents, providers, listen, store path
      store.rs         Postgres pool, migrations, the single writer task
      registry.rs      participants: identity, adapter binding, policies
      channel.rs       channel log: append, read, projection types
      router.rs        invocation policy, mention parsing, dispatch
      queue.rs         per-agent queue: single-flight, batch-on-drain
      adapters/
        gate.rs        weaver agent adapter: dial, frame, close
        upstream.rs    upstream model adapter: HTTP providers
      lifecycle.rs     verb invocation via sudo, JSON answer capture
      traceview.rs     NDJSON tailers, run/turn grouping, ring buffer
      web/
        mod.rs         shared plumbing: state, sessions, errors, assets
        user.rs        the user surface: gate-boundary routes (channels)
        admin.rs       the admin surface: operator-boundary routes
                       (lifecycle, trace), behind the role gate
        templates/     askama templates: shell, channel, lifecycle, trace

## 3. Configuration

TOML, path given by `--config`, default `/etc/weaver-web/config.toml`:

    listen   = "0.0.0.0:8080"
    database = "postgres:///weaver_web?host=/run/postgresql"   # local socket, peer auth
    admins   = ["todd"]   # participant names holding the admin role

    [[agents]]
    name  = "alpha"
    gate  = "/run/weaver-alpha/gate.sock"
    trace = "/home/todd/.weaveragents/weaver-alpha/trace.out"

    [[providers]]
    name    = "claude"
    api     = "anthropic"
    model   = "claude-fable-5"
    key_env = "ANTHROPIC_API_KEY"

Agents and providers declared here are *available*, and whether one
participates in a given channel is registry state, not config. Upstream
credentials enter by environment variable only, named by `key_env`, never
stored in the config file or the database.

## 4. Store schema

Postgres, four tables, in the `weaver_web` database. Migrations are sqlx
migration files in `migrations/`, applied at startup.

    CREATE TABLE participants (
      id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
      name        TEXT NOT NULL UNIQUE,      -- mention handle, kebab-case
      display     TEXT NOT NULL,
      kind        TEXT NOT NULL,             -- 'human' | 'agent' | 'model'
      adapter     TEXT,                      -- agents: config agent name
                                             -- models: config provider name
                                             -- humans: NULL
      respond     TEXT NOT NULL DEFAULT 'mention',
                                             -- 'mention' | 'never'
      credential  BYTEA,                     -- NULL in v1; passkey public
                                             -- key in the IAM era (PRD 6)
      role        TEXT NOT NULL DEFAULT 'user'
                                             -- 'user' | 'admin'; see
                                             -- section 14
    );

    CREATE TABLE channels (
      id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
      name        TEXT NOT NULL UNIQUE,
      topic       TEXT,
      created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
    );

    CREATE TABLE members (
      channel_id     BIGINT NOT NULL REFERENCES channels(id),
      participant_id BIGINT NOT NULL REFERENCES participants(id),
      joined_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
      PRIMARY KEY (channel_id, participant_id)
    );

    CREATE TABLE channel_events (
      id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
                                             -- global monotonic, the SSE cursor
      channel_id     BIGINT NOT NULL REFERENCES channels(id),
      ts             TIMESTAMPTZ NOT NULL DEFAULT now(),
                                             -- server receipt time, the order
      participant_id BIGINT REFERENCES participants(id),
      kind           TEXT NOT NULL,
      body           TEXT,                   -- message text, or detail JSON
      run_label      TEXT,                   -- agent closes only, verbatim
      turn_label     TEXT,                   -- agent closes only, verbatim
      close_kind     TEXT                    -- 'answered'|'stopped'|'refused'
    );
    CREATE INDEX idx_events_channel ON channel_events(channel_id, id);

The identity column's monotonicity as an SSE cursor holds because all
writes flow through the single writer (section 5) - with one writer there
are no interleaved transactions to commit out of id order.

`channel_events.kind` vocabulary, closed set, unknown kinds are a defect:

| kind            | meaning                                            |
|-----------------|----------------------------------------------------|
| `message`       | a participant spoke, `body` is the text            |
| `close`         | an agent turn closed, labels and `close_kind` set  |
| `turn-open`     | an agent turn was dispatched, `body` names whom    |
| `session-open`  | a browser session attached (PRD 6), `body` detail  |
| `session-close` | a browser session ended                            |
| `member-change` | join or leave, `body` detail JSON                  |
| `app-error`     | weaver-web's own defect surfaced (PRD 4.1)         |

The run and turn labels are stored verbatim as opaque text, per the gate
contract: they are labels the program minted and this store never
interprets them beyond equality, which is all the trace link needs.

## 5. The single writer

One tokio task owns all writes. Every mutation - message posted, close
landed, member change - is a command on an mpsc channel to that task,
which appends through its own connection, then broadcasts the appended
event (with its assigned `id`) on a `tokio::sync::broadcast` per
process. SSE handlers and page renders read through an sqlx pool, and late
joiners read the table up to the broadcast's current position, then
follow. This gives the log its single ordering (the writer's) with no
write contention anywhere, and is the trace custody model restated in
code (PRD 3). Postgres LISTEN/NOTIFY is deliberately not used in v1 -
the process is its own fan-out - but it is the named seam if an
out-of-process follower of the log ever becomes legitimate, and such a
follower enters by a declared interface on weaver-web, never by SQL
grants on the tables (section 1's engine-not-schema rule).

## 6. The gate adapter

One adapter instance per configured agent.

- **Dial per turn.** The adapter connects to the gate socket when it has
  a turn to send and drops the connection after the close. Elected over a
  held connection because the contract promises a response by the path
  its request took, one turn is in flight per agent anyway (so a held
  connection buys no ordering), and dial-per-turn is self-healing across
  agent unload/load. Connection refused or absent socket is reported as
  the agent being unloaded, typed distinctly from a turn failure.
- **Framing:** one JSON object `{"text": "..."}`, newline terminated,
  UTF-8, per the contract and deployment facts. The serialized line is
  length-checked against the 32 KiB inclusive bound *after* JSON
  encoding, before write, and a violation is weaver-web's own `app-error`,
  never sent.
- **The close** is read as one line, parsed as JSON, and mapped: `kind`
  to `close_kind`, `run` and `turn` (when present) to the labels. A close
  that answers no turn - the unnamed close - is recorded as `app-error`
  per PRD 4.1, since weaver-web authors every request line and a line
  that never parsed is its own defect.
- **Timeouts:** none on the turn itself in v1. The gate serializes turns
  and a queued turn legitimately waits, so a read deadline would convert
  another client's long turn into a false failure. A socket-level error
  or EOF mid-turn is reported as delivery lost, with the trace view as
  the recovery path (the contract: the record holds the close).

## 7. Prompt serialization

The adapter builds the request `text` from the channel log, server-side
(PRD 4.1):

    [#general] Turn context. You are alpha. Messages since your last turn:
    todd: <text>
    claude: <text>
    todd: @alpha <text>

- Window: messages since the agent's last `close` in this channel,
  newest-last, truncated oldest-first to fit the line bound with the
  final mention always included.
- Speaker labels are participant `name` values. The format is a v1
  election, deliberately plain text: structured attribution is the
  named future ask (PRD 5) and is not smuggled in as ad hoc syntax.
- Batch-on-drain (PRD 4.1): all queued mentions of this agent in this
  channel land in one context block, one turn.

## 8. The queue

Per agent: an mpsc of pending invocations and a worker. The worker
drains everything pending into one batch, marks `turn-open`, runs the
gate adapter, lands the `close`. Queue depth and in-flight state are
readable (an atomic snapshot per agent) and rendered in the channel view
(PRD 4.1: queueing is visible truth). One worker per agent is the
single-flight rule made structural - there is no code path that could
send a second concurrent turn.

## 9. The router

On each appended `message` event:

1. Parse mentions: `@name` tokens matched against channel members.
2. For each mentioned participant whose `kind` is not `human` and whose
   `respond` policy admits the author: enqueue an invocation.
3. Agent-authored messages route the same way since the coordination
   change of 2026-08-20 (agents coordinate by mentioning each other),
   with two exclusions. The author itself, so a self-mention never
   self-invokes. And the hello-loop counter, added the same day after
   the first open volley greeted itself in circles: the router serves
   `agent_hop_budget` agent-to-agent hops since the last human message
   (config, default 8), then pauses the volley visibly with an app-note
   naming the count and the cure - any human word resets the budget.
   A volley also ends on its own when a message carries no mention, and
   the operator's stop and unload verbs remain the hard kill.
   Model-authored messages still trigger nothing until the upstream
   adapter exists.

`respond = 'mention'` is the only active policy in v1, and `never` exists so
an agent can be parked in a channel silently. The allowlist policy enters
with IAM-era identity, not before.

## 10. Upstream adapter

Same interface as the gate adapter (an async `turn(context) -> reply`),
implemented as one HTTP request to the configured provider (Anthropic
Messages API first). No streaming in v1 - the provider's streamed reply
is collected whole so both participant kinds present identically in the
channel, per the PRD's honesty rule: the surface does not simulate for
one participant a liveness the other cannot have. Upstream replies land
as `message` events with no run or turn labels (they have no trace).

## 11. Lifecycle runner

`tokio::process::Command`:

    sudo WEAVER_ADMIN_CONFIG=/etc/weaver/config \
      /usr/local/libexec/weaver/weaver-admin <verb> <agent>

stdout parsed as one JSON object, rendered verbatim and formatted,
non-zero exit with unparseable stdout rendered as the raw bytes and the
exit code, never swallowed. The sudoers fragment shipped in `deploy/`:

    todd ALL=(root) NOPASSWD: /usr/local/libexec/weaver/weaver-admin validate *, \
      /usr/local/libexec/weaver/weaver-admin load *, \
      /usr/local/libexec/weaver/weaver-admin unload *

Load state: the gate socket path's existence, polled on a short interval
and on demand before render, labeled as an inference in the UI (PRD 4.2).

## 12. Trace view

Per agent, a tailer task: open the NDJSON file, seek to a bounded
backfill window (last N bytes, N elected at 1 MiB), read forward, then
follow appends (inotify via `notify`, falling back to polling). Each
line parses as JSON into a tolerant shape: known envelope fields typed,
everything else retained raw. Unknown event kinds render as raw JSON
(PRD 4.3). Events group by run then turn in a bounded in-memory ring per
agent (elected: 10,000 events), because the trace surface is a live view, not a
query engine, per the PRD's retention non-goal. Loss marks render as
first-class objects.

The view carries two operator filters, both applied server-side per the
display-engine constraint and both riding the SSE stream's own URL so
the live tail obeys them: field selection (hide top-level keys or
`payload.<key>` subkeys, the list discovered from the record itself)
and a substring search over the full raw event, so a hidden field
still matches. Discontinuity marks ignore both filters - a gap in the
record is never filterable out of sight. A file rotation or truncation
(detected by inode or shrink) is surfaced in the view as a discontinuity
mark, mirroring the stream's own honesty rule.

## 13. HTTP surface

Two route families mirror the two boundaries and the two roles, per
the operator's ruling of 2026-08-19: the user surface crosses the gate,
the admin surface crosses the operator boundary, and every `/admin/*`
route sits behind the role gate. The split is structural so the IAM act
attaches authentication to existing roles rather than rearchitecting.

    GET  /                                shell, redirects to /channels
    POST /session                         open a session (v1: anonymous)
    GET  /channels                        channel list
    POST /channels                        create channel
    GET  /channels/{name}                 channel view (page)
    POST /channels/{name}/messages        post a message (form)
    POST /channels/{name}/members         add participant
    GET  /channels/{name}/stream          SSE: channel events from cursor

    GET  /admin/lifecycle                 lifecycle surface     [admin]
    POST /admin/lifecycle/{agent}/{verb}  run a verb            [admin]
    GET  /admin/trace/{agent}             trace surface         [admin]
    GET  /admin/trace/{agent}/stream      SSE: trace events     [admin]

    GET  /assets/*                        vendored static assets

SSE events are named: the channel stream emits `event: channel` with an
HTML fragment (htmx SSE swap) and `id:` set to the `channel_events.id`
cursor so `Last-Event-ID` reconnection resumes without gaps. The same
endpoint serves `Accept: application/json` with the event as JSON - the
carve-out seam (PRD 3) built as content negotiation on one stream rather
than as a second endpoint.

## 14. Sessions and roles in v1

A browser session is an anonymous server-minted cookie whose only job is
continuity: it binds a human's messages to one `human` participant
(picked or created on first visit by name). Sessions are recorded in the
`sessions` table, so the record's shape does not change when passkeys
replace anonymity - the IAM era swaps how a session is minted, not what
the record says about it.

**Roles.** Every participant carries a role, `user` or `admin`. The
admin surface is gated on it at the router. v1 assignment is the
config's `admins` list, reconciled at startup and at session open, so
the access rule stays the operator's declaration. The honest caveat,
accepted as a development-cycle fact: until the IAM act, sessions are
anonymous, so anyone on the LAN can claim an admin name. The separation
is boundary hygiene and structural readiness now, and it becomes access
control when the IAM act makes sessions prove who they are. The gate
itself does not move then - only the proof does.

## 15. Open elections

- **The context window rule** (section 7): "since the agent's last close"
  is the v1 election, and whether an agent should also receive a bounded tail
  of older context is deferred until real use shows the need.
- **The channel store's growth policy:** no pruning in v1, and an archival
  election is owed when a real channel's size makes the question concrete.
- **Provider set:** Anthropic first, the adapter trait being the seam and
  further providers are additive elections.
- **The JSON projection's shape** (section 13) is minimally specified on
  purpose, and it hardens when the carve-out triggers, not before.

**Three elections the absorption of 2026-08-24 surfaces and holds**, per
issue #292's rule that a collision between the two corpora is the operator's
rather than the absorbing act's:

- **The edition.** This crate pins `edition = "2021"` where the workspace
  carries 2024 and every other member inherits it. The migration is real
  work rather than a manifest line, the derive-heavy dependencies being
  where an edition move bites, and it was not verifiable on the machine the
  absorption ran on, which cannot resolve this crate's dependency tree. The
  pin states the fact. Aligning is owed and is its own act, with a build
  behind it.
- **The licence.** This crate carries `Apache-2.0` and its own `LICENSE`
  file where the workspace carries `UNLICENSED`. The suite's licence
  boundary is an open operator question, so the absorption declined to
  settle it by inheritance and declined equally to leave a second licence
  in the tree unremarked. Both facts stand and neither is resolved.
- **The store.** `sqlx` with `postgres` is the first database dependency in
  this workspace. A frontend holding its own store is ordinary and this one
  is this crate's alone. Whether the tree now has a database, or whether
  this stays a property of one member, is the question the absorption
  raises rather than answers.

**What the absorption did not change is the seam.** This crate links no
crate of this tree and reaches the agent exactly as an outside consumer
does: a socket dialed by path, a binary run by the operator's verb, and a
record read where the operator keeps it. Both sides being editable in one
commit is a fact about the repository and not about the boundary.
