#!/usr/bin/env bash
# Build the template guest from the Ubuntu cloud image, one step at a time, each
# step's output and status kept under logs/.
#
#   VM_ROOT=<dir> code/build-template.sh create            disk, seed, define and boot
#   VM_ROOT=<dir> code/build-template.sh ship BUNDLE       copy the WeaverAgent bundle in
#   VM_ROOT=<dir> code/build-template.sh step NAME [ARGS]  run guest/template-steps NAME
#                                                         (GUEST_ENV="VAR=value ..." sets its env)
#   VM_ROOT=<dir> code/build-template.sh seal              seal, shut down, mark read-only
#   VM_ROOT=<dir> code/build-template.sh destroy           undefine and remove the template
#
# The step names, in order, are in COMMANDS.md. A step that breaks is rerun after the
# break is recorded and got past; nothing here retries on its own.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
# shellcheck source=lib.sh
source "$HERE/lib.sh"

NAME=${NAME:-weaver-template}
PORT=${PORT:-2230}
VCPUS=${VCPUS:-16}
MEMORY=${MEMORY:-32768}
SIZE=${SIZE:-120G}
DIR=$VM_ROOT/template
DISK=$DIR/template.qcow2
LOG=${LOG:-$VM_ROOT/logs/template}
KNOWN_HOSTS=$DIR/known_hosts
mkdir -p "$LOG"

create() {
    virsh_ dominfo "$NAME" >/dev/null 2>&1 && die "$NAME is already defined"
    [ -e "$DISK" ] && die "$DISK already stands"
    fetch_image
    ensure_key
    mkdir -p "$DIR"
    guest_keys "$DIR" "$PORT"
    # A standalone disk, not an overlay on the cloud image: the sealed template is the
    # one backing file every clone names.
    qemu-img convert -O qcow2 "$IMAGE" "$DISK"
    qemu-img resize -q "$DISK" "$SIZE"
    write_seed "$DIR" "$NAME" "$NAME-$(date -u +%Y%m%dT%H%M%SZ)" "$LOG"
    define_guest "$NAME" "$DISK" "$DIR/seed.iso" "$VCPUS" "$MEMORY" "$PORT" \
        "$DIR/template_VARS.fd" >"$LOG/virt-install.txt" 2>&1
    virsh_ dumpxml "$NAME" >"$LOG/domain.xml"
    boot_ok "$PORT" "$LOG" "first-boot-$(date -u +%Y%m%dT%H%M%SZ)" 600
    gscp "$PORT" "$HERE/guest/template-steps" /tmp/template-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/template-steps /usr/local/sbin/template-steps'
}

ship() { # ship BUNDLE: the bundle reaches the guest and stays on this host's disk
    local bundle=$1
    [ -f "$bundle" ] || die "no bundle at $bundle"
    gssh "$PORT" 'mkdir -p ~/bundles'
    gscp "$PORT" "$bundle" "bundles/$(basename "$bundle")"
    gssh "$PORT" "sha256sum ~/bundles/$(basename "$bundle")" | tee "$LOG/bundle.txt"
    sha256sum "$bundle" | tee -a "$LOG/bundle.txt"
}

step() {
    # Refresh the guest's copy of the steps first, so a step edited after create runs.
    gscp "$PORT" "$HERE/guest/template-steps" /tmp/template-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/template-steps /usr/local/sbin/template-steps'
    # GUEST_ENV, where set, is given to the step as its environment (env VAR=value ...),
    # so a workaround a step needed is named on the command line that ran it.
    run_step "$PORT" "$LOG" "$1" "${GUEST_ENV:+env $GUEST_ENV }template-steps $*"
}

seal() {
    gscp "$PORT" "$HERE/guest/template-steps" /tmp/template-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/template-steps /usr/local/sbin/template-steps'
    run_step "$PORT" "$LOG" seal-identity "template-steps seal"
    tail -3 "$LOG/seal-identity.txt"
    # The guest powers itself off at the end of the seal step.
    local t=0
    until [ "$(virsh_ domstate "$NAME")" = "shut off" ]; do
        t=$((t + 5)); [ "$t" -lt 300 ] || die "$NAME did not shut off"; sleep 5
    done
    virsh_ dumpxml "$NAME" >"$LOG/domain-sealed.xml"
    # The sealed template is never booted again: its domain is undefined and the disk
    # made read-only, so every clone is an overlay naming it.
    virsh_ undefine "$NAME" --nvram >/dev/null
    chmod 0444 "$DISK"
    # The build guest's host key pair, its pin and its seed image, which carries the private
    # key, go with it: the sealed template is never booted again.
    rm -rf -- "${DIR:?}/hostkey" "${DIR:?}/known_hosts" "${DIR:?}/seed.iso"
    sha256sum "$DISK" | tee "$LOG/template-sha256.txt"
    qemu-img info "$DISK" >"$LOG/template-qemu-img-info.txt"
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
ship) shift; ship "$@" ;;
step) shift; step "$@" ;;
seal) seal ;;
destroy) destroy ;;
*) sed -n '2,13p' "$0" >&2; exit 2 ;;
esac
