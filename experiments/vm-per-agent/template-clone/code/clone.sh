#!/usr/bin/env bash
# Make one clone of the sealed template, an overlay naming it, with its own seed,
# firmware variables and SSH port, and the host's models offered read-only over
# virtiofs; then run guest/clone-steps in it, one step at a time.
#
#   VM_ROOT=<dir> MODELS_DIR=<dir> code/clone.sh create
#   VM_ROOT=<dir> code/clone.sh step NAME [ARGS]    (GUEST_ENV="VAR=value ..." sets its env)
#   VM_ROOT=<dir> code/clone.sh virtiofsd       the host side of the share, as running
#   VM_ROOT=<dir> code/clone.sh netdev VALUE    shut down, set the -netdev argument, start
#   VM_ROOT=<dir> code/clone.sh destroy
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
# shellcheck source=lib.sh
source "$HERE/lib.sh"

NAME=${NAME:-weaver-clone1}
PORT=${PORT:-2231}
VCPUS=${VCPUS:-12}
MEMORY=${MEMORY:-24576}
TEMPLATE=$VM_ROOT/template/template.qcow2
DIR=$VM_ROOT/clones/$NAME
DISK=$DIR/disk.qcow2
LOG=${LOG:-$VM_ROOT/logs/$NAME}
mkdir -p "$LOG"

create() {
    : "${MODELS_DIR:?set MODELS_DIR to the host directory holding the model artifacts}"
    virsh_ dominfo "$NAME" >/dev/null 2>&1 && die "$NAME is already defined"
    [ -r "$TEMPLATE" ] && [ ! -w "$TEMPLATE" ] || die "$TEMPLATE is not a sealed, read-only template"
    ensure_key
    mkdir -p "$DIR"
    qemu-img create -q -f qcow2 -F qcow2 -b "$TEMPLATE" "$DISK"
    write_seed "$DIR" "$NAME" "$NAME-$(date -u +%Y%m%dT%H%M%SZ)"
    define_guest "$NAME" "$DISK" "$DIR/seed.iso" "$VCPUS" "$MEMORY" "$PORT" "$DIR/VARS.fd" \
        --memorybacking source.type=memfd,access.mode=shared \
        --filesystem "type=mount,accessmode=passthrough,driver.type=virtiofs,source.dir=$MODELS_DIR,target.dir=weaver-models,readonly=on" \
        >"$LOG/virt-install.txt" 2>&1
    virsh_ dumpxml "$NAME" >"$LOG/domain.xml"
    virtiofsd_args | tee "$LOG/virtiofsd.txt"
    wait_ssh "$PORT" 600 | tee "$LOG/first-boot.txt"
    gscp "$PORT" "$HERE/guest/clone-steps" /tmp/clone-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/clone-steps /usr/local/sbin/clone-steps'
}

# The share's host side: the virtiofsd libvirt started for this domain, and whether it
# was told to refuse writes. libvirt names neither the domain nor its socket on
# virtiofsd's command line (it hands over an fd), so it is found by the directory the
# domain shares.
virtiofsd_args() {
    local pids src
    src=$(virsh_ dumpxml "$NAME" | sed -n "/<filesystem/,/<\/filesystem>/s/.*<source dir='\([^']*\)'.*/\1/p" | head -1)
    [ -n "$src" ] || { echo "no filesystem in $NAME's domain"; return 0; }
    pids=$(pgrep -f "virtiofsd .*--shared-dir $src( |\$)" || true)
    [ -n "$pids" ] || { echo "no virtiofsd sharing $src"; return 0; }
    for p in $pids; do
        printf 'virtiofsd pid %s user %s: %s\n' "$p" "$(ps -o user= -p "$p")" \
            "$(tr '\0' ' ' <"/proc/$p/cmdline")"
    done
}

step() {
    gscp "$PORT" "$HERE/guest/clone-steps" /tmp/clone-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/clone-steps /usr/local/sbin/clone-steps'
    # GUEST_ENV, as in build-template.sh: the step's environment, named on its command line.
    run_step "$PORT" "$LOG" "$1" "${GUEST_ENV:+env $GUEST_ENV }clone-steps $*"
}

# netdev VALUE: the clone's user-mode -netdev argument replaced by VALUE (it must keep
# id=un0 and the SSH hostfwd), the domain shut down first and started after, so the
# loopback probe can be run against each backend option.
netdev() {
    local value=$1 x t=0
    case $value in user,id=un0,*hostfwd=*) ;; *) die "VALUE must start user,id=un0, and keep the SSH hostfwd" ;; esac
    if [ "$(virsh_ domstate "$NAME")" != "shut off" ]; then
        virsh_ shutdown "$NAME" >/dev/null
        until [ "$(virsh_ domstate "$NAME")" = "shut off" ]; do
            t=$((t + 3)); [ "$t" -lt 180 ] || die "$NAME did not shut off"; sleep 3
        done
    fi
    x=$(mktemp)
    virsh_ dumpxml --inactive "$NAME" >"$x"
    python3 - "$x" "$value" <<'PY'
import re, sys
p, val = sys.argv[1], sys.argv[2]
s = open(p).read()
s, n = re.subn(r"<qemu:arg value='user,id=un0[^']*'/>", "<qemu:arg value='%s'/>" % val, s)
if n != 1:
    sys.exit("expected one user-mode -netdev argument, found %d" % n)
open(p, "w").write(s)
PY
    virsh_ define "$x" >/dev/null; rm -f "$x"
    virsh_ dumpxml --inactive "$NAME" | grep "qemu:arg value='user" | tee -a "$LOG/netdev.txt"
    virsh_ start "$NAME" >/dev/null
    wait_ssh "$PORT" 300
}

destroy() {
    if virsh_ dominfo "$NAME" >/dev/null 2>&1; then
        virsh_ destroy "$NAME" >/dev/null 2>&1 || true
        virsh_ undefine "$NAME" --nvram >/dev/null
    fi
    rm -rf "$DIR"
}

case ${1:-} in
create) create ;;
step) shift; step "$@" ;;
virtiofsd) virtiofsd_args ;;
netdev) shift; netdev "$1" ;;
destroy) destroy ;;
*) sed -n '2,11p' "$0" >&2; exit 2 ;;
esac
