#!/usr/bin/env bash
# Decision 3's tag (2026-11-15): the stack step's test count (stack_tests and stack_build's
# pipe), proof's pass line, and build-template.sh tag with stamp_one, each taken from its
# own lines. The stack log and the sealed stamp of build 5 (2026-11-15) are inlined in
# their form: build 5 sealed before the stamp carried a version. Pure logic: no root, no
# guest.
source "$(dirname "$0")/lib-test.sh"
need gawk
STEPS=$CODE/guest/template-steps; BT=$CODE/build-template.sh
# Build 5's stack log, the lines about its test step, in the form bootstrap-stack.sh wrote.
B5_STACK='== tree
   built into    /x/target/weaveragent/release
   lock          lock.sh answered 2 (0 in step, 1 drift, 2 unchecked)

== test
   output: /x/target/weaveragent/weaver-deploy-tests-46148.log
   2867 passed, 0 failed

== build
   output: /x/target/weaveragent/weaver-deploy-build-46148.log'
# Build 5's sealed stamp, the keys tag reads and the lines around them.
B5_STAMP='sealed = 2026-11-15T01:40:20Z
os = Ubuntu 26.04.1 LTS
stack.install-tuple = b5ba2fdb6ff6e238b705808febdf9d81287f8d10579969ec118b09b93d0bd1f6 (as written at image build; see the build'"'"'s note)
stack.peak-memory-mib = 12796
weaveragent.commit = be6e7e26384d26a246b7f2fa5cee624920047f41
weaveragent.origin = https://github.com/toddwbucy/WeaverAgent.git
weaverweb.commit = 6219f16cbca40b250d645994e60467f0a1793471
weaverweb.origin = https://github.com/toddwbucy/WeaverWeb.git'

# ---- stack_tests, under gawk and gawk --posix
eval "$(fn "$STEPS" stack_tests)"
check "stack_tests is in the file" eval 'declare -F stack_tests >/dev/null'
st() { printf '%b' "$1" >"$T/so"; stack_tests "$T/so" >"$T/st.out"; }
st_is() { st "$1" && [ "$(cat "$T/st.out")" = "$2" ]; }
st_no() { ! st "$1" && grep -qF -- "$2" "$T/st.out"; }
good='\n== tree\n   revision x\n\n== test\n   output: /x/weaver-deploy-tests-1.log\n   2867 passed, 0 failed\n\n== build\n   output: y\n'
for mode in plain posix; do
    if [ $mode = posix ]; then awk() { command gawk --posix "$@"; }; fi
    check "$mode: a clean count is read" st_is "$good" "2867 passed, 0 failed"
    check "$mode: a failed test refuses" st_no '\n== test\n   5 passed, 1 failed\n' "counted 5 passed, 1 failed"
    check "$mode: no test passed refuses" st_no '\n== test\n   0 passed, 0 failed\n' "counted 0 passed, 0 failed"
    check "$mode: no test section refuses" st_no '\n== build\n   output: y\n' "names no one test count"
    check "$mode: a count in another section is not read" st_no '\n== test\n   output: z\n\n== build\n   7 passed, 0 failed\n' "names no one test count"
    check "$mode: two counts refuse" st_no '\n== test\n   1 passed, 0 failed\n   2 passed, 0 failed\n' "names no one test count"
    check "$mode: a count out of form is not read" st_no '\n== test\n   12 passed, 0 failed, 3 skipped\n' "names no one test count"
    check "$mode: no file refuses" eval '! stack_tests "$T/absent" >/dev/null'
    unset -f awk
done
printf '%s\n' "$B5_STACK" >"$T/b5-stack"
check "build 5's own stack log reads 2867 passed, 0 failed" eval '[ "$(stack_tests "$T/b5-stack")" = "2867 passed, 0 failed" ]'

# ---- stack_build's pipe: bootstrap's status, never tee's, and its output kept
eval "$(fn "$STEPS" stack_build)"
fake=$T/wa; mkdir -p "$fake/deploy" "$T/home"
cuda_env() { :; }; CCCL_INCLUDE=x CXX_MAJOR=15 CUDA_LIBS=y COMPUTE_CAP=86; record() { :; }
sb() { printf '#!/usr/bin/env bash\nprintf "\\n== test\\n   9 passed, 0 failed\\n"; echo to-stderr >&2; exit %s\n' "$1" >"$fake/deploy/bootstrap-stack.sh"
       ( WA=$fake HOME=$T/home; stack_build ) >"$T/sb.out" 2>&1; }
sb 7; rc=$?
check "a bootstrap that exits 7 makes stack_build return 7" test "$rc" = 7
check "and its output is kept, stdout and stderr" eval 'grep -qx "   9 passed, 0 failed" "$T/home/stack-out" && grep -qx to-stderr "$T/home/stack-out"'
check "and still streams to the step's log" grep -qx "   9 passed, 0 failed" "$T/sb.out"
sb 0; rc=$?
check "a bootstrap that exits 0 makes stack_build return 0" test "$rc" = 0
REC='    record stack.tests "$tests (bootstrap-stack.sh'"'"'s test step)" || return 1'
STACK_FN=$(fn "$STEPS" stack); PROOF_FN=$(fn "$BT" proof)
check "stack records the count after the peak memory" eval 'grep -qxF -- "$REC" <<<"$(grep -A8 "record_read stack.peak-memory-mib" <<<"$STACK_FN")"'

# ---- proof's pass line and tag
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$BT" tag stamp_one)"
check "tag and stamp_one are in the file" eval 'declare -F tag >/dev/null && declare -F stamp_one >/dev/null'
check "proof clears an earlier pass after making its directory" eval 'grep -qx "    rm -f \"\$out/passed.txt\"" <<<"$(grep -A2 "^    mkdir -p \"\$out\"$" <<<"$PROOF_FN")"'
PASSLINE=$(sed -n '/^    echo "proof passed: sealed file sha256/,/tee "\$out\/passed.txt"$/p' "$BT")
check "proof's pass line is in the file" test -n "$PASSLINE"
LOG=$T/log; out=$LOG/proof; DISK=$T/disk.qcow2; mkdir -p "$out"
SHA=$(printf 'a%.0s' {1..64}); MSHA=$(printf 'b%.0s' {1..64}); WA=$(printf 'c%.0s' {1..40}); INST=$(printf 'd%.0s' {1..64})
echo "$SHA  $DISK" >"$LOG/appliance-sha256.txt"; echo "$MSHA  $out/MANIFEST" >"$out/identity.txt"
eval "$PASSLINE" >/dev/null
check "proof's pass line names both digests" grep -qx "proof passed: sealed file sha256 $SHA, MANIFEST sha256 $MSHA" "$out/passed.txt"
: >"$DISK"; chmod 0444 "$DISK"
stamp() { printf '%b' "sealed = x\n$1" >"$out/stamp"; }
STD="appliance.version = 1.0\nstack.install-tuple = $INST (as written at image build; see the build's note)\nstack.peak-memory-mib = 1\nstack.tests = 2867 passed, 0 failed (bootstrap-stack.sh's test step)\nweaveragent.commit = $WA\nweaverweb.commit = $(printf 'e%.0s' {1..40})\n"
tg() { ( set -euo pipefail; tag ) >"$T/tag.out" 2>&1; }
tg_no() { ! tg && grep -qF -- "$1" "$T/tag.out"; }
stamp "$STD"; tg; rc=$?
check "a proved stamp prints the tag" test "$rc" = 0
check "its name is appliance/1.0" grep -qx "  name:    appliance/1.0" "$T/tag.out"
check "on the stamp's WeaverAgent commit" grep -qx "  commit:  $WA" "$T/tag.out"
want=$(printf '%s\n' "appliance image 1.0" "" "version: 1.0" \
    "install record: /etc/weaver/stack/install-tuple, sha256 $INST, as written at image build" \
    "tests: 2867 passed, 0 failed, at bootstrap-stack.sh's test step in the build guest")
check "the message file holds the version, the digest and the count" test "$(cat "$LOG/tag-message.txt")" = "$want"
check "and is printed" grep -qx "    tests: 2867 passed, 0 failed, at bootstrap-stack.sh's test step in the build guest" "$T/tag.out"
check "with the command the operator runs" grep -qx "  made by: git tag -a appliance/1.0 -F $LOG/tag-message.txt $WA" "$T/tag.out"
check "VERSION 1.0 agrees" eval 'VERSION=1.0 tg'
check "VERSION 2.0 refuses" eval 'VERSION=2.0 tg_no "names version 1.0, not VERSION 2.0"'
check "WA_COMMIT agrees" eval 'WA_COMMIT=$WA tg'
check "another WA_COMMIT refuses" eval 'WA_COMMIT=$(printf "f%.0s" {1..40}) tg_no "not WA_COMMIT"'
rm -f "$out/passed.txt"; check "no pass refuses" tg_no "no proof has passed for this image"
echo "proof passed: sealed file sha256 $SHA" >"$out/passed.txt"; check "a pass out of form refuses" tg_no "no proof has passed"
eval "$PASSLINE" >/dev/null; echo "$INST  $DISK" >"$LOG/appliance-sha256.txt"
check "a pass of another sealed file refuses" tg_no "not the one the seal recorded"
echo "$SHA  $DISK" >"$LOG/appliance-sha256.txt"
chmod 0644 "$DISK"; check "a writable disk refuses" tg_no "no sealed image at"; chmod 0444 "$DISK"
rm -f "$out/stamp"; check "no stamp refuses" tg_no "no stamp read out of the sealed image"
TL="stack.tests = 2867 passed, 0 failed (bootstrap-stack.sh's test step)\\n"
stamp "${STD/"$TL"/}"; check "no stack.tests refuses" tg_no "names stack.tests 0 times"
stamp "${STD}stack.tests = 1 passed, 0 failed\n"; check "two stack.tests refuse" tg_no "names stack.tests 2 times"
stamp "${STD/2867 passed, 0 failed/5 passed, 1 failed}"; check "a failed count refuses" tg_no "the stamp's stack.tests is '5 passed, 1 failed"
stamp "${STD/appliance.version = 1.0/appliance.version = 1.1x}"; check "a version out of form refuses" tg_no "the stamp's appliance.version is '1.1x'"
stamp "${STD}appliance.version = 1.0\n"; check "two versions refuse" tg_no "names appliance.version 2 times"
stamp "${STD/$INST/xyz}"; check "an install digest out of form refuses" tg_no "the stamp's stack.install-tuple is 'xyz"
stamp "${STD/weaveragent.commit = $WA/weaveragent.commit = abc}"; check "a commit out of form refuses" tg_no "the stamp's weaveragent.commit is 'abc'"
stamp "${STD}stack.tests.extra = 3 passed, 0 failed\nxstack.tests = 4 passed, 0 failed\n"; check "a key that only starts or ends with the name is not read" tg
printf '%s\n' "$B5_STAMP" >"$out/stamp"; check "build 5's own stamp refuses: no version" tg_no "names appliance.version 0 times"
printf 'appliance.version = 5.0\nstack.tests = 2867 passed, 0 failed (bootstrap-stack.sh'"'"'s test step)\n' >>"$out/stamp"
check "with a version and a count added, its digest and commit are read" eval 'tg && grep -q "sha256 b5ba2fdb6ff6e238b705808febdf9d81287f8d10579969ec118b09b93d0bd1f6, as written" "$LOG/tag-message.txt" && grep -qx "  commit:  be6e7e26384d26a246b7f2fa5cee624920047f41" "$T/tag.out"'
check "tag is dispatched" grep -qx 'tag) tag ;;' "$BT"
finish
