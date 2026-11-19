#!/usr/bin/env bash
# No producer piped into a reader that exits early, in the scripts the build runs
# (weaver-appliance-build-Spec section 2, the host tests): build-template.sh, lib.sh,
# guest/template-steps, make-template.sh where it stands, and the request script, nor in
# the host tests themselves, which the build runs first. Under pipefail such a pipeline
# fails with the producer's SIGPIPE (141) whenever the reader exits first, so a check built
# on it reads as failed, or, negated, as passed. The scripts and the tests read a command's
# output first and match it after, or print the first lines with sed, which reads its input
# to the end. early-exit.py finds them; checked here against stand-in lines first, then a
# control that shows the failure for real, then the scripts and the tests. This file's own
# stand-in lines and its control each end in the mark "# stand-in", and only they are left
# out of the scan; a mark in any other file is not honoured. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need python3
EE=$CODE/test/host/early-exit.py
finds() { printf '%s\n' "$1" >"$T/s.sh"; python3 -I "$EE" "$T/s.sh" >/dev/null; [ $? = 1 ]; }
FOUND=('a | grep -q x' 'a | grep -F -q x' 'a | grep -qxF -- x || rc=$?' 'x=$(a | head -1) || return 1'  # stand-in
       'a | head' 'a | grep -m1 x' 'a | grep --quiet x' 'a | grep -l x' '! a 2>/dev/null | grep -qE "^b$"'  # stand-in
       'b=$(a | sed -n 1p | head -2)' 'gssh p "if ss | grep -q .; then :; fi"'  # stand-in
       'a | grep -e PAT -q' 'a | grep -f F -q' 'a | grep --regexp PAT --quiet' 'a | grep -ie PAT -l'  # stand-in
       'a | grep -eproof -q' 'a | grep -fF -l' 'a | grep -A 2 -q')  # stand-in
NOT_FOUND=('a || grep -q x' 'grep -q x <<<"$v"' '# a | head -1' 'a | grep -c x' 'a | sed -n 1,5p'  # stand-in
           'a | grep -E "q|head"' 'a | grep -vE "^(#|$)"' 'a |& tee f' 'v=$(a); grep -qx b <<<"$v"'  # stand-in
           'a | grep -e -q x' 'a | grep -f -l x' 'a | grep -eq x')  # stand-in
for l in "${FOUND[@]}"; do check "found: $l" finds "$l"; done
for l in "${NOT_FOUND[@]}"; do check "not found: $l" eval '! finds "$l"'; done
# The control: under pipefail a producer cut by grep -q fails the pipeline with its
# SIGPIPE, while the same match on a read output does not.
rc=$(bash -o pipefail -c 'seq 1 200000 | grep -q 1; echo $?')  # stand-in
check "the control: seq piped into grep -q exits 141 under pipefail" test "$rc" = 141
rc=$(bash -o pipefail -c 'v=$(seq 1 200000); grep -q 1 <<<"$v"; echo $?')
check "the control: the output read first, then matched, exits 0" test "$rc" = 0
# This file, its marked lines blanked so line numbers hold. Unblanked, each line the scan
# finds in it is a marked one: the mark leaves out the stand-in lines and the control alone.
SELF=$CODE/test/host/t-pipefail.sh
sed 's/.*# stand-in$//' "$SELF" >"$T/t-pipefail.sh"
own=$(python3 -I "$EE" "$SELF" | sed -n 's/^[^:]*:\([0-9]*\): .*/\1/p')
marked=$(grep -n '# stand-in$' "$SELF" | sed -n 's/:.*//p')
unmarked=$(LC_ALL=C comm -23 <(LC_ALL=C sort <<<"$own") <(LC_ALL=C sort <<<"$marked"))
check "this file unblanked: the scan finds only marked lines, and finds some" test -n "$own" -a -z "$unmarked"
files=("$CODE/build-template.sh" "$CODE/lib.sh" "$CODE/guest/template-steps" "$CODE/guest/ready/requests")
[ -e "$CODE/make-template.sh" ] && files+=("$CODE/make-template.sh")
tests=()
for f in "$CODE"/test/host/*.sh; do
    if [ "$f" = "$SELF" ]; then tests+=("$T/t-pipefail.sh"); else tests+=("$f"); fi
done
out=$(python3 -I "$EE" "${files[@]}" "${tests[@]}"); rc=$?
out=$(sed -e "s|^$T/|test/host/|" -e "s|^$CODE/||" <<<"$out")
check "none in the scripts the build runs (${#files[@]} files) or the host tests (${#tests[@]} files)${out:+: $out}" \
    test "$rc" = 0
finish
