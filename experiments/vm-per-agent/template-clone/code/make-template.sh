#!/usr/bin/env bash
# Make the appliance template from end to end (weaver-appliance-build-Spec section 2, "One
# script makes the template"; the operator's ruling of 2026-11-19): the host tests, then
# build-template.sh's create, the guest steps facts to connectors, mock, the steps ready,
# cleanup and probes, then seal, proof and tag. It stops at the first refusal, naming the
# part and its last line, and leaves the guest as it is: nothing is rerun or retried here.
# tag prints the tag's message and applies nothing.
#
#   VM_ROOT=<dir> OS=arch-20261115 VERSION=<release>.0 DRIVER=<release> DRIVER_ROUTE=<route> \
#     COMPUTE_CAP=<cc> WA_COMMIT=<sha> WW_COMMIT=<sha> [HOST_TESTS=<path>] code/make-template.sh
#
# The parameters are build-template.sh's (the Spec's section 1), taken from the environment
# and passed on as they stand. It refuses, before the host tests, where VM_ROOT or an input
# create needs is unset; their forms are build-template.sh's to judge. HOST_TESTS is the
# path of the command that runs the host tests, by default test/host/run-all.sh beside this
# script; a host that confines its tests names its own runner, so no fact of a host enters
# the tree. Each part's start, end and exit status go to one log,
# VM_ROOT/appliance/make-template/make-template.log, and to the standard output, with each
# part's own output beside the log as PART.out. Nothing is teed. Each part runs in a process
# group of its own, so on INT, TERM or HUP the part in hand is sent TERM whole (the build's
# own traps answer it), and the log names where the run stopped.
set -uo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
INPUTS="OS VERSION DRIVER DRIVER_ROUTE COMPUTE_CAP WA_COMMIT WW_COMMIT"
GUEST_STEPS="facts os-probes packages rustup trees toolkit driver stack connectors"
LAST_STEPS="ready cleanup probes"

missing=""
[ -n "${VM_ROOT:-}" ] || missing+=" VM_ROOT"
for v in $INPUTS; do [ -n "${!v:-}" ] || missing+=" $v"; done
if [ -n "$missing" ]; then
    echo "REFUSED: unset:$missing. create needs each, named, never defaulted; nothing has run"
    exit 1
fi
HOST_TESTS=${HOST_TESTS:-test/host/run-all.sh}
case $HOST_TESTS in /*) ;; *) HOST_TESTS=$HERE/$HOST_TESTS ;; esac
[ -f "$HOST_TESTS" ] && [ -x "$HOST_TESTS" ] || { echo "REFUSED: HOST_TESTS, $HOST_TESTS, is not an executable file; nothing has run"; exit 1; }
VM_ROOT=$(realpath -m -- "$VM_ROOT") || { echo "REFUSED: VM_ROOT cannot be made canonical; nothing has run"; exit 1; }
export VM_ROOT
D=$VM_ROOT/appliance/make-template
# One run to a build: a second would write over the first's record, and create refuses a
# disk that stands in any case. The run's folder is taken by a plain mkdir, which fails
# where it stands, so two runs started together cannot both pass the check (CodeRabbit on
# pull request 44, 2026-11-19).
stands_msg="REFUSED: $D stands: a run already began in this VM_ROOT, and its record is kept; nothing has run"
if [ -e "$D" ] || [ -L "$D" ]; then echo "$stands_msg"; exit 1; fi
mkdir -p -- "${D%/*}" || { echo "REFUSED: ${D%/*} cannot be made; nothing has run"; exit 1; }
if ! mkdir -- "$D" 2>/dev/null; then
    if [ -e "$D" ] || [ -L "$D" ]; then echo "$stands_msg"; else echo "REFUSED: $D cannot be made; nothing has run"; fi
    exit 1
fi
LOGF=$D/make-template.log
say() { local l; l="$(date -u +%FT%TZ) $*"; printf '%s\n' "$l" >>"$LOGF"; printf '%s\n' "$l"; }

PID="" PART=""
# part NAME COMMAND...: one part, in a process group of its own and in the background, so
# that a signal is answered at once; its output to D/NAME.out; its exit status.
part() {
    local name=$1 rc=0; shift
    PART=$name; say "begin $name"
    setsid "$@" </dev/null >"$D/$name.out" 2>&1 & PID=$!
    wait "$PID" || rc=$?
    PID=""; say "end $name exit $rc"
    return "$rc"
}
# stop NAME STATUS: the run ends at its first refusal, naming the part and its last line.
stop() {
    local l
    l=$(awk 'NF { l = $0 } END { print l }' "$D/$1.out" 2>/dev/null) || l=""
    say "STOPPED at $1 (exit $2): ${l:-no output}"
    exit 1
}
# stopped SIGNAL STATUS: the part in hand is sent TERM, its whole group, and waited on.
stopped() {
    trap '' INT TERM HUP
    if [ -n "$PID" ]; then
        kill -TERM -- "-$PID" 2>/dev/null || kill -TERM "$PID" 2>/dev/null || true
        wait "$PID" 2>/dev/null || true
        say "STOPPED by SIG$1 in $PART: the part was sent TERM; its output is $D/$PART.out"
    else
        say "STOPPED by SIG$1 between parts, after $PART"
    fi
    exit "$2"
}
trap 'stopped INT 130' INT
trap 'stopped TERM 143' TERM
trap 'stopped HUP 129' HUP

say "make-template.sh: VM_ROOT=$VM_ROOT VERSION=$VERSION WA_COMMIT=$WA_COMMIT WW_COMMIT=$WW_COMMIT"
# The code this run is made from, by digest, so a run can be tied to its tree.
( cd "$HERE" && sha256sum make-template.sh build-template.sh lib.sh console-drive.py guest/template-steps \
    guest/cuda-runtime/PKGBUILD guest/ready/weaver-survey guest/ready/requests \
    guest/ready/weaver-appliance-requests.service ) >"$D/code-sha256.txt" 2>&1 || true

part host-tests "$HOST_TESTS" || stop host-tests $?
part create "$HERE/build-template.sh" create || stop create $?
for s in $GUEST_STEPS; do part "step-$s" "$HERE/build-template.sh" step "$s" || stop "step-$s" $?; done
part mock "$HERE/build-template.sh" mock || stop mock $?
for s in $LAST_STEPS; do part "step-$s" "$HERE/build-template.sh" step "$s" || stop "step-$s" $?; done
for s in seal proof tag; do part "$s" "$HERE/build-template.sh" "$s" || stop "$s" $?; done
say "MADE: every part passed; the tag's message, applied by the operator alone, follows"
cat -- "$D/tag.out"
