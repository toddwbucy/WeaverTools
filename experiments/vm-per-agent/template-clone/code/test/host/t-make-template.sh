#!/usr/bin/env bash
# make-template.sh (weaver-appliance-build-Spec section 2, "One script makes the template")
# with build-template.sh and the host tests stood in: a copy of the script beside a stand-in
# build-template.sh that records each call, refuses where it is told to, or waits on a
# child until it is stopped. Checked: the parts' order, each part's output beside the one
# log, the inputs passed on, the stop at the first refusal naming the part and its last
# line, the refusal of an unset input before the host tests, the refusal of a second run
# into the same VM_ROOT, HOST_TESTS and its default, and a run stopped by TERM sending the
# part in hand TERM whole. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need setsid pgrep python3
C=$T/code; mkdir -p "$C/test/host"
cp "$CODE/make-template.sh" "$C/"
export CALLS=$T/calls SEEN=$T/seen FAIL_AT=$T/fail-at HANG_AT=$T/hang-at READY=$T/ready TRAPPED=$T/trapped
export MARK="make-template-hang-$$"
cat >"$C/build-template.sh" <<'SH'
#!/usr/bin/env bash
# The stand-in build-template.sh: records its call, refuses the call FAIL_AT names, or for
# the call HANG_AT names waits on a child until stopped, with a TERM trap of its own that
# bash defers until the child ends, as the build's traps are deferred.
echo "$*" >>"$CALLS"
[ "$1" != create ] || printf '%s\n' "VERSION=$VERSION WA_COMMIT=$WA_COMMIT VM_ROOT=$VM_ROOT" >"$SEEN"
echo "== the stand-in's $* ran"
if [ -e "$FAIL_AT" ] && [ "$(cat "$FAIL_AT")" = "$*" ]; then printf '\nREFUSED: the stand-in refuses %s\n\n' "$*" >&2; exit 1; fi
if [ -e "$HANG_AT" ] && [ "$(cat "$HANG_AT")" = "$*" ]; then
    trap 'echo "its trap ran" >"$TRAPPED"; exit 143' TERM
    printf '%s %s\n' "$$" "$(ps -o pgid= -p $$ | tr -d ' ')" >"$READY.part" && mv -- "$READY.part" "$READY"
    python3 -I -c 'import time; time.sleep(60)' "$MARK"
    echo "the child ended by itself"
fi
[ "$1" != tag ] || echo "the tag's message: appliance/5.0 (stand-in)"
SH
printf '#!/usr/bin/env bash\necho host-tests >>"$CALLS"; echo "host tests: run"\n[ ! -e "%s" ] || { echo "== 26 passed, 2 failed"; exit 3; }\n' "$T/ht-fail" >"$T/ht.sh"
printf '#!/usr/bin/env bash\necho "run-all (the default)" >>"$CALLS"\n' >"$C/test/host/run-all.sh"
chmod 0755 "$C/build-template.sh" "$T/ht.sh" "$C/test/host/run-all.sh"
export OS=arch-20261115 VERSION=5.0 DRIVER=620.70.06 DRIVER_ROUTE=arch-prebuilt COMPUTE_CAP=86
export WA_COMMIT=1111111111111111111111111111111111111111 WW_COMMIT=2222222222222222222222222222222222222222
PARTS="host-tests create step-facts step-os-probes step-packages step-rustup step-trees step-toolkit step-driver step-stack step-connectors mock step-ready step-cleanup step-probes seal proof tag"
WANT='host-tests
create
step facts
step os-probes
step packages
step rustup
step trees
step toolkit
step driver
step stack
step connectors
mock
step ready
step cleanup
step probes
seal
proof
tag'
n=0
# mt [ENV-OPTION...]: one run into a new VM_ROOT (or VR_GIVEN's), env's options (-u NAME)
# first, its output in T/mt.out.
mt() { n=$((n + 1)); VR=${VR_GIVEN:-$T/vr$n}; rm -f -- "$CALLS"
    env "$@" VM_ROOT="$VR" HOST_TESTS="$T/ht.sh" bash "$C/make-template.sh" >"$T/mt.out" 2>&1; rc=$?
    L=$VR/appliance/make-template; }
calls() { cat "$CALLS" 2>/dev/null; }

mt
check "a run where every part passes: exit 0" test "$rc" = 0
check "the parts in the Spec's order: the host tests, create, facts to connectors, mock, ready, cleanup, probes, seal, proof, tag" \
    test "$(calls)" = "$WANT"
check "each part's begin and its end with exit 0, in order, in the one log" \
    eval '[ "$(sed -n "s/^[0-9TZ:-]* \(begin\|end\) \([a-z-]*\).*/\1 \2/p" "$L/make-template.log")" = "$(for p in $PARTS; do echo "begin $p"; echo "end $p"; done)" ] && [ "$(grep -c " end [a-z-]* exit 0$" "$L/make-template.log")" = 18 ]'
check "each part's output beside the log" \
    eval '( for p in $PARTS; do [ -s "$L/$p.out" ] || exit 1; done ) && grep -qx "== the stand-in.s step facts ran" "$L/step-facts.out" && grep -qx "host tests: run" "$L/host-tests.out"'
check "the log ends MADE, and the standard output ends with the tag's message" \
    eval 'grep -q " MADE: every part passed" <<<"$(sed -n "\$p" "$L/make-template.log")" && [ "$(sed -n "\$p" "$T/mt.out")" = "the tag'"'"'s message: appliance/5.0 (stand-in)" ]'
check "the inputs are passed on as they stand, VM_ROOT made canonical" \
    test "$(cat "$SEEN")" = "VERSION=5.0 WA_COMMIT=$WA_COMMIT VM_ROOT=$T/vr$n"
check "the code's digests are kept with the log" grep -q '  make-template.sh$' "$L/code-sha256.txt"

echo "step driver" >"$FAIL_AT"; mt; rm -f -- "$FAIL_AT"
check "a refusal at the driver step stops the run there: exit 1, nothing after it" \
    test "$rc" = 1 -a "$(calls | sed -n '$p')" = "step driver" -a "$(calls | wc -l)" = 9
check "and names the part and its last line, in the log and on the standard output" \
    eval '[ "$(sed -n "\$p" "$L/make-template.log" | cut -d" " -f2-)" = "STOPPED at step-driver (exit 1): REFUSED: the stand-in refuses step driver" ] && grep -q " STOPPED at step-driver (exit 1): REFUSED: the stand-in refuses step driver$" "$T/mt.out"'
echo proof >"$FAIL_AT"; mt; rm -f -- "$FAIL_AT"
check "a proof that fails: seal ran, tag did not" test "$rc" = 1 -a "$(calls | sed -n '$p')" = proof -a "$(calls | grep -c '^seal$')" = 1
touch "$T/ht-fail"; mt; rm -f -- "$T/ht-fail"
check "host tests that fail stop the run before create, naming their last line" \
    eval '[ "$rc" = 1 ] && [ "$(calls)" = host-tests ] && grep -q " STOPPED at host-tests (exit 3): == 26 passed, 2 failed$" "$L/make-template.log"'

mt -u WW_COMMIT
check "an input create needs, unset: refused naming it, before the host tests, nothing made" \
    eval '[ "$rc" = 1 ] && [ -z "$(calls)" ] && [ ! -e "$VR" ] && grep -qx "REFUSED: unset: WW_COMMIT. create needs each, named, never defaulted; nothing has run" "$T/mt.out"'
rm -f -- "$CALLS"; env -u VM_ROOT -u OS -u VERSION -u DRIVER -u DRIVER_ROUTE -u COMPUTE_CAP -u WA_COMMIT -u WW_COMMIT \
    HOST_TESTS="$T/ht.sh" bash "$C/make-template.sh" >"$T/mt.out" 2>&1; rc=$?
check "all unset: each named, VM_ROOT first" \
    eval '[ "$rc" = 1 ] && [ -z "$(calls)" ] && grep -qx "REFUSED: unset: VM_ROOT OS VERSION DRIVER DRIVER_ROUTE COMPUTE_CAP WA_COMMIT WW_COMMIT. create needs each, named, never defaulted; nothing has run" "$T/mt.out"'

mt; first=$(sha256sum <"$L/make-template.log"); VR_GIVEN=$VR mt
check "a second run into the same VM_ROOT is refused, and the first's record kept" \
    eval '[ "$rc" = 1 ] && [ -z "$(calls)" ] && grep -q "^REFUSED: $L stands: a run already began in this VM_ROOT" "$T/mt.out" && [ "$(sha256sum <"$L/make-template.log")" = "$first" ]'
chmod 0644 "$T/ht.sh"; mt; chmod 0755 "$T/ht.sh"
check "HOST_TESTS that is not an executable file: refused, nothing run" \
    eval '[ "$rc" = 1 ] && [ -z "$(calls)" ] && grep -q "^REFUSED: HOST_TESTS, $T/ht.sh, is not an executable file" "$T/mt.out"'
n=$((n + 1)); rm -f -- "$CALLS"; env VM_ROOT="$T/vr$n" bash "$C/make-template.sh" >"$T/mt.out" 2>&1; rc=$?
check "HOST_TESTS unset: test/host/run-all.sh beside the script" test "$rc" = 0 -a "$(calls | sed -n 1p)" = "run-all (the default)"

# Stopped by TERM while the trees step waits on its child.
echo "step trees" >"$HANG_AT"; n=$((n + 1)); L=$T/vr$n/appliance/make-template; rm -f -- "$CALLS" "$READY" "$TRAPPED"
env VM_ROOT="$T/vr$n" HOST_TESTS="$T/ht.sh" bash "$C/make-template.sh" >"$T/mt.out" 2>&1 &
mp=$!
if wait_for "$mp" 20 test -s "$READY"; then
    t0=$(date +%s); kill -TERM "$mp"; wait "$mp"; rc=$?; secs=$(( $(date +%s) - t0 ))
else
    rc=none secs=none; kill -TERM "$mp" 2>/dev/null; wait "$mp" 2>/dev/null
fi
rm -f -- "$HANG_AT"
left=$(pgrep -f -- "$MARK" || true)
check "a run sent TERM in a part exits 143 within 20 s, not after the part's child" \
    eval '[ "$rc" = 143 ] && [ "$secs" -lt 20 ]'
check "the part ran in a process group of its own" eval 'read -r p g <"$READY"; [ -n "$p" ] && [ "$p" = "$g" ]'
check "its whole group was sent TERM: the child is gone and the part's own trap ran" \
    eval '[ -z "$left" ] && [ "$(cat "$TRAPPED" 2>/dev/null)" = "its trap ran" ]'
check "the log names where the run stopped, and nothing ran after the part" \
    eval 'grep -q " STOPPED by SIGTERM in step-trees: the part was sent TERM; its output is $L/step-trees.out$" "$L/make-template.log" && [ "$(calls | sed -n "\$p")" = "step trees" ]'
[ -z "$left" ] || kill $left || true

check "nothing is teed" bash -c '! grep -qwE "tee" <(grep -v "^[[:space:]]*#" "$1")' _ "$CODE/make-template.sh"
finish
