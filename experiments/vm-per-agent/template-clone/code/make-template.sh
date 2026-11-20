#!/usr/bin/env bash
# make-template.sh: make one release of the appliance template from end to end, with one
# command (weaver-appliance-build-Spec section 2, "One script makes the template", the
# operator's ruling of 2026-11-19). Run it once per release, after a change merges in
# WeaverAgent or WeaverWeb. A release is a new VERSION, <release>.0, with the two commits
# it is compiled from. This header is enough to make one. The README beside code/, in its
# section "Making a release", lists each part and says when the build code has to change
# with the commits. The Spec is docs/profiles/weaver-appliance-build-Spec.md.
#
# THE COMMAND. Each value is illustrative, except OS and DRIVER_ROUTE, which take one value
# each, and DRIVER, which must be the release guest/template-steps pins:
#
#   VM_ROOT=/srv/appliance/build-6.0 OS=arch-20261115 VERSION=6.0 \
#     DRIVER=620.70.06 DRIVER_ROUTE=arch-prebuilt COMPUTE_CAP=86 \
#     WA_COMMIT=0123456789abcdef0123456789abcdef01234567 \
#     WW_COMMIT=89abcdef0123456789abcdef0123456789abcdef \
#     code/make-template.sh
#
# THE INPUTS come from the environment and pass to build-template.sh as they stand. The
# run refuses before the host tests where VM_ROOT or any of the next seven is unset, and
# build-template.sh judges each one's form.
#   VM_ROOT       a new directory for each build, made canonical. It holds this run's
#                 record, the cloud image, the build disk, the logs and the login key. A
#                 run refuses a VM_ROOT in which a run already began.
#   OS            arch-20261115, the one base this tree builds.
#   VERSION       <release>.0, such as 6.0. The release moves when the binaries move
#                 (the operator's decision 4 of 2026-11-15). This tree always compiles,
#                 so it builds releases alone, and refuses a respin, such as 6.1.
#   DRIVER        the NVIDIA driver release. The driver step refuses any release but the
#                 one guest/template-steps pins (PIN_NVIDIA, 620.70.06).
#   DRIVER_ROUTE  arch-prebuilt, the one route this tree builds.
#   COMPUTE_CAP   the compute capability the CUDA kernels are compiled for, without its
#                 dot, such as 86. The build guest has no card to read it from.
#   WA_COMMIT     the WeaverAgent commit, in full (40 hex digits).
#   WW_COMMIT     the WeaverWeb commit, in full (40 hex digits).
#   HOST_TESTS    optional: the command that runs the host tests, by default
#                 test/host/run-all.sh. A relative path is read from this script's
#                 folder. A host that confines its tests (in a sandbox of its own, for
#                 example) names its own runner here, so the host's paths stay out of the
#                 tree.
# build-template.sh's other parameters (Spec section 1: NAME, PORT, VCPUS, MEMORY, SIZE,
# CUOBJDUMP, QEMU_USER and the rest) pass through as well, each with its default.
#
# THE HOST. No root and no card. libvirt's system connection (qemu:///system) through virsh
# and virt-install, with UEFI firmware, and these on PATH: qemu-img, qemu-storage-daemon
# with FUSE (fusermount3), btrfs, blkid, ssh, scp, ssh-keygen, ssh-keyscan, gpg, curl,
# xorriso, python3, openssl, setfacl, flock, setsid, and the cuobjdump of a CUDA toolkit
# that knows the four architectures the proof checks (CUOBJDUMP). The host fetches the
# cloud image, its signed digest file and the signing key over the network, and the build
# guest downloads through QEMU's user-mode network. The build guest has 24 vCPUs, 64 GiB
# of memory and a 120G disk by default. The host tests need tools of their own. run-all.sh
# names a test skipped for a missing tool (exit 77), which does not fail the run, and its
# last line, in host-tests.out, gives the passed, failed and skipped counts. Check that the
# skipped count is the one this host expects.
#
# WHAT IT DOES, in order: the host tests, then build-template.sh's create, the guest steps
# facts to connectors, mock, the steps ready, cleanup and probes, then seal, proof and tag
# (the README lists what each does). Each part's start, end and exit status go to one log,
# VM_ROOT/appliance/make-template/make-template.log, and to the standard output. Each
# part's own output is beside the log as PART.out, with code-sha256.txt, the digests of the
# code the run is made from. build-template.sh's own step logs are in VM_ROOT/logs/appliance
# and the proof's records in VM_ROOT/logs/appliance/proof. Nothing is teed. Each part runs
# in a process group of its own, so on INT, TERM or HUP the part in hand is sent TERM whole
# (the build's own traps answer it), and the log names where the run stopped.
#
# THE END. A run that passes ends with "MADE: every part passed", then the tag's message.
# The proof's verdict is VM_ROOT/logs/appliance/proof/passed.txt: "proof passed:", with the
# sealed file's sha256 and the MANIFEST's. A run that fails stops at the first refusal,
# and the log's last line names it: "STOPPED at PART (exit N):", then that part's last
# line. PART.out holds the rest. A proof that fails lists its findings in PROOF-FAILED, in
# its records' folder. Nothing is retried, and the guest is left as it is. A new run needs
# a new VM_ROOT. A build guest still defined keeps its name, and a running one its SSH
# port, and create refuses a NAME that is defined. So the next run names a NAME and PORT
# of its own, or the guest is first removed by build-template.sh destroy, run under the
# old VM_ROOT. destroy removes that VM_ROOT/appliance whole, the old run's record in it.
#
# AFTER IT. The sealed file is VM_ROOT/appliance/appliance.qcow2: read-only, standalone
# (it names no backing file), its sha256 in VM_ROOT/logs/appliance/appliance-sha256.txt.
# A libvirt storage pool for VM_ROOT/appliance stands after the run, since virt-install
# makes one for each directory a disk path names. build-template.sh destroy removes it,
# and with it VM_ROOT/appliance whole, the sealed file included, so a sealed file to keep
# is copied out first.
# tag applies nothing and runs no git. The tag is the operator's to make in WeaverAgent and
# push, by the command tag prints (git tag -a appliance/VERSION, its message from
# VM_ROOT/logs/appliance/tag-message.txt, on WA_COMMIT).
#
# A HOST'S OWN SETTINGS (its paths, its runner) stay outside the tree. A wrapper of the
# host's own can hold them, for example, with every path and value illustrative:
#
#   #!/usr/bin/env bash
#   export VM_ROOT=/srv/appliance/build-$(date -u +%Y%m%dT%H%M%SZ)
#   export HOST_TESTS=/srv/appliance/run-host-tests CUOBJDUMP=/usr/local/cuda/bin/cuobjdump
#   export OS=arch-20261115 VERSION=6.0 DRIVER=620.70.06 DRIVER_ROUTE=arch-prebuilt
#   export COMPUTE_CAP=86 WA_COMMIT=<40 hex digits> WW_COMMIT=<40 hex digits>
#   exec /srv/src/WeaverTools/experiments/vm-per-agent/template-clone/code/make-template.sh
#
# WHAT IT DOES NOT DO. It makes no virtual machine from the sealed file (WeaverTools issue
# 49), and it changes no sealed image: nothing is installed into an image after its seal
# (the operator's direction of 2026-11-14).
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
