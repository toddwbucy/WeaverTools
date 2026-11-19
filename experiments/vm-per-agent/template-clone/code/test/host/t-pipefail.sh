#!/usr/bin/env bash
# No producer piped into a reader that exits early, in the scripts the build runs
# (weaver-appliance-build-Spec section 2, the host tests): build-template.sh, lib.sh,
# guest/template-steps, make-template.sh where it stands, and the request script. Under
# pipefail such a pipeline fails with the producer's SIGPIPE (141) whenever the reader
# exits first, so a check built on it reads as failed, or, negated, as passed. The scripts
# read a command's output first and match it after, or print the first lines with sed,
# which reads its input to the end. early-exit.py finds them; checked here against
# stand-in lines first, then a control that shows the failure for real, then the scripts.
# Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need python3
EE=$CODE/test/host/early-exit.py
finds() { printf '%s\n' "$1" >"$T/s.sh"; python3 -I "$EE" "$T/s.sh" >/dev/null; [ $? = 1 ]; }
for l in 'a | grep -q x' 'a | grep -F -q x' 'a | grep -qxF -- x || rc=$?' 'x=$(a | head -1) || return 1' \
         'a | head' 'a | grep -m1 x' 'a | grep --quiet x' 'a | grep -l x' '! a 2>/dev/null | grep -qE "^b$"' \
         'b=$(a | sed -n 1p | head -2)' 'gssh p "if ss | grep -q .; then :; fi"'; do
    check "found: $l" finds "$l"
done
for l in 'a || grep -q x' 'grep -q x <<<"$v"' '# a | head -1' 'a | grep -c x' 'a | sed -n 1,5p' \
         'a | grep -E "q|head"' 'a | grep -vE "^(#|$)"' 'a |& tee f' 'v=$(a); grep -qx b <<<"$v"'; do
    check "not found: $l" eval '! finds "$l"'
done
# The control: under pipefail a producer cut by grep -q fails the pipeline with its
# SIGPIPE, while the same match on a read output does not.
rc=$(bash -o pipefail -c 'seq 1 200000 | grep -q 1; echo $?')
check "the control: seq piped into grep -q exits 141 under pipefail" test "$rc" = 141
rc=$(bash -o pipefail -c 'v=$(seq 1 200000); grep -q 1 <<<"$v"; echo $?')
check "the control: the output read first, then matched, exits 0" test "$rc" = 0
files=("$CODE/build-template.sh" "$CODE/lib.sh" "$CODE/guest/template-steps" "$CODE/guest/ready/requests")
[ -e "$CODE/make-template.sh" ] && files+=("$CODE/make-template.sh")
out=$(python3 -I "$EE" "${files[@]}"); rc=$?
check "none in the scripts the build runs (${#files[@]} files)${out:+: $out}" test "$rc" = 0
finish
