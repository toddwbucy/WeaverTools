#!/usr/bin/env bash
# stack's preflight, as guest/template-steps defines it: every name in DEPLOY_FILES is a
# regular file in the WeaverAgent checkout before stack_preconditions and stack_build run
# (CodeRabbit on pull request 38). stack runs against a stand-in checkout, with the two
# steps after the preflight stubbed to record their call; stack_build's stub fails, so
# stack stops there and reads no built stack. The perturbation at the end takes the
# preflight out of stack's text and shows the missing file then reaches the build, so
# the checks are live. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
TS=$CODE/guest/template-steps
f=$(fn "$TS" stack) || exit 1
eval "$f"
eval "$(grep -m1 '^DEPLOY_FILES=' "$TS")"
CALLS=$T/calls
stack_preconditions() { echo preconditions >>"$CALLS"; }
stack_build() { echo build >>"$CALLS"; return 1; }
WA=$T/wa
WA_COMMIT=0123456789abcdef0123456789abcdef01234567
mkdir -p "$WA/deploy"
for d in $DEPLOY_FILES; do echo "# $d" >"$WA/deploy/$d"; done
run() { : >"$CALLS"; out=$(stack 2>&1); rc=$?; }
calls() { tr '\n' ' ' <"$CALLS"; }

check "DEPLOY_FILES names account-drop.sh" grep -qw account-drop.sh <<<"$DEPLOY_FILES"

run
check "all present: on to stack_preconditions, then stack_build" [ "$(calls)" = "preconditions build " ]
check "all present: no refusal" eval '! grep -q "has no deploy/" <<<"$out"'

mv "$WA/deploy/account-drop.sh" "$T/account-drop.sh.away"
run
check "one missing: status 1" [ "$rc" = 1 ]
check "one missing: named, with the commit" grep -qx "WeaverAgent at $WA_COMMIT has no deploy/account-drop.sh" <<<"$out"
check "one missing: refused before any build step" [ ! -s "$CALLS" ]

mkdir "$WA/deploy/account-drop.sh"
run
check "a directory in its place: refused before any build step" \
    eval '[ "$rc" = 1 ] && [ ! -s "$CALLS" ] && grep -qx "WeaverAgent at $WA_COMMIT has no deploy/account-drop.sh" <<<"$out"'
rmdir "$WA/deploy/account-drop.sh"
mv "$T/account-drop.sh.away" "$WA/deploy/account-drop.sh"

mv "$WA/deploy/turn.py" "$T/turn.py.away"
run
check "another missing, turn.py: named, before any build step" \
    eval '[ "$rc" = 1 ] && [ ! -s "$CALLS" ] && grep -qx "WeaverAgent at $WA_COMMIT has no deploy/turn.py" <<<"$out"'
mv "$T/turn.py.away" "$WA/deploy/turn.py"

# The perturbation: without its preflight, stack lets the missing file reach the build.
mv "$WA/deploy/account-drop.sh" "$T/account-drop.sh.away"
p=$(sed '/^    local d$/,/^    done$/d' <<<"$f")
check "the perturbation takes the preflight out" eval '[ "$p" != "$f" ] && ! grep -q "has no deploy/" <<<"$p"'
eval "$p"
run
check "perturbed, account-drop.sh missing: stack_preconditions and stack_build run" [ "$(calls)" = "preconditions build " ]
mv "$T/account-drop.sh.away" "$WA/deploy/account-drop.sh"
finish
