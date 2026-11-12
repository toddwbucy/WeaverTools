# Brief: the determinism matrix on the current stack, 2026-11-12

The suite Planner's brief to the virtualization Executor, as the first commit of
`act/matrix-current-stack`. It is restated here with every fact about the measuring host
left out. The act serves the host half of the virtual-machine measurement for
toddwbucy/WeaverWeb issue #71: the same agent served on the host and in a virtual
machine, compared by this matrix.

## Why

The matrix last ran on 2026-10-01, on the stack at WeaverAgent `9608e5b`. Read against
WeaverAgent main at `61d99dce`, it cannot drive the stack. Three things moved:

1. **The worker unit.** The harness reads `weaver-worker@<agent>.service` for the load's
   InvocationID, its control group and its journal. Admin's start step now stands each
   load in the transient scope `weaver-<agent>.scope` in `weaver.slice`, so every load's
   device read is unreadable and every session faults.
2. **The admin configuration.** The harness reads `spu-binary`, `worker-binary` and
   `gate-binary` from `admin_config` itself and judges `allow-list`, `agent-spu` and
   `spu-implementations`. Admin now takes `WEAVER_ADMIN_CONFIG` as a base and reads the
   agent's own root beneath it, and no longer reads those three files, so preflight
   refuses whatever `admin_config` names.
3. **The engine's lines.** Admin sends the worker's output to `worker.log` in the
   agent's territory, so the journal holds no device line and the cross-check is lost.

## The job

- Scope is the three breaks and nothing else:
  - the agent's scope `weaver-<agent>.scope` in place of the worker unit, for the
    InvocationID, the control group and the journal's start entry;
  - `admin_config` as the base, with the agent's root read beneath it, and the
    allow-list and SPU-map judgment removed, since admin no longer reads those files;
  - the device cross-check read from `<territory>/worker.log`.
- The matrix Spec's rules and the tests that name the old shapes change in the same act.
- The Spec says which stack shape each rule now assumes and the WeaverAgent pull
  requests that set it (#72, #166, #533), so the next move of the stack is found by
  reading, not by a failed preflight.
- Anything else found goes in a list for the Planner, not into the act.

## Read first

- `determinism-matrix-Spec.md` sections 5 and 7, and `code/README.md`.
- `code/confirm_cells.py` and `code/determinism_matrix.py`.
- In WeaverAgent, read only: `crates/weaver-admin/src/main.rs` (the loader,
  `load_service_config_from`), `crates/weaver-admin/src/scope.rs`,
  `crates/weaver-admin/src/stack.rs`, `crates/weaver-admin/src/log.rs`,
  `deploy/create-agent.sh`, and `docs/crates/weaver-admin/weaver-admin-Spec.md`
  section 9.

## Acceptance

- The experiment tests pass: plain scripts, non-zero on the first failure.
- The Executor's self-check on the diff.
- Then the branch is pushed and a draft pull request opened against `main`. That is the
  one push this brief covers. The Planner grades it.

## Constraints

- No run against the live stack that needs privilege. A dry preflight that only reads is
  allowed.
- WeaverAgent, WeaverWeb and WeaverAnalysis are not edited.
- The repositories are public: no host name, address, box path, account or group name
  of a box, or credential.
