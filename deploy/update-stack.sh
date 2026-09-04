#!/usr/bin/env bash
# Bring this box's installed agent stack to the tree's current main.
#
# Box-agnostic: every path is read from /etc/weaver/admin rather than
# written here, so the same script serves either seat.
#
#   ./deploy/update-stack.sh            plan only, no privileges, no writes
#   ./deploy/update-stack.sh --install  plan, then install what changed
#
# The plan is the point. An install that swaps every binary hides which act
# actually moved, and the campaign's comparability rests on knowing that, so
# this diffs deployed against built and installs only what differs.
set -euo pipefail

INSTALL=0
[ "${1:-}" = "--install" ] && INSTALL=1

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ADMIN_CONFIG=${WEAVER_ADMIN_CONFIG:-/etc/weaver/admin}
cd "$REPO"

say() { printf '\n== %s\n' "$*"; }
die() { printf '\nREFUSED: %s\n' "$*" >&2; exit 1; }

read_key() { cat "$ADMIN_CONFIG/$1" 2>/dev/null || true; }

# Reads a run's new trace lines on stdin and prints what the load event says
# about the two facts #419 put there. Non-zero where no load event names a
# composer, which is how the verify step knows the install took.
weaver_read_load() {
  python3 -c '
import sys, json
for line in sys.stdin:
    try:
        e = json.loads(line)
    except ValueError:
        continue
    if e.get("kind") == "load":
        p = e.get("payload", {})
        print("  state_member:", json.dumps(p.get("state_member")))
        print("  composer:    ", json.dumps(p.get("composer")))
        if p.get("state_store"):
            print("  state_store: ", json.dumps(p["state_store"]))
        sys.exit(0 if p.get("composer") else 1)
sys.exit(1)
'
}

WORKER_BINARY=$(read_key worker-binary)
AGENT_DIR=$(read_key agent-config-directory)
ALLOW_LIST=$(read_key allow-list)
[ -n "$WORKER_BINARY" ] || die "no worker-binary in $ADMIN_CONFIG"
BIN_DIR=$(dirname "$WORKER_BINARY")

# ---------------------------------------------------------------- 1. box facts
say "box"
printf '  host          %s\n' "$(hostname)"
printf '  bin dir       %s\n' "$BIN_DIR"
printf '  worker-binary %s\n' "$WORKER_BINARY"
printf '  driver        %s\n' "$(nvidia-smi --query-gpu=driver_version --format=csv,noheader 2>/dev/null || echo none)"

# The cccl window of #397. Outside it the engine does not compile, and a
# failure here is cheaper than one twenty minutes into a build.
CCCL=$(pacman -Q cccl 2>/dev/null | awk '{print $2}' || true)
printf '  cccl          %s\n' "${CCCL:-unknown}"
# **Both ends, and by version order rather than by glob.** A `3.3.*` pattern
# waves 3.3.5 through, which is outside what #397 measured, and a `3.1.4*`
# one turns away 3.1.5, which is inside it. The pacman release suffix is
# dropped before the comparison.
cccl_in_window() {
  local v=${1%%-*} lo=3.1.4 hi=3.3.4
  [ "$(printf '%s\n%s\n' "$lo" "$v" | sort -V | head -1)" = "$lo" ] &&
  [ "$(printf '%s\n%s\n' "$v" "$hi" | sort -V | head -1)" = "$v" ]
}
if [ -z "$CCCL" ]; then
  printf '  (cccl not queryable; not gating on it)\n'
elif ! cccl_in_window "$CCCL"; then
  die "cccl $CCCL is outside the 3.1.4-3.3.4 window #397 measured. Fix the pin first."
fi

# --------------------------------------------------------------- 2. update main
say "tree"
# **A failed refresh is not a stale-but-fine refresh.** Suppressing it would
# fast-forward to whatever origin/main last said and then report that commit
# as the box's, which is the kind of quiet staleness this script exists to
# remove rather than introduce.
git remote update origin >/dev/null || die "cannot refresh origin; refusing to build against a stale origin/main"
BEFORE=$(git rev-parse --short HEAD)
BRANCH=$(git branch --show-current)
if [ "$BRANCH" = "main" ]; then
  git merge --ff-only origin/main >/dev/null 2>&1 || die "main will not fast-forward; resolve by hand"
else
  printf '  on branch %s, not fast-forwarding\n' "$BRANCH"
fi
AFTER=$(git rev-parse --short HEAD)
printf '  %s -> %s\n' "$BEFORE" "$AFTER"

# ------------------------------------------------------------------- 3. build
say "build"
NVCC_CCBIN=${NVCC_CCBIN:-/usr/bin/g++-15} \
  cargo build --release --workspace \
    --features weaver-spu/cuda,weaver-harness/pyworker
printf '  ok\n'

# -------------------------------------------------------------------- 4. test
say "test"
cargo test --release -p weaver-trace -p weaver-harness -p weaver-analysis \
  --features weaver-harness/pyworker 2>&1 | grep -E '^test result' | \
  awk '{p+=$4; f+=$6} END {printf "  %d passed, %d failed\n", p, f; exit (f>0)}'

# --------------------------------------------------------------------- 5. plan
say "plan"
CHANGED=()
for b in $(ls "$BIN_DIR"); do
  [ -f "target/release/$b" ] || continue
  d=$(sha256sum "$BIN_DIR/$b" | cut -d' ' -f1)
  n=$(sha256sum "target/release/$b" | cut -d' ' -f1)
  if [ "$d" = "$n" ]; then
    printf '  %-22s unchanged\n' "$b"
  else
    printf '  %-22s CHANGED  %s -> %s\n' "$b" "${d:0:12}" "${n:0:12}"
    CHANGED+=("$b")
  fi
done
# **Binaries current is not the same as the box being finished.** A run that
# installed and then died before reconciling leaves every digest matching
# and an agent that will not load, so an early exit here would refuse to
# repair exactly the state a failed run leaves behind. Only a plan run stops
# on this.
if [ ${#CHANGED[@]} -eq 0 ]; then
  printf '  every deployed binary already matches the build\n'
  if [ "$INSTALL" -eq 0 ]; then
    say "the box is current at $AFTER"
    exit 0
  fi
fi

# ------------------------------------------------ 6. what the install implies
# **The order here was wrong once and the deadlock is worth naming.** As of
# #420 an election other than `none` requires the member's binary beside the
# worker's, and an absent `state-store` key means the default, which is the
# embedded engine and not `none`. So a declaration that never mentioned the
# store stops loading the moment that admin lands. But the *old* admin
# refuses `state-store` as an unknown field, so the fix cannot be applied
# before the install that needs it.
#
# The resolution is to stop predicting. Install, then ask the admin that is
# actually running, agent by agent, and reconcile what it refuses. `validate`
# is the oracle and it is cheap, so a later schema change gets the same
# treatment without this script having to know about it in advance.
say "what the install implies"
STATE_BINARY="$BIN_DIR/weaver-state"
if [ -f "$STATE_BINARY" ]; then
  printf '  weaver-state present: declarations may elect any engine\n'
else
  printf '  weaver-state absent: a declaration electing a store will be reconciled\n'
  printf '  to `state-store: engine: none` after the install, backed up first\n'
fi

[ "$INSTALL" -eq 1 ] || { say "plan only. rerun with --install"; exit 0; }

# ------------------------------------------------------------------ 7. install
PATCHED=()
COMPLETED=0
RESTORED=0
INSTALL_DONE=0
BACKUP=""

# **A rollback that restores only the binaries leaves the box worse than it
# found it.** The old admin refuses `state-store` as an unknown field, so a
# declaration this script patched and did not un-patch is unloadable under
# the binaries the rollback just put back. Declarations go back first.
#
# **And only the binaries this run actually replaced.** A death inside the
# install loop leaves later members never backed up and never touched, so
# restoring them from a backup that does not hold them would fail the
# restore itself.
# **The restore is best-effort across every member, not until the first
# failure.** It is the last line of defence, and `set -e` abandoning it
# halfway leaves a box in a state neither the old stack nor the new one -
# which a mid-loop test produced: the first member back, the second left
# new, and the run reporting only the failure that stopped it.
restore() {
  [ "$RESTORED" -eq 0 ] || return 0
  RESTORED=1
  local failed=0
  if [ ${#PATCHED[@]} -gt 0 ]; then
    for entry in "${PATCHED[@]}"; do
      printf '  restoring declaration %s\n' "${entry%%|*}" >&2
      cp -a "${entry##*|}" "${entry%%|*}" \
        || { printf '  FAILED to restore %s\n' "${entry%%|*}" >&2; failed=1; }
    done
  fi
  if [ "$INSTALL_DONE" -eq 1 ] && [ -n "$BACKUP" ]; then
    printf '  rolling the binaries back from %s\n' "$BACKUP" >&2
    for b in "${CHANGED[@]}"; do
      # Never backed up is never replaced, so there is nothing to put back.
      [ -f "$BACKUP/$b" ] || continue
      sudo install -o root -g root -m 0755 "$BACKUP/$b" "$BIN_DIR/$b" \
        || { printf '  FAILED to restore %s\n' "$b" >&2; failed=1; }
    done
  fi
  if [ "$failed" -ne 0 ]; then
    printf '  RESTORE INCOMPLETE - inspect %s and %s by hand\n' "$BIN_DIR" "$BACKUP" >&2
  fi
  return 0
}

rollback() {
  restore
  die "$1"
}

# **An abort that never reached `rollback` still has to restore.** The
# pipefail defect exited on `set -e`, so none of the rollback paths ran and
# the box was left with new binaries and an unloadable agent. A rollback
# reachable only from the failures this script predicted is not a rollback.
#
# **The trap is armed before the first binary moves**, not after the loop:
# a `sudo install` that fails on the fourth of six would otherwise exit with
# three replaced and nothing registered to put them back.
on_exit() {
  local rc=$?
  if [ "$COMPLETED" -eq 0 ] && { [ "$INSTALL_DONE" -eq 1 ] || [ ${#PATCHED[@]} -gt 0 ]; }; then
    printf '\n  the run did not complete (exit %d)\n' "$rc" >&2
    restore
  fi
}
trap on_exit EXIT

if [ ${#CHANGED[@]} -gt 0 ]; then
  say "install"
  # Credentials are asked for where they are needed. Reconcile and verify
  # reach admin through the NOPASSWD verbs, so a repair run that installs
  # nothing prompts for nothing.
  sudo -v
  # **Exclusive, not merely named.** A seconds-resolution name with
  # `mkdir -p` lets two runs in one second share a directory, and the
  # second run's copies would then be what the first run's rollback
  # restores. `mktemp -d` creates or fails.
  BACKUP=$(sudo mktemp -d "/opt/weaver/backup-$(date +%Y%m%dT%H%M%S)-XXXXXX")
  # State may change from the next line on, so the trap's condition is
  # armed before it does rather than after the loop closes.
  INSTALL_DONE=1
  for b in "${CHANGED[@]}"; do
    sudo cp -a "$BIN_DIR/$b" "$BACKUP/$b"
    sudo install -o root -g root -m 0755 "target/release/$b" "$BIN_DIR/$b"
    printf '  installed %s\n' "$b"
  done
  printf '  previous binaries kept at %s\n' "$BACKUP"
else
  say "install"
  printf '  nothing to install; continuing to reconcile and verify\n'
fi

# **A refusal is this function's answer, not its failure.** `weaver-admin
# validate` exits non-zero when it refuses, `tail` exits zero, and under
# `pipefail` the pipeline carries the refusal's status - so `set -e` killed
# the script at the first agent that needed reconciling, which is every
# agent this step exists for. Measured on this box 2026-09-04: the run
# printed the step's header, installed binaries already in place, and
# stopped without reconciling, verifying, or rolling back.
validate() {
  sudo -n WEAVER_ADMIN_CONFIG="$ADMIN_CONFIG" "$BIN_DIR/weaver-admin" validate "$1" 2>&1 | tail -1 || true
}

# -------------------------------------------------------- 8. reconcile agents
say "reconcile declarations"
for agent in $ALLOW_LIST; do
  decl="$AGENT_DIR/$agent.yaml"
  if [ ! -f "$decl" ]; then
    printf '  %-12s no declaration at %s\n' "$agent" "$decl"
    continue
  fi
  verdict=$(validate "$agent")
  if [ "$verdict" = '{"kind":"validated"}' ]; then
    printf '  %-12s validated\n' "$agent"
    continue
  fi
  # The one reconciliation this script knows how to make, and only where the
  # box cannot stand a leg at all. Anything else is the operator's.
  if [ ! -f "$STATE_BINARY" ] && ! grep -q '^state-store:' "$decl"; then
    printf '  %-12s %s\n' "$agent" "$verdict"
    cp -a "$decl" "$decl.pre-$AFTER-bak"
    PATCHED+=("$decl|$decl.pre-$AFTER-bak")
    printf 'state-store:\n  engine: none\n' >> "$decl"
    verdict=$(validate "$agent")
    if [ "$verdict" != '{"kind":"validated"}' ]; then
      rollback "$agent still refuses after the declaration: $verdict"
    fi
    printf '  %-12s declared `state-store: engine: none`, validated (backup %s)\n' \
      "$agent" "$(basename "$decl.pre-$AFTER-bak")"
  else
    rollback "$agent refuses and this script will not guess the fix: $verdict"
  fi
done

# -------------------------------------------------------------------- 9. verify
# A load that is not read back is an install that was not verified. This reads
# the event out of the agent's own sink, the only place the claim can be
# checked from.
# **Every agent the allow-list admits, not the first one.** Step 8 reconciles
# each of them, so verifying one and reporting the box current would leave
# the others' declarations changed and never loaded.
say "verify"
VERIFIED=0
for AGENT in $ALLOW_LIST; do
  decl="$AGENT_DIR/$AGENT.yaml"
  if [ ! -f "$decl" ]; then
    printf '  %-12s no declaration, not verified\n' "$AGENT"
    continue
  fi
  SINK=$(sed -n 's/^[[:space:]]*path:[[:space:]]*\(.*\)$/\1/p' "$decl" | head -1)
  [ -n "$SINK" ] || rollback "cannot find the trace sink for $AGENT"
  printf '  %s\n' "$AGENT"
  LINES=$(wc -l < "$SINK")
  sudo -n WEAVER_ADMIN_CONFIG="$ADMIN_CONFIG" "$BIN_DIR/weaver-admin" unload "$AGENT" >/dev/null 2>&1 || true
  sudo -n WEAVER_ADMIN_CONFIG="$ADMIN_CONFIG" "$BIN_DIR/weaver-admin" load "$AGENT" 2>&1 | tail -1 || true
  NEW=$(( $(wc -l < "$SINK") - LINES ))
  [ "$NEW" -gt 0 ] || rollback "$AGENT: the load wrote no events to $SINK"
  if ! tail -n "$NEW" "$SINK" | weaver_read_load; then
    rollback "$AGENT: the load event does not name its composer; the install did not take"
  fi
  sudo -n WEAVER_ADMIN_CONFIG="$ADMIN_CONFIG" "$BIN_DIR/weaver-admin" unload "$AGENT" >/dev/null 2>&1 || true
  VERIFIED=$((VERIFIED + 1))
done
[ "$VERIFIED" -gt 0 ] || rollback "no agent in the allow-list could be verified"

COMPLETED=1
say "the box is at $AFTER"
