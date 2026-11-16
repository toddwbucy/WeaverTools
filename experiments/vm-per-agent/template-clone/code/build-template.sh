#!/usr/bin/env bash
# Build the appliance image from the Ubuntu cloud image, one step at a time, each step's
# output and status kept under logs/. The binaries are compiled inside this guest from the
# two named commits; nothing built on the host enters the image
# (weaver-appliance-build-Spec, section 2).
#
#   VM_ROOT=<dir> OS=26.04 VERSION=<release>.0 DRIVER=<release> DRIVER_ROUTE=<route> \
#     COMPUTE_CAP=<cc> WA_COMMIT=<sha> WW_COMMIT=<sha> code/build-template.sh create
#                                              disk, seed, define and boot
#   ... code/build-template.sh step NAME [ARGS]  run guest/template-steps NAME
#                                              (GUEST_ENV="VAR=value ..." adds to its env)
#   ... code/build-template.sh mock              the offline mock, run inside the build guest
#   ... code/build-template.sh seal              seal, shut down, read the verdict, mark
#                                              read-only
#   ... code/build-template.sh verdict           read the verdict again from the shut-off
#                                              guest, where seal's own read failed
#   ... code/build-template.sh proof             read the sealed image; boot an overlay
#                                              with no seed, then one as a first boot,
#                                              with a key and a network placed in it
#   ... code/build-template.sh tag               print the version's tag for the operator,
#                                              after a proof that passed; apply nothing
#   ... code/build-template.sh destroy           undefine and remove the build guest
#
# The seven inputs are named, never defaulted: OS, VERSION (the image's version,
# <release>.<respin>), DRIVER (the NVIDIA driver release), DRIVER_ROUTE (the route it comes
# by), COMPUTE_CAP (the compute capability the CUDA kernels are compiled for, without its
# dot), WA_COMMIT and WW_COMMIT (the WeaverAgent and WeaverWeb commits, in full). create
# needs all seven. Every step is given those that are
# set, and a guest step that needs one refuses where it is not. The step names, in order,
# are in COMMANDS.md. A step that breaks is rerun after the break is recorded and got
# past; nothing here retries.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
OS_GIVEN=${OS:-}
# shellcheck source=lib.sh
source "$HERE/lib.sh"

NAME=${NAME:-weaver-appliance-build}
PORT=${PORT:-2230}
# The build's own size: 24 vCPUs for the compile (the operator's word of 2026-11-14). A
# box is later run under the operator's own definition.
VCPUS=${VCPUS:-24}
MEMORY=${MEMORY:-65536}
SIZE=${SIZE:-120G}
# The appliance image has its own path and name, apart from the #21 template's
# ($VM_ROOT/template/template.qcow2), which clone.sh reads and which this script no longer
# builds (template-clone-Spec.md, section 0).
DIR=$VM_ROOT/appliance
DISK=$DIR/appliance.qcow2
LOG=${LOG:-$VM_ROOT/logs/appliance}
KNOWN_HOSTS=$DIR/known_hosts
mkdir -p "$LOG"

# inputs_ok all|set: each input that is set has its form; with "all", each is set.
inputs_ok() {
    if [ "$1" = all ]; then
        [ -n "$OS_GIVEN" ] || die "name OS (26.04)"
        local v; for v in VERSION DRIVER DRIVER_ROUTE COMPUTE_CAP WA_COMMIT WW_COMMIT; do [ -n "${!v:-}" ] || die "name $v"; done
    fi
    [ -z "${WA_COMMIT:-}" ] || [[ "$WA_COMMIT" =~ ^[0-9a-f]{40}$ ]] || die "WA_COMMIT is a commit in full, 40 hex digits"
    [ -z "${WW_COMMIT:-}" ] || [[ "$WW_COMMIT" =~ ^[0-9a-f]{40}$ ]] || die "WW_COMMIT is a commit in full, 40 hex digits"
    # One form for DRIVER, the same regex guest/template-steps' driver step checks.
    [ -z "${DRIVER:-}" ] || [[ "$DRIVER" =~ ^[0-9]{3}\.[0-9]+(\.[0-9]+)?$ ]] || die "DRIVER is a driver release such as 615.71.09"
    [ -z "${DRIVER_ROUTE:-}" ] || [ "$DRIVER_ROUTE" = ubuntu-prebuilt ] || die "DRIVER_ROUTE is a route this tree builds: ubuntu-prebuilt"
    [ -z "${COMPUTE_CAP:-}" ] || [[ "$COMPUTE_CAP" =~ ^[0-9]{2,3}$ ]] || die "COMPUTE_CAP is a compute capability without its dot, such as 86"
    # VERSION is <release>.<respin> (the operator's decision 4 of 2026-11-15): the release
    # moves when the binaries move, the respin when only the OS layer moves. A respin
    # carries a release's binaries into a new OS layer and never recompiles. This tree
    # always compiles, so it builds a release alone, respin 0.
    [ -z "${VERSION:-}" ] || [[ "$VERSION" =~ ^[1-9][0-9]*\.(0|[1-9][0-9]*)$ ]] || die "VERSION is <release>.<respin>, such as 1.0"
    [ -z "${VERSION:-}" ] || [[ "$VERSION" =~ \.0$ ]] || die "VERSION $VERSION is a respin. A respin carries a release's binaries into a new OS layer and never recompiles, and no path here does that: this tree compiles, so it builds a release, VERSION <release>.0"
}
# inputs_env: the guest command's environment prefix: the inputs that are set, and the base
# image by name and digest, which the stamp records.
inputs_env() {
    BASE_IMAGE="$IMAGE_NAME $IMAGE_SHA256"
    guest_env VERSION WA_COMMIT WW_COMMIT DRIVER DRIVER_ROUTE COMPUTE_CAP BASE_IMAGE
}
steps_in() {
    gscp "$PORT" "$HERE/guest/template-steps" /tmp/template-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/template-steps /usr/local/sbin/template-steps'
}

create() {
    inputs_ok all
    virsh_ dominfo "$NAME" >/dev/null 2>&1 && die "$NAME is already defined"
    # The build guest's name must differ from the image's, or the seal cannot tell them apart.
    [ "$NAME" != "$IMAGE_HOSTNAME" ] || die "NAME is the image's own hostname, $IMAGE_HOSTNAME; name the build guest otherwise"
    [ -e "$DISK" ] && die "$DISK already stands"
    fetch_image
    ensure_key
    mkdir -p "$DIR"
    guest_keys "$DIR" "$PORT"
    # A standalone disk, not an overlay on the cloud image: the sealed image names no
    # backing file.
    qemu-img convert -O qcow2 "$IMAGE" "$DISK"
    qemu-img resize -q "$DISK" "$SIZE"
    write_seed "$DIR" "$NAME" "$NAME-$(date -u +%Y%m%dT%H%M%SZ)" "$LOG"
    define_guest "$NAME" "$DISK" "$DIR/seed.iso" "$VCPUS" "$MEMORY" "$PORT" \
        "$DIR/appliance_VARS.fd" >"$LOG/virt-install.txt" 2>&1
    virsh_ dumpxml "$NAME" >"$LOG/domain.xml"
    boot_ok "$PORT" "$LOG" "first-boot-$(date -u +%Y%m%dT%H%M%SZ)" 600
    steps_in
}

step() {
    inputs_ok set
    # Refresh the guest's copy of the steps first, so a step edited after create runs.
    steps_in
    # The inputs, and GUEST_ENV where set, are the step's environment (env VAR=value ...),
    # so what a step was given is named on the command line that ran it.
    run_step "$PORT" "$LOG" "$1" "$(inputs_env)${GUEST_ENV:+env $GUEST_ENV }template-steps $*"
    # run_step never stops its caller; a failed guest step stops here, after its log is kept.
    step_ok "$LOG" "$1" || die "step $1 did not pass; see $LOG/$1.txt"
}

seal() {
    inputs_ok set
    [ -n "${WA_COMMIT:-}" ] && [ -n "${WW_COMMIT:-}" ] && [ -n "${VERSION:-}" ] || die "name WA_COMMIT, WW_COMMIT and VERSION: the verdict sets the stamp's commits and version against them"
    steps_in
    run_step "$PORT" "$LOG" seal-identity "$(inputs_env)template-steps seal"
    tail -3 "$LOG/seal-identity.txt"
    # A seal that did not pass is never followed by the undefine, the read-only disk or the
    # checksum.
    step_ok "$LOG" seal-identity || die "the seal did not pass; see $LOG/seal-identity.txt"
    # The guest's final step runs as root after the SSH session ends: it removes the build
    # account, re-runs the probes, writes the stamp and the MANIFEST last, and powers off.
    # Its verdict is read from the sealed image by `proof`.
    local t=0
    until [ "$(virsh_ domstate "$NAME")" = "shut off" ]; do
        t=$((t + 5)); [ "$t" -lt "${SEAL_WAIT:-1200}" ] || die "$NAME did not shut off"; sleep 5
    done
    echo "shut off after ${t}s"
    virsh_ dumpxml "$NAME" >"$LOG/domain-sealed.xml"
    seal_verdict
}

# seal_verdict: the final step's verdict, read out of the shut-off guest's disk before
# anything marks it sealed: a MANIFEST, no SEAL-REFUSED, and a stamp naming both commits.
# A refused or incomplete seal stops here, with the domain defined and the disk writable,
# for destroy. A read that fails is told apart from a file that is absent and stops here
# too, with nothing changed: seal cannot be run again (the guest is off and its account
# gone), so `build-template.sh verdict` runs this read again on its own.
seal_verdict() {
    inputs_ok set
    [ -n "${WA_COMMIT:-}" ] && [ -n "${WW_COMMIT:-}" ] && [ -n "${VERSION:-}" ] || die "name WA_COMMIT, WW_COMMIT and VERSION: the verdict sets the stamp's commits and version against them"
    [ "$(virsh_ domstate "$NAME" 2>/dev/null)" = "shut off" ] || die "$NAME is not a defined, shut-off build guest: the verdict reads a powered-off guest's disk"
    # The verdict ends in an undefine, so the domain must be the build guest: one of its
    # disks is DISK, by dom_drop's rule. A shut-off domain of that name on another disk is
    # refused, and nothing is read or changed.
    local on=0; dom_on_disk "$NAME" "$DISK" || on=$?
    case $on in
        0) ;;
        1) die "$NAME is not the build guest: none of its disks is $DISK. Nothing is read or changed" ;;
        *) die "$NAME's definition could not be read or parsed. Nothing is read or changed" ;;
    esac
    [ -f "$DISK" ] && [ -w "$DISK" ] || die "$DISK is not a writable disk: absent, or already marked sealed"
    local r=$VM_ROOT/seal-read v rc
    [ ! -e "$r" ] || die "$r stands: another seal or verdict is reading the image, or one was cut short (destroy removes it)"
    mkdir -- "$r"
    trap 'trap "" INT TERM HUP PIPE; image_close "$VM_ROOT/seal-read"; rm -rf -- "${VM_ROOT:?}/seal-read" || true' EXIT
    trap 'exit 130' INT; trap 'exit 143' TERM; trap 'exit 129' HUP
    ( image_open "$DISK" "$r" ) || die "the image could not be opened (above). Nothing is marked sealed: build-template.sh verdict reads it again"
    rm -f "$LOG"/sealed-*
    for v in SEAL-REFUSED MANIFEST stamp seal-final.log seal-probes; do
        rc=0; image_get "$r" "/etc/weaver-appliance/$v" "$LOG/sealed-$v" || rc=$?
        [ "$rc" != 2 ] || die "the image could not be read at /etc/weaver-appliance/$v (above). Nothing is marked sealed: build-template.sh verdict reads it again"
    done
    image_close "$r"; rm -rf -- "${r:?}"
    trap - EXIT INT TERM HUP
    if [ -f "$LOG/sealed-SEAL-REFUSED" ]; then
        cat "$LOG/sealed-SEAL-REFUSED"
        die "the seal refused (above, and $LOG/sealed-seal-final.log); the image is not sealed"
    fi
    [ -s "$LOG/sealed-MANIFEST" ] || die "the image holds no MANIFEST; the image is not sealed (see $LOG/sealed-seal-final.log)"
    # Each commit the stamp names must be the input: an image compiled from other commits
    # than the ones asked for is not this build's.
    local got want
    for v in weaveragent.commit:WA_COMMIT weaverweb.commit:WW_COMMIT; do
        want=${v##*:}; v=${v%%:*}
        got=$(sed -n "s/^$v = \([0-9a-f]\{40\}\)\$/\1/p" "$LOG/sealed-stamp" 2>/dev/null | head -1)
        [ -n "$got" ] || die "the image's stamp names no $v; the image is not sealed"
        [ "$got" = "${!want}" ] || die "the image's stamp names $v $got, not $want ${!want}; the image is not sealed"
    done
    # And the version the host names: an image stamped with another version is not this
    # build's.
    got=$(sed -n 's/^appliance\.version = \([0-9][0-9.]*\)$/\1/p' "$LOG/sealed-stamp" 2>/dev/null | head -1)
    [ -n "$got" ] || die "the image's stamp names no appliance.version; the image is not sealed"
    [ "$got" = "$VERSION" ] || die "the image's stamp names appliance.version $got, not VERSION $VERSION; the image is not sealed"
    # The seal ends with a compaction, while the domain is still defined: one that fails
    # leaves the disk as it was, writable, and `verdict` runs again.
    compact_disk
    # The published digest is of the compacted file (the operator's decision 18 of
    # 2026-11-15). It and the file's figures are recorded while the domain is still
    # defined and the disk writable, so a seal cut short here is finished by `verdict`
    # (the Planner's review of 2026-11-15, F3).
    sha256sum "$DISK" | tee "$LOG/appliance-sha256.txt"
    qemu-img info "$DISK" >"$LOG/appliance-qemu-img-info.txt"
    compact_check "$DISK" | tee "$LOG/appliance-compact.txt" \
        || die "the compacted disk is not compact ($LOG/appliance-compact.txt); nothing is marked sealed, and verdict runs again"
    # The sealed image is never booted again as itself: the disk is made read-only, then
    # its domain undefined. proof boots an overlay. A cut between the two leaves a sealed
    # disk, its digest recorded, under a defined domain: proof runs, and the domain is
    # undefined by hand (virsh undefine NAME --nvram).
    chmod 0444 "$DISK"
    virsh_ undefine "$NAME" --nvram >/dev/null
    # The build guest's host key pair, its pin and its seed image, which carries the private
    # key, go with it.
    rm -rf -- "${DIR:?}/hostkey" "${DIR:?}/known_hosts" "${DIR:?}/seed.iso"
}

# compact_disk: the build's disk rewritten whole, holding only the clusters the guest
# allocated. A qcow2 never moves its end back, so the sealed file's length is the high-water
# mark of the build's writes (20,964,769,792 bytes on the fifth build, for 3.23 GiB of
# data). qemu-img convert writes a new standalone file of the allocated, non-zero clusters
# alone. The copy must compare equal to the disk, guest byte for guest byte, before it
# replaces it. Anything short of that stops here with the disk unchanged and the copy
# removed.
compact_disk() {
    local part=$DIR/appliance.compact.part rc
    [ ! -e "$part" ] && [ ! -L "$part" ] || die "$part stands, left by a compaction cut short. It may be removed by hand, alone, and verdict run again. destroy removes the whole build, the disk with it"
    qemu-img info "$DISK" >"$LOG/appliance-before-compaction-qemu-img-info.txt"
    echo "before compaction: $(stat -c %s "$DISK") bytes long, $(du -B1 "$DISK" | cut -f1) bytes allocated" \
        | tee "$LOG/appliance-compaction.txt"
    trap 'rm -f -- "$DIR/appliance.compact.part"' EXIT
    trap 'exit 130' INT; trap 'exit 143' TERM; trap 'exit 129' HUP
    qemu-img convert -f qcow2 -O qcow2 "$DISK" "$part" || die "the compaction failed (above); the disk is unchanged and verdict runs again"
    # The copy reaches the disk before it is compared and before it replaces the disk
    # (the Planner's review of 2026-11-15, F10).
    sync -- "$part" || die "the compacted copy could not be synced; the disk is unchanged"
    rc=0; qemu-img compare -f qcow2 -F qcow2 "$DISK" "$part" >>"$LOG/appliance-compaction.txt" 2>&1 || rc=$?
    [ "$rc" = 0 ] || die "the compacted copy does not compare equal to the disk (qemu-img compare exited $rc; $LOG/appliance-compaction.txt); the disk is unchanged"
    # The copy must be compact before it replaces the disk, so a copy that is not stops the
    # seal here, with the domain defined (F4).
    compact_check "$part" >>"$LOG/appliance-compaction.txt" 2>&1 \
        || die "the compacted copy is not compact, or could not be checked ($LOG/appliance-compaction.txt); the disk is unchanged"
    mv -f -- "$part" "$DISK" || die "the compacted copy could not replace the disk; the disk is unchanged"
    trap - EXIT INT TERM HUP
    sync -- "$DIR" || die "the directory could not be synced after the compacted copy replaced the disk; verdict runs again"
    echo "after compaction: $(stat -c %s "$DISK") bytes long, $(du -B1 "$DISK" | cut -f1) bytes allocated; compared equal" \
        | tee -a "$LOG/appliance-compaction.txt"
}

# proof: what the sealed image holds, read offline, and what one boot of it sends.
#  1. The stamp, the MANIFEST (its sha256 is the image's identity), the seal's probe
#     record or its refusal, the network and update configuration, and what the seal's
#     power-off wrote after its final probe (image_late_writes: a random seed, systemd's
#     credential secret, a non-empty log), read out of the image through image_open. The
#     sealed file's sha256, set against the one the seal recorded, and whether the file is
#     compact (compact_check, the operator's decision 18 of 2026-11-15). The version the
#     stamp names (decision 4). The shipped ggml CUDA library's device code, listed with
#     the host's cuobjdump: no PTX, and cubins for exactly the architectures WeaverAgent's
#     build names (device_code_check, decision 6).
#  2. One boot of an overlay on the sealed image, with no seed and a fresh firmware store,
#     its user-mode NIC's every frame captured (QEMU filter-dump), for PROOF_BOOT seconds.
#     The capture is summarized by kind: an image that configures no network sends no
#     DHCP and no DNS. The NIC is restrict=on: the sealed image carries no guard against
#     the host (the build VM's guard is its seed's, section 4, and the image's firewall is
#     the operator's), so the rig isolates the boot itself. The capture still holds every
#     frame the guest sends; past QEMU's own DHCP, none is answered. The second, third
#     and fourth builds' no-seed boots ran without restrict=on.
# mock: the offline mock (code/test/offline-mock.sh), which stubs sudo and so runs only
# inside the build guest, never on a host. The code tree is copied in, the guest marked as a
# build guest (/run/weaver-build-guest, root's, in /run so it is gone at the next boot and
# never in the sealed image), the mock run, and the copy removed. Its output and count are
# LOG/mock.txt. It runs before the cleanup, while the guest still has the tools it uses.
mock() {
    tar -C "$HERE" -cf - . | gssh "$PORT" 'rm -rf ~/mock-code && mkdir ~/mock-code && tar -C ~/mock-code -xf -' \
        || die "the code tree did not copy into the guest"
    run_step "$PORT" "$LOG" mock 'sudo install -o root -g root -m 0644 /dev/null /run/weaver-build-guest || exit 1
        bash ~/mock-code/test/offline-mock.sh; rc=$?; rm -rf ~/mock-code; exit $rc'
    # The last check line, or, where the mock printed none (its refusal outside a build
    # guest, or a stop before its first check), its last lines: never a silent exit.
    local last
    last=$(grep -E '^(ok|FAIL) ' "$LOG/mock.txt" | sed -n '$p' || true)
    if [ -n "$last" ]; then
        echo "$last"
    else
        echo "the mock printed no ok or FAIL line; its last lines:"; tail -5 "$LOG/mock.txt"
    fi
    echo "checks passed: $(grep -c '^ok ' "$LOG/mock.txt" || true)"
    step_ok "$LOG" mock || die "the offline mock did not pass (a refusal, or a check that failed); see $LOG/mock.txt"
}

# PROOF_OWN: what this run of proof made, as words (read, proof, first), each added as soon
# as its directory is made, so that proof_cleanup removes only those. destroy names all
# three.
PROOF_OWN=""
# proof_cleanup: on any exit of proof, what its boots and its offline read would leave,
# of what PROOF_OWN names: each overlay domain (only where it stands on its own overlay),
# its directory and pool, and the offline read's FUSE export and directory. A console or
# capture not yet kept is kept first, as interrupted-*. Signals are ignored while it runs,
# so a second one cannot cut it short, and no step of it can stop the rest.
proof_cleanup() {
    local d f rc
    trap '' INT TERM HUP PIPE
    # A wait the signal cut short: its sleep is killed, not left to run out.
    [ -z "$PROOF_SLEEP" ] || kill "$PROOF_SLEEP" 2>/dev/null || true
    for d in proof first; do
        case " $PROOF_OWN " in *" $d "*) ;; *) continue ;; esac
        # A domain that could not be stopped or undefined (2) keeps its directory: an
        # undefined domain that still runs goes on writing its overlay.
        rc=0; dom_drop "$NAME-$d" "$VM_ROOT/$d/overlay.qcow2" || rc=$?
        if [ "$rc" = 2 ]; then
            echo "proof cleanup: $NAME-$d was not stopped and undefined (above); $VM_ROOT/$d is kept" >&2
            continue
        fi
        [ -d "$VM_ROOT/$d" ] || continue
        # The first boot's offline placement, where a signal cut it short: its export is
        # stopped before its directory goes.
        [ "$d" != first ] || image_close "$VM_ROOT/first/place" || true
        # The first boot's console log sits beside its socket, under con/. The password
        # file under secret/ goes with the directory.
        for f in console.log con/console.log boot.pcap; do
            [ ! -s "$VM_ROOT/$d/$f" ] || cp "$VM_ROOT/$d/$f" "$LOG/proof/interrupted-$d-${f##*/}" 2>/dev/null || true
        done
        pool_drop "$VM_ROOT/$d" || true
        rm -rf -- "${VM_ROOT:?}/$d" || true
    done
    case " $PROOF_OWN " in *" read "*) ;; *) return 0 ;; esac
    image_close "$VM_ROOT/proof-read" || true
    rm -rf -- "${VM_ROOT:?}/proof-read" || true
}

# proof_lock: proof and destroy each hold VM_ROOT/proof.lock while they run, so a second
# proof, or a destroy, refuses while one runs instead of tearing it down.
proof_lock() {
    exec {PROOF_LOCK_FD}>>"$VM_ROOT/proof.lock" || die "cannot open $VM_ROOT/proof.lock"
    flock -n "$PROOF_LOCK_FD" || die "a proof is running (it holds $VM_ROOT/proof.lock); $1"
}
# proof_sleep SECONDS: proof's waits, run in the background and waited on, so a TERM or
# INT reaches the trap at once. The sleep runs without the lock's descriptor, so a sleep
# the trap leaves behind holds no lock (on the fifth build an orphaned boot wait held it
# for 104 s after a TERM), and proof_cleanup kills it. Anything else proof starts that can
# outlive a command (the offline read's daemons) starts without the descriptor too.
# The proof's boots name their overlay with a DAC label that libvirt must not relabel (the
# Planner's ruling of 2026-11-15). libvirt's DAC driver otherwise relabels every image in a
# disk's backing chain to QEMU's account and restores none that is read-only: on the fifth
# build the sealed disk was left owned by the account every QEMU on the host runs as, which
# may make it writable again. The label on the disk holds for its whole backing chain, so
# libvirt leaves the sealed disk's owner and mode alone and QEMU reads it as the 0444 file
# it is. The overlay itself is QEMU's to write through an ACL (proof_overlay).
PROOF_NORELABEL="source.seclabel0.model=dac,source.seclabel0.relabel=no"
# proof_overlay PATH: a new overlay on the sealed disk, writable by its owner and
# QEMU_USER alone, with any ACL entry it inherits removed first, as qemu_file does.
proof_overlay() {
    [ ! -e "$1" ] && [ ! -L "$1" ] || die "$1 already stands: proof_overlay makes a new overlay only"
    qemu-img create -q -f qcow2 -b "$DISK" -F qcow2 "$1" && chmod 0600 "$1" \
        && setfacl -b "$1" && setfacl -m "u:${QEMU_USER:-libvirt-qemu}:rw" "$1" \
        || die "cannot make the overlay $1, writable by its owner and ${QEMU_USER:-libvirt-qemu} alone"
}
PROOF_SLEEP=""
proof_sleep() {
    ( exec {PROOF_LOCK_FD}>&-; exec sleep "$1" ) & PROOF_SLEEP=$!
    wait "$PROOF_SLEEP"
    PROOF_SLEEP=""
}

proof() {
    [ -f "$DISK" ] && [ ! -w "$DISK" ] || die "no sealed image at $DISK"
    proof_lock "this one refuses rather than tear it down"
    # What a cut-short run left is refused, not reused or removed: destroy removes it. The
    # cleanup is set only after these refusals, and removes only what this run makes.
    local d
    for d in proof first; do
        ! virsh_ dominfo "$NAME-$d" >/dev/null 2>&1 || die "$NAME-$d is already defined, left by a run cut short (destroy removes it)"
        [ ! -e "$VM_ROOT/$d" ] || die "$VM_ROOT/$d stands, left by a run cut short (destroy removes it)"
    done
    [ ! -e "$VM_ROOT/proof-read" ] || die "$VM_ROOT/proof-read stands, left by a run cut short (destroy removes it)"
    trap proof_cleanup EXIT
    trap 'exit 130' INT; trap 'exit 143' TERM; trap 'exit 129' HUP
    local out=$LOG/proof r=$VM_ROOT/proof-read f p rc disk_before
    # The sealed disk's owner and mode, read again after both boots: a change fails the
    # proof (the boots' no-relabel label, PROOF_NORELABEL).
    disk_before=$(stat -c '%U:%G %a' "$DISK")
    mkdir -p "$out"
    # A pass is recorded only at the end of a run that passed, so no earlier one stands.
    rm -f "$out/passed.txt"
    mkdir -- "$r"; PROOF_OWN="read"
    ( exec {PROOF_LOCK_FD}>&-; image_open "$DISK" "$r" ) || die "the sealed image could not be opened (above); nothing booted"
    for f in stamp MANIFEST seal-probes SEAL-REFUSED; do
        rc=0; image_get "$r" "/etc/weaver-appliance/$f" "$out/$f" || rc=$?
        [ "$rc" != 2 ] || die "the sealed image could not be read at /etc/weaver-appliance/$f (above); nothing booted"
    done
    {
        echo "== /etc/weaver-appliance"; image_ls "$r" /etc/weaver-appliance || echo "(absent)"
        for p in /etc/netplan /etc/systemd/network /etc/NetworkManager/system-connections \
                 /etc/apt/sources.list.d /etc/apt/apt.conf.d /etc/cloud/cloud.cfg.d \
                 /etc/systemd/system /home /root; do
            echo "== $p"; image_ls "$r" "$p" || echo "(absent)"
        done
        echo "== /etc/systemd/resolved.conf, uncommented lines"
        image_cat "$r" /etc/systemd/resolved.conf | grep -vE '^[[:space:]]*(#|$)' || echo "(none)"
        echo "== /etc/apt/sources.list.d/ubuntu.sources, Enabled lines"
        image_cat "$r" /etc/apt/sources.list.d/ubuntu.sources | grep -i '^Enabled' || echo "(none)"
    } >"$out/network-and-updates.txt" 2>&1
    # Judged with the first boot's findings, after both boots (finding 10 of the review).
    image_late_writes "$r" >"$out/late-writes.txt" 2>&1 || die "the offline read of late writes failed"
    # The shipped ggml CUDA library's device code (decision 6 of 2026-11-15), judged with
    # the rest after both boots. A record with neither its last line nor a finding did not
    # run to its end, and fails too.
    device_code_check "$r" "$r" >"$out/device-code.txt" 2>&1 || true
    grep -qE '^(PROOF-FAIL: |device code: checked$)' "$out/device-code.txt" \
        || echo "PROOF-FAIL: the device-code check did not run to its end" >>"$out/device-code.txt"
    cat "$out/device-code.txt"
    image_close "$r"
    # An image with no MANIFEST, or with a refusal, is not sealed: nothing is booted.
    if [ -f "$out/SEAL-REFUSED" ] || [ ! -f "$out/MANIFEST" ]; then
        [ ! -f "$out/SEAL-REFUSED" ] || cat "$out/SEAL-REFUSED"
        die "no MANIFEST in the image, or a seal refusal: not a sealed image; nothing booted"
    fi
    sha256sum "$out/MANIFEST" | tee "$out/identity.txt"
    echo "MANIFEST: $(M=$LINKS_MARK awk '$0 == ENVIRON["M"] { exit } { n++ } END { print n + 0 }' "$out/MANIFEST") files, $(manifest_links "$out/MANIFEST" | wc -l) links" | tee -a "$out/identity.txt"
    # The sealed file itself (decision 18): its sha256 is the published digest, so it must
    # be the one the seal recorded after its compaction, and the file must be compact.
    local fsum ssum
    {
        fsum=$(sha256sum "$DISK" | cut -d' ' -f1)
        echo "the sealed file: sha256 $fsum, $(stat -c %s "$DISK") bytes long, $(du -B1 "$DISK" | cut -f1) bytes allocated"
        ssum=$(cut -d' ' -f1 "$LOG/appliance-sha256.txt" 2>/dev/null || true)
        if [ -z "$ssum" ]; then
            echo "PROOF-FAIL: no sha256 recorded by the seal ($LOG/appliance-sha256.txt)"
        elif [ "$ssum" = "$fsum" ]; then
            echo "the seal recorded the same sha256"
        else
            echo "PROOF-FAIL: the seal recorded sha256 $ssum, not the sealed file's"
        fi
        compact_check "$DISK" || true
    } 2>&1 | tee "$out/sealed-file.txt"
    # The version the stamp names (decision 4): one line, <release>.<respin>, and VERSION
    # where the caller names it.
    local ver
    {
        ver=$(sed -n 's/^appliance\.version = //p' "$out/stamp" 2>/dev/null)
        if [ "$(grep -c '^appliance\.version = ' "$out/stamp" 2>/dev/null)" = 1 ] \
                && [[ "$ver" =~ ^[1-9][0-9]*\.(0|[1-9][0-9]*)$ ]]; then
            echo "the stamp names version $ver"
            [ -z "${VERSION:-}" ] || [ "$ver" = "$VERSION" ] || echo "PROOF-FAIL: the stamp names version $ver, not VERSION $VERSION"
        else
            echo "PROOF-FAIL: the stamp names no one version of the form <release>.<respin> ($(tr '\n' ' ' <<<"${ver:-none}"))"
        fi
    } 2>&1 | tee "$out/version.txt"
    # The overlay boot. The capture and console files are made here, writable by their
    # owner and QEMU's account alone (qemu_file).
    local pn=$NAME-proof pdir=$VM_ROOT/proof
    mkdir -- "$pdir"; PROOF_OWN="$PROOF_OWN proof"
    proof_overlay "$pdir/overlay.qcow2"
    qemu_file "$pdir/boot.pcap"
    qemu_file "$pdir/console.log"
    virt-install --connect "$URI" --name "$pn" --memory 4096 --vcpus 2 \
        --osinfo "$OS_VARIANT" --machine q35 --cpu host-passthrough \
        --boot "firmware=efi,firmware.feature0.name=secure-boot,firmware.feature0.enabled=no,nvram=$pdir/VARS.fd" \
        --disk "path=$pdir/overlay.qcow2,format=qcow2,bus=virtio,$PROOF_NORELABEL" \
        --network none \
        --qemu-commandline="-netdev user,id=un0,ipv6=off,restrict=on -device virtio-net-pci,netdev=un0,bus=pcie.0,addr=0x0a -object filter-dump,id=fd0,netdev=un0,file=$pdir/boot.pcap -chardev file,id=con0,path=$pdir/console.log -serial chardev:con0" \
        --graphics none --console none --noautoconsole --import \
        >"$out/proof-virt-install.txt" 2>&1
    proof_sleep "${PROOF_BOOT:-180}"
    dom_drop "$pn" "$pdir/overlay.qcow2" || die "$pn was not stopped and undefined (above); proof's cleanup keeps $pdir where it still runs"
    cp "$pdir/boot.pcap" "$out/boot.pcap"
    tr -d '\r' <"$pdir/console.log" >"$out/boot-console.log" || true
    pool_drop "$pdir"
    rm -rf -- "${pdir:?}"; PROOF_OWN=${PROOF_OWN/ proof/}
    python3 -I - "$out/boot.pcap" <<'PY' | tee "$out/boot-frames.txt"
import struct, sys
from collections import Counter
data = open(sys.argv[1], "rb").read()
if len(data) < 24:
    print("capture: empty (no frame header written)")
    print("PROOF-FAIL: the no-seed boot's capture holds no header, so the capture did not run")
    sys.exit(0)
endian = "<" if data[:4] in (b"\xd4\xc3\xb2\xa1", b"\x4d\x3c\xb2\xa1") else ">"
off, kinds, guest = 24, Counter(), 0
# Every field is read only where the frame is long enough for it. A short frame, or a
# record cut off at the end of the file, is counted as such and never stops the proof.
while off + 16 <= len(data):
    _, _, incl, _ = struct.unpack(endian + "IIII", data[off:off + 16])
    if off + 16 + incl > len(data):
        kinds["record cut off at the end of the capture"] += 1; break
    f = data[off + 16:off + 16 + incl]; off += 16 + incl
    # QEMU's user-mode network answers from MAC addresses 52:55:..., every other frame is
    # the guest's own (one too short to name its source is counted as the guest's).
    if len(f) < 12 or f[6:8] != b"\x52\x55": guest += 1
    if len(f) < 14:
        kinds["frame shorter than an Ethernet header"] += 1; continue
    et = struct.unpack("!H", f[12:14])[0]
    if et == 0x0806: kinds["ARP"] += 1
    elif et == 0x86DD:
        kinds["IPv6 " + ({58: "ICMPv6"}.get(f[20], f"proto {f[20]}") if len(f) > 20 else "frame too short")] += 1
    elif et == 0x0800:
        if len(f) < 34:
            kinds["IPv4 frame too short"] += 1; continue
        proto, ihl = f[23], (f[14] & 15) * 4
        sport, dport = (struct.unpack("!HH", f[14 + ihl:18 + ihl])
                        if proto in (6, 17) and len(f) >= 18 + ihl else (0, 0))
        name = {67: "DHCP", 68: "DHCP", 53: "DNS", 123: "NTP"}.get(dport) or {67: "DHCP", 53: "DNS"}.get(sport)
        kinds["IPv4 " + (name or {1: "ICMP", 6: f"TCP to {dport}", 17: f"UDP to {dport}"}.get(proto, f"proto {proto}"))] += 1
    else: kinds[f"ethertype {et:#06x}"] += 1
print(f"capture: {sum(kinds.values())} frames, {guest} sent by the guest")
for k, n in sorted(kinds.items()): print(f"  {k}: {n}")
# An image that configures no network sends nothing (the fourth build's five boots sent
# none), so any frame the guest sends fails the proof.
if guest: print(f"PROOF-FAIL: the no-seed boot sent {guest} frame(s); an image with no network configured sends none")
PY
    proof_first_boot "$out"
    local disk_after
    disk_after=$(stat -c '%U:%G %a' "$DISK")
    {
        echo "the sealed disk: $disk_before before the proof, $disk_after after"
        [ "$disk_after" = "$disk_before" ] || echo "PROOF-FAIL: the sealed disk's owner or mode changed during the proof"
    } | tee "$out/disk.txt"
    # The verdict: every finding of the offline read's late writes and device code, of the
    # no-seed boot's capture, of the first boot and of the disk's owner and mode fails the
    # proof, after both boots' records are kept.
    grep -h '^PROOF-FAIL: ' "$out/sealed-file.txt" "$out/version.txt" "$out/late-writes.txt" "$out/device-code.txt" "$out/boot-frames.txt" "$out/first-boot.txt" "$out/disk.txt" >"$out/PROOF-FAILED" || true
    if [ -s "$out/PROOF-FAILED" ]; then
        cat "$out/PROOF-FAILED"
        die "the proof failed on $(wc -l <"$out/PROOF-FAILED") finding(s); see $out/sealed-file.txt, $out/version.txt, $out/late-writes.txt, $out/device-code.txt, $out/boot-frames.txt, $out/first-boot.txt and $out/disk.txt"
    fi
    rm -f "$out/PROOF-FAILED"
    # The pass names the sealed file the seal recorded, which the proof found equal, and
    # the MANIFEST: tag reads it.
    echo "proof passed: sealed file sha256 $(cut -d' ' -f1 "$LOG/appliance-sha256.txt"), MANIFEST sha256 $(cut -c1-64 "$out/identity.txt" | head -1)" \
        | tee "$out/passed.txt"
}

# tag: the annotated tag the operator makes in WeaverAgent for this version (decision 3 of
# 2026-11-15), printed and never applied. Nothing here runs git. Its name is
# appliance/<version>, it goes on the WeaverAgent commit the version was compiled from,
# and its message carries the version, the install record's digest and the stack's test
# count. Each is read from the stamp the proof read out of the sealed image, and only after
# a proof that passed for the sealed file the seal recorded. VERSION and WA_COMMIT, where
# named, must equal the stamp's. The message is also written to the log, for git tag -F.
tag() {
    local out=$LOG/proof f=$LOG/tag-message.txt s ver wa inst tests passed sealed
    [ -f "$DISK" ] && [ ! -w "$DISK" ] || die "no sealed image at $DISK"
    passed=$(sed -n 's/^proof passed: sealed file sha256 \([0-9a-f]\{64\}\), MANIFEST sha256 [0-9a-f]\{64\}$/\1/p' "$out/passed.txt" 2>/dev/null)
    [ -n "$passed" ] || die "no proof has passed for this image ($out/passed.txt): the tag names a version that was proved"
    sealed=$(cut -d' ' -f1 "$LOG/appliance-sha256.txt" 2>/dev/null || true)
    [ "$passed" = "$sealed" ] || die "the proof that passed was of sealed file sha256 $passed, not the one the seal recorded (${sealed:-none}): run proof again"
    s=$out/stamp
    [ -f "$s" ] || die "no stamp read out of the sealed image ($s)"
    ver=$(stamp_one "$s" appliance.version '^[1-9][0-9]*\.(0|[1-9][0-9]*)$') || die "$ver"
    [ -z "${VERSION:-}" ] || [ "$ver" = "$VERSION" ] || die "the stamp names version $ver, not VERSION $VERSION"
    wa=$(stamp_one "$s" weaveragent.commit '^[0-9a-f]{40}$') || die "$wa"
    [ -z "${WA_COMMIT:-}" ] || [ "$wa" = "$WA_COMMIT" ] || die "the stamp names weaveragent.commit $wa, not WA_COMMIT $WA_COMMIT"
    inst=$(stamp_one "$s" stack.install-tuple '^[0-9a-f]{64}( .*)?$') || die "$inst"
    inst=${inst%% *}
    tests=$(stamp_one "$s" stack.tests '^[1-9][0-9]* passed, 0 failed( .*)?$') || die "$tests"
    tests=${tests%% (*}
    printf '%s\n' "appliance image $ver" "" \
        "version: $ver" \
        "install record: /etc/weaver/stack/install-tuple, sha256 $inst, as written at image build" \
        "tests: $tests, at bootstrap-stack.sh's test step in the build guest" >"$f"
    echo "the tag, for the operator to make in WeaverAgent and push (nothing here applies it):"
    echo "  name:    appliance/$ver"
    echo "  commit:  $wa"
    echo "  message: (written to $f)"
    sed 's/^/    /' "$f"
    echo "  made by: git tag -a appliance/$ver -F $f $wa"
}

# stamp_one STAMP KEY REGEX: the value of the stamp's one KEY line, which must match
# REGEX. Otherwise a sentence, and status 1.
stamp_one() {
    local v n
    n=$(awk -v k="$2" 'index($0, k " = ") == 1 { n++ } END { print n + 0 }' "$1")
    [ "$n" = 1 ] || { echo "the stamp names $2 $n times, not once"; return 1; }
    v=$(awk -v k="$2" 'index($0, k " = ") == 1 { print substr($0, length(k) + 4) }' "$1")
    [[ "$v" =~ $3 ]] || { echo "the stamp's $2 is '$v', out of its form"; return 1; }
    printf '%s\n' "$v"
}

# proof_place OUT PDIR: one of the operator's two by-hand steps after the console login,
# the network set (decision 16 of 2026-11-15), written into the proof's own overlay
# offline, before its boot, and never into the image: a netplan file naming the static
# address QEMU's user network forwards to (10.0.2.15), its gateway and its resolver,
# matched by the NIC's MAC address. Placed offline so SSH answers from the boot, and the
# proof can show it refusing root before the first login. The other step, the key, is
# placed at the console after that login (proof_console). restrict=on's DHCP offers no
# gateway and no resolver, and netplan's wait for the network on 26.04 waits for a
# resolver (--dns): a boot with no network named waited 120 s (measured on the fourth
# build). The file is read back from the overlay opened again read-only, with its owner
# and mode, and the filesystem is checked (e2fsck -n), before anything boots. image_open
# opens an overlay writable, never the sealed image.
proof_place() {
    local out=$1 pdir=$2 r=$2/place rc=0
    mkdir -- "$pdir/put" "$r"
    cat >"$pdir/put/netplan.yaml" <<EOF
network:
  version: 2
  ethernets:
    nic0:
      match:
        macaddress: "$PROOF_MAC"
      addresses: [10.0.2.15/24]
      routes:
        - to: default
          via: 10.0.2.2
      nameservers:
        addresses: [10.0.2.3]
EOF
    cp "$pdir/put/netplan.yaml" "$out/first-boot-netplan.txt"
    ( exec {PROOF_LOCK_FD}>&-; image_open "$pdir/overlay.qcow2" "$r" rw ) \
        || die "the proof's overlay could not be opened writable (above); nothing booted"
    ( image_put "$r" "$pdir/put/netplan.yaml" "$PROOF_NETPLAN" 0100600 ) || rc=$?
    cp "$r/put.log" "$out/first-boot-place-debugfs.txt" 2>/dev/null || true
    image_close "$r"
    rm -f -- "$r/put.cmd" "$r/put.log"
    [ "$rc" = 0 ] || die "the proof's network file could not be written into its overlay (above, and $out/first-boot-place-debugfs.txt); nothing booted"
    ( exec {PROOF_LOCK_FD}>&-; image_open "$pdir/overlay.qcow2" "$r" ) \
        || die "the proof's overlay could not be opened again to read its network file back (above); nothing booted"
    # The file read back as written, and the filesystem clean, or nothing boots.
    {
        rc=0
        image_got "$r" "$pdir/put/netplan.yaml" "$PROOF_NETPLAN" 0100600 || rc=1
        echo "== e2fsck -fn on the overlay's root filesystem"
        e2fsck -fn "$r/root.raw" 2>&1 || { echo "e2fsck exited $?"; rc=1; }
        if [ "$rc" = 0 ]; then echo "placed: the network file reads back as written; the filesystem is clean"
        else echo "NOT PLACED"; fi
    } >"$out/first-boot-place.txt" 2>&1
    image_close "$r"
    rmdir -- "$r"
    cat "$out/first-boot-place.txt"
    [ "$(tail -1 "$out/first-boot-place.txt")" = "placed: the network file reads back as written; the filesystem is clean" ] \
        || die "the proof's network file did not read back as written, or the filesystem is not clean ($out/first-boot-place.txt); nothing booted"
}

# proof_first_boot OUT: a second overlay, booted as an operator's first boot is, with a seed
# attached that the image must ignore, since it disables cloud-init (the operator's ruling of
# 2026-11-16): its meta-data names a key and a hostname, its user-data a bootcmd, a runcmd and
# the default account, and none may be acted on. The operator's network, set by hand after
# logging in, is placed in the overlay offline (proof_place), so SSH answers from the boot;
# everything else the operator does, the proof does at the console. At this boot
# sshd-keygen makes the host keys. The proof waits for the console's login prompt, which
# must greet with the image's hostname (IMAGE_HOSTNAME), and scans the keys SSH presents.
# Before the first login it tries root over SSH with an empty password, a wrong one and the
# proof key, and each must be refused (proof_refusal). Then the first login, at the serial
# console through QEMU's socket (proof_console): root, no password asked, the change
# required and made, and the proof key placed from that session (the operator's ruling of
# 2026-11-16: root, set at first login). It then logs in over SSH as root with that key and
# reads the host keys' fingerprints, as the operator does: each key SSH presents must be
# one the guest made. Only on that match does it read cloud-init's state (disabled, the seed
# ignored), the hostname, the network as the overlay named it, SSH's units and
# sshd-keygen's run, the listeners, and the stamp's commits, try root over SSH again with
# the password the console set and a wrong one (each refused), set the MANIFEST against
# the booted guest's files and links (manifest_compare), and read the base inbound policy
# (proof_inbound). The network is QEMU's restrict=on: the guest reaches neither the host nor
# beyond it, and the SSH forward still answers.
proof_first_boot() {
    local out=$1 pn=$NAME-first pdir=$VM_ROOT/first port=${PROOF_PORT:-$((PORT + 100))}
    # The base inbound policy's probe port: TCP 8080 in the guest, forwarded from the next
    # host port.
    local dport=$((port + 1))
    local t=0 limit=${PROOF_FIRST_BOOT:-300} sfp gfp gfps prompt
    mkdir -- "$pdir"; PROOF_OWN="$PROOF_OWN first"
    proof_overlay "$pdir/overlay.qcow2"
    # The console: QEMU's socket, which the first login is typed through, and its log of
    # every byte the console sent from the firmware on. A unix socket's path holds at most
    # 107 bytes, and QEMU cannot start on a longer one.
    local sock=$pdir/con/console.sock
    [ "${#sock}" -le 107 ] \
        || die "the console's socket path, $pdir/con/console.sock, is longer than a unix socket allows (107 bytes); set VM_ROOT shorter; nothing booted"
    qemu_dir "$pdir/con"
    qemu_file "$pdir/con/console.log"
    proof_place "$out" "$pdir"
    # The password the first login sets, made here and read only from this file: it is
    # typed at the console and offered to SSH, never printed or logged, and removed after
    # the last try.
    mkdir -m 0700 -- "$pdir/secret"
    ( umask 077; python3 -I -c 'import secrets, string; a = string.ascii_letters + string.digits; print("".join(secrets.choice(a) for _ in range(24)))' >"$pdir/secret/pw" ) \
        || die "the first login's password could not be made; nothing booted"
    printf '#!/bin/sh\nexec cat "%s"\n' "$pdir/secret/askpass.pw" >"$pdir/secret/askpass" && chmod 0700 "$pdir/secret/askpass" \
        || die "the SSH tries' password helper could not be made; nothing booted"
    mkdir -- "$pdir/seed"
    printf 'instance-id: %s\nlocal-hostname: weaver-proof-seed\npublic-keys:\n  - %s\n' \
        "$pn-1" "$(cut -d' ' -f1,2 "$KEY.pub")" >"$pdir/seed/meta-data"
    cat >"$pdir/seed/user-data" <<'EOF'
#cloud-config
bootcmd:
  - [sh, -c, "echo bootcmd ran > /var/tmp/weaver-proof-bootcmd"]
runcmd:
  - [sh, -c, "echo runcmd ran > /var/tmp/weaver-proof-runcmd"]
users:
  - default
EOF
    cp "$pdir/seed/meta-data" "$out/first-boot-seed-meta-data.txt"
    cp "$pdir/seed/user-data" "$out/first-boot-seed-user-data.txt"
    xorriso -as mkisofs -quiet -o "$pdir/seed.iso" -V cidata -J -r \
        "$pdir/seed/user-data" "$pdir/seed/meta-data" 2>/dev/null \
        || die "the first boot's seed image could not be made; nothing booted"
    virt-install --connect "$URI" --name "$pn" --memory 4096 --vcpus 2 \
        --osinfo "$OS_VARIANT" --machine q35 --cpu host-passthrough \
        --boot "firmware=efi,firmware.feature0.name=secure-boot,firmware.feature0.enabled=no,nvram=$pdir/VARS.fd" \
        --disk "path=$pdir/overlay.qcow2,format=qcow2,bus=virtio,$PROOF_NORELABEL" \
        --disk "path=$pdir/seed.iso,device=cdrom,bus=sata" \
        --network none \
        --qemu-commandline="-netdev user,id=un0,ipv6=off,restrict=on,hostfwd=tcp:127.0.0.1:$port-:22,hostfwd=tcp:127.0.0.1:$dport-:8080 -device virtio-net-pci,netdev=un0,mac=$PROOF_MAC,bus=pcie.0,addr=0x0a -chardev socket,id=con0,path=$pdir/con/console.sock,server=on,wait=off,logfile=$pdir/con/console.log -serial chardev:con0" \
        --graphics none --console none --noautoconsole --import \
        >"$out/first-boot-virt-install.txt" 2>&1
    # Wait for the console's login prompt, then scan once. Each scan connection leaves
    # without authenticating, and OpenSSH's PerSourcePenalties (on by default since 9.8)
    # counts each against its source; through QEMU's user network every host connection
    # comes from one address, so polling with scans got the host refused and lost a key
    # type (measured on the fourth build). The scan names the three types sshd-keygen makes
    # (ssh-keygen -A); one short scan is made again after the penalty lapses.
    # grep reads the console through a process substitution: under pipefail, a pipe from tr
    # would fail once grep had its match and tr had more to write, and the wait would run to
    # its bound with the prompt on the console.
    until grep -aqE '^[^[:space:]]+ login:' < <(tr -d '\r' <"$pdir/con/console.log" 2>/dev/null); do
        t=$((t + 2)); [ "$t" -lt "$limit" ] || break; sleep 2
    done
    local want=3 n=0
    proof_keyscan "$port" "$pdir/scanned"
    if [ "$(ssh-keygen -lf "$pdir/scanned" 2>/dev/null | grep -c 'SHA256:' || true)" -lt "$want" ]; then
        proof_sleep 30; n=1
        proof_keyscan "$port" "$pdir/scanned"
    fi
    {
        echo "== first boot, $t s to the console's login prompt (bound $limit s); $n more scans"
        prompt=$(tr -d '\r' <"$pdir/con/console.log" | grep -aoE '^[^[:space:]]+ login:' | tail -1 || true)
        echo "== the console's login prompt: ${prompt:-none}"
        [ "$prompt" = "$IMAGE_HOSTNAME login:" ] || echo "PROOF-FAIL: the console's login prompt is '${prompt:-none}', not the image's hostname, $IMAGE_HOSTNAME"
        echo "== the keys SSH presents"; ssh-keygen -lf "$pdir/scanned" 2>&1 || true
        sfp=$(ssh-keygen -lf "$pdir/scanned" 2>/dev/null | grep -oE 'SHA256:[A-Za-z0-9+/]{43}' | sort -u || true)
        # Before the first login root's password is empty and expired, and no key is
        # authorized: SSH must refuse root every way.
        echo "== SSH as root before the first login"
        proof_refusal "$port" "$pdir/scanned" "$pdir" "an empty password" empty
        proof_refusal "$port" "$pdir/scanned" "$pdir" "a wrong password" wrong
        proof_refusal "$port" "$pdir/scanned" "$pdir" "the proof key, not yet placed" key
        echo "== the first login, at the console"
        proof_console "$out" "$pdir"
        # The operator's read after logging in: the host keys' fingerprints, in the guest.
        echo "== the host keys' fingerprints, read in the guest as root, under the keys SSH presented"
        gfp=$(KNOWN_HOSTS=$pdir/scanned GUEST_USER=root gssh "$port" \
              'for k in /etc/ssh/ssh_host_*_key.pub; do ssh-keygen -lf "$k"; done' 2>&1) \
            || gfp="$gfp (the login as root, or the read, exited non-zero)"
        printf '%s\n' "$gfp"
        gfps=$(grep -oE 'SHA256:[A-Za-z0-9+/]{43}' <<<"$gfp" | sort -u || true)
        # Each key SSH presents is one the guest holds. Whether SSH presents all of them is
        # reported beside it.
        if [ -n "$sfp" ] && [ -z "$(comm -13 <(printf '%s\n' "$gfps") <(printf '%s\n' "$sfp"))" ]; then
            if [ "$gfps" = "$sfp" ]; then
                echo "the host keys: MATCH, the guest holds each key SSH presents, and SSH presents each the guest holds ($(wc -l <<<"$sfp") keys)"
            else
                echo "the host keys: MATCH for the keys SSH presents ($(wc -l <<<"$sfp") of the guest's $(wc -l <<<"$gfps")); SSH did not present: $(comm -23 <(printf '%s\n' "$gfps") <(printf '%s\n' "$sfp") | tr '\n' ' ')"
            fi
            KNOWN_HOSTS=$pdir/scanned GUEST_USER=root gssh "$port" '
                # A seed is attached, and the image disables cloud-init: none of it is
                # acted on.
                echo "== cloud-init, with a seed attached"
                cloud-init status --long 2>&1 | head -8
                st=$(cloud-init status 2>/dev/null | sed -n "s/^status: //p")
                [ "$st" = disabled ] || echo "PROOF-FAIL: cloud-init status is ${st:-unread}, not disabled, with a seed attached"
                [ -f /etc/cloud/cloud-init.disabled ] || echo "PROOF-FAIL: /etc/cloud/cloud-init.disabled is absent"
                echo "== the seed'"'"'s asks"
                for m in /var/tmp/weaver-proof-bootcmd /var/tmp/weaver-proof-runcmd; do
                    if [ -e "$m" ]; then echo "PROOF-FAIL: the seed'"'"'s ${m##*-} ran"; else echo "not run: ${m##*-}"; fi
                done
                # getent exits 2 where the account is not found. Any other failure is a read
                # that did not happen, not an account that was not made.
                getent passwd ubuntu >/dev/null; g=$?
                case $g in
                    0) echo "PROOF-FAIL: the seed'"'"'s default account was made" ;;
                    2) echo "not made: the default account" ;;
                    *) echo "PROOF-FAIL: getent exited $g, so whether the default account exists is not known" ;;
                esac
                echo "== the hostname"; hostname
                # The data mount points, with no disk attached: laid by the image, empty,
                # owned by root, and nothing mounted on them.
                echo "== the data mount points, with no disk attached"
                for d in '"$DATA_MOUNTS"'; do
                    s=$(stat -c "%F %u:%g %a" -- "$d" 2>/dev/null); n=$(ls -A -- "$d" 2>/dev/null | wc -l)
                    echo "$d: ${s:-absent}, $n entries"
                    [ "$s" = "directory 0:0 755" ] && [ "$n" = 0 ] || echo "PROOF-FAIL: $d is not an empty directory root owns with mode 755"
                    [ -z "$s" ] || [ "$(stat -c %d -- "$d")" = "$(stat -c %d -- /)" ] || echo "PROOF-FAIL: $d has a filesystem mounted on it, with no disk attached"
                done
                [ "$(hostname)" = '"$IMAGE_HOSTNAME"' ] || echo "PROOF-FAIL: the hostname is $(hostname), not '"$IMAGE_HOSTNAME"'"
                echo "== the network as the overlay named it"; ip -br addr; ip route; resolvectl dns 2>&1
                echo "== the wait for the network"; systemd-analyze blame 2>/dev/null | grep -E "wait-online" || echo "(not in blame)"
                echo "== ubuntu.sources after the first boot"; grep -i "^Enabled" /etc/apt/sources.list.d/ubuntu.sources || echo "(no Enabled line)"
                echo "== ssh units"; systemctl is-enabled ssh.service ssh.socket sshd-keygen.service; systemctl is-active ssh.service
                # sshd-keygen made the host keys: its first-boot condition held and it ran.
                kg=$(systemctl show -p ConditionResult -p Result -p ActiveState sshd-keygen.service)
                echo "sshd-keygen.service: $(tr "\n" " " <<<"$kg")"
                grep -qx ConditionResult=yes <<<"$kg" && grep -qx Result=success <<<"$kg" \
                    || echo "PROOF-FAIL: sshd-keygen did not run at the first boot"
                echo "== listening sockets (ruling 4: nothing but SSH listens beyond loopback)"
                # One listing, judged as printed. A listing that fails, or lists nothing (SSH
                # itself listens), fails the proof: a check that did not run never passes.
                if lsk=$(ss -Hltunp 2>&1); then
                    printf "%s\n" "$lsk"
                    n=$(printf "%s\n" "$lsk" | grep -c .)
                    echo "the listing ran: $n listening sockets"
                    [ "$n" -gt 0 ] || echo "PROOF-FAIL: the listing of listening sockets is empty, though SSH listens"
                    printf "%s\n" "$lsk" | while read -r net state rq sq local peer rest; do
                        [ -n "$local" ] || continue
                        port=${local##*:}; host=${local%:*}
                        # A socket bound to the loopback device (%lo) is loopback, whatever
                        # its address.
                        case "$host" in 127.*|"[::1]"|"[::1]%"*|"[::ffff:127."*|*%lo) continue ;; esac
                        [ "$net" = tcp ] && [ "$port" = 22 ] && continue
                        echo "PROOF-FAIL: a listener beyond loopback other than SSH: $net $local"
                    done
                else
                    printf "%s\n" "$lsk"
                    echo "PROOF-FAIL: the listening sockets could not be listed (ss exited non-zero)"
                fi
                echo "== the commits, from the stamp"; grep -E "^(weaveragent|weaverweb)\.(commit|origin) " /etc/weaver-appliance/stamp
                n=$(grep -cE "^(weaveragent|weaverweb)\.commit = [0-9a-f]{40}$" /etc/weaver-appliance/stamp)
                [ "$n" = 2 ] || echo "PROOF-FAIL: the stamp names $n of the two commits"
                echo "== the MANIFEST itself"; sha256sum /etc/weaver-appliance/MANIFEST' 2>&1 \
                || echo "PROOF-FAIL: the login as root, or its reads, exited $?"
            # The password is set now, and SSH must still refuse it.
            echo "== SSH as root after the first login"
            proof_refusal "$port" "$pdir/scanned" "$pdir" "the password set at the console" "$pdir/secret/pw"
            proof_refusal "$port" "$pdir/scanned" "$pdir" "a wrong password" wrong
            # The MANIFEST set against the booted guest, judged here: its files listed as the
            # seal lists them, less the host keys' private halves (no digest of a private key
            # is made), and its links formatted here as the seal formats them. A listing that
            # fails, a difference outside the first boot's expected list, and a MANIFEST
            # without its links section each fail the proof (manifest_compare). The files'
            # listing runs under pipefail, so a find error fails it, though sort and
            # sha256sum succeed. The booted listings stay in the overlay's directory and go
            # with it.
            echo "== the MANIFEST against the booted guest"
            if ! KNOWN_HOSTS=$pdir/scanned GUEST_USER=root gssh "$port" \
                    "bash -o pipefail -c \"find $MANIFEST_SCOPE -xdev -type f -not -path '/etc/weaver-appliance/MANIFEST*' -not -path /boot/grub/grubenv -not -name 'ssh_host_*_key' -print0 | LC_ALL=C sort -zu | xargs -0 sha256sum\"" \
                    >"$pdir/files-booted" 2>"$pdir/files-booted.err"; then
                echo "PROOF-FAIL: the booted guest's files could not be listed: $(tail -2 "$pdir/files-booted.err" | tr '\n' ' ')"
            elif ! KNOWN_HOSTS=$pdir/scanned GUEST_USER=root gssh "$port" \
                    "find $MANIFEST_SCOPE -xdev -type l -printf '%p\0%l\0'" 2>"$pdir/links-booted.err" \
                    | links_format >"$pdir/links-booted"; then
                echo "PROOF-FAIL: the booted guest's links could not be listed: $(tail -2 "$pdir/links-booted.err" | tr '\n' ' ')"
            else
                manifest_compare "$out/MANIFEST" "$pdir/files-booted" "$pdir/links-booted"
            fi
            proof_inbound "$port" "$dport" "$pdir/scanned"
        else
            echo "the host keys: NO MATCH; nothing more is read"
            echo "PROOF-FAIL: the host keys: the guest's fingerprints do not name the keys SSH presents"
            echo "guest: ${gfps:-none}"; echo "ssh: ${sfp:-none}"
        fi
        rm -f -- "$pdir/secret/pw" "$pdir/secret/askpass.pw"
    } >"$out/first-boot.txt" 2>&1
    cat "$out/first-boot.txt"
    dom_drop "$pn" "$pdir/overlay.qcow2" || die "$pn was not stopped and undefined (above); proof's cleanup keeps $pdir where it still runs"
    tr -d '\r' <"$pdir/con/console.log" >"$out/first-boot-console.log" || true
    pool_drop "$pdir"
    rm -rf -- "${pdir:?}"; PROOF_OWN=${PROOF_OWN/ first/}
}

# proof_console OUT PDIR: the first login, typed at the serial console through QEMU's
# socket in PDIR/con by console-drive.py, as the operator logs in: root at the prompt, the
# password from PDIR/secret/pw given twice when the change is asked, then, in that session,
# the proof key placed as root's one authorized key, one read line (the terminal, the
# account, root's password state, the keys held), and the logout. The steps and the
# driver's log go to OUT; neither holds the password, which is sent from its file.
# console_judge grades the log; a driver that stopped is a PROOF-FAIL too.
proof_console() {
    local out=$1 pdir=$2 rc
    cut -d' ' -f1,2 "$KEY.pub" >"$out/first-boot-authorized_keys.txt"
    cat >"$out/first-boot-session.steps" <<EOF
# The first login at the console (the operator's ruling of 2026-11-16: root, set at first login).
send \\r
expect 120 login:
send root\\r
expect 60 New password:
sendfile $pdir/secret/pw
expect 30 Retype new password:
sendfile $pdir/secret/pw
expect 60 root@[^ ]*:~#
send install -d -m 0700 /root/.ssh && echo '$(cat "$out/first-boot-authorized_keys.txt")' >/root/.ssh/authorized_keys && chmod 0600 /root/.ssh/authorized_keys\\r
expect 30 root@[^ ]*:~#
send printf 'READ tty=%s user=%s root=%s keys=%s\\\\n' "\$(tty)" "\$(id -un)" "\$(passwd -S root | cut -d' ' -f2)" "\$(wc -l </root/.ssh/authorized_keys)"\\r
expect 30 READ tty=/\\S+ user=\\S+ root=\\S+ keys=\\S+
send exit\\r
expect 60 login:
EOF
    # rc is read with ||: under errexit a driver that stops must be judged below, not end
    # the proof before console_judge says why.
    rc=0
    python3 -I "$LIB_DIR/console-drive.py" "$pdir/con/console.sock" "$out/first-boot-session.log" "$out/first-boot-session.steps" || rc=$?
    [ "$rc" = 0 ] || echo "PROOF-FAIL: the first login: the console driver stopped (exit $rc), above"
    console_judge "$out/first-boot-session.log" "$IMAGE_HOSTNAME"
}

# proof_refusal PORT KNOWN PDIR LABEL SOURCE: one SSH login as root through the forward at
# PORT, under the host keys in KNOWN, which must be refused (refusal_judge). SOURCE key is
# the proof key; empty is an empty password, wrong one the guest never had, and a file the
# password in it, each given through SSH_ASKPASS with public keys off. The helper reads the
# password from PDIR/secret, and the client sends it only if the server offers a password
# method. ssh -v's output stays in PDIR and holds no password.
proof_refusal() {
    # rc is read with ||: the script runs under errexit, and a refused ssh (255) is the
    # result this judges, not a failure that ends the proof.
    local port=$1 known=$2 pdir=$3 label=$4 src=$5 o rc=0
    o=$(mktemp "$pdir/refusal.XXXXXX") || { echo "PROOF-FAIL: SSH as root with $label: no file for ssh's output"; return 0; }
    if [ "$src" = key ]; then
        ssh -v -i "$KEY" -p "$port" -o BatchMode=yes -o IdentitiesOnly=yes -o ConnectTimeout=10 \
            -o StrictHostKeyChecking=yes -o UserKnownHostsFile="$known" -o GlobalKnownHostsFile=/dev/null \
            root@127.0.0.1 true >"$o" 2>&1 </dev/null || rc=$?
    else
        case $src in
            empty) ( umask 077; : >"$pdir/secret/askpass.pw" ) ;;
            wrong) ( umask 077; printf 'not-the-password-%s\n' "$RANDOM$RANDOM" >"$pdir/secret/askpass.pw" ) ;;
            *) ( umask 077; cp -- "$src" "$pdir/secret/askpass.pw" ) ;;
        esac
        SSH_ASKPASS=$pdir/secret/askpass SSH_ASKPASS_REQUIRE=force DISPLAY= setsid -w ssh -v -p "$port" \
            -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive \
            -o NumberOfPasswordPrompts=1 -o ConnectTimeout=10 -o StrictHostKeyChecking=yes \
            -o UserKnownHostsFile="$known" -o GlobalKnownHostsFile=/dev/null \
            root@127.0.0.1 true >"$o" 2>&1 </dev/null || rc=$?
        rm -f -- "$pdir/secret/askpass.pw"
    fi
    refusal_judge "$o" "$rc" "$label"
}

# proof_keyscan PORT FILE: the keys SSH presents on the forward at PORT, added to FILE.
# A second scan adds to the first and never replaces it, so a key the first found stands
# (the Planner's review of 2026-11-15, F11). A key that differs between the two scans
# stands twice, and the fingerprint comparison then fails.
proof_keyscan() {
    local port=$1 f=$2
    ssh-keyscan -T 10 -t rsa,ecdsa,ed25519 -p "$port" 127.0.0.1 >"$f.scan" 2>/dev/null || true
    touch -- "$f"
    sort -u -- "$f" "$f.scan" >"$f.merged" && mv -f -- "$f.merged" "$f"
    rm -f -- "$f.scan"
}

# proof_inbound PORT DPORT KNOWN: the base inbound policy (the operator's decision 9 of
# 2026-11-15) on the first boot, with no agent loaded. In the guest: the tables the kernel
# holds, the image's own read back with its input policy drop and its SSH accept, and a
# listener started on TCP 8080, serving a new empty directory, that must answer on the
# guest's own loopback. From the host:
# that listener, reached through the forward on DPORT, must be silent (dropped, inbound_probe),
# and then SSH, through the forward on PORT, must answer. SSH is probed last: the probe
# leaves without authenticating, which sshd counts against the forward's one source
# address (PerSourcePenalties). Each finding is a PROOF-FAIL line.
proof_inbound() {
    local port=$1 dport=$2 known=$3 lpid lb ans
    echo "== the base inbound policy, with no agent loaded"
    KNOWN_HOSTS=$known GUEST_USER=root gssh "$port" '
        echo "-- nftables.service: $(systemctl is-active nftables.service)"
        echo "-- the tables the kernel holds"; nft list tables
        if r=$(nft list table inet weaver_appliance 2>&1); then
            printf "%s\n" "$r"
            grep -q "type filter hook input priority filter; policy drop;" <<<"$r" || echo "PROOF-FAIL: the table inet weaver_appliance does not drop inbound by default"
            grep -q "tcp dport 22 accept" <<<"$r" || echo "PROOF-FAIL: the table inet weaver_appliance does not accept SSH"
        else
            printf "%s\n" "$r"; echo "PROOF-FAIL: the table inet weaver_appliance is not loaded"
        fi' 2>&1 || echo "PROOF-FAIL: the read of the loaded rules exited $?"
    lpid=$(KNOWN_HOSTS=$known GUEST_USER=root gssh "$port" '
        d=$(mktemp -d) && cd "$d" && setsid nohup python3 -m http.server 8080 --bind 0.0.0.0 >/dev/null 2>&1 </dev/null & p=$!
        for i in $(seq 50); do ss -Hltn "sport = :8080" | grep -q . && break; sleep 0.2; done
        echo "$p"' 2>/dev/null) || lpid=""
    if ! [[ "$lpid" =~ ^[0-9]+$ ]]; then
        echo "PROOF-FAIL: the listener on TCP 8080 did not start in the guest (${lpid:-no PID})"
    else
        lb=$(KNOWN_HOSTS=$known GUEST_USER=root gssh "$port" '
            ss -Hltn "sport = :8080"
            python3 -c "import urllib.request as u; print(\"loopback:\", u.urlopen(\"http://127.0.0.1:8080/\", timeout=5).status)"' 2>&1) || true
        printf -- '-- the listener on TCP 8080, in the guest\n%s\n' "$lb"
        grep -qx 'loopback: 200' <<<"$lb" || echo "PROOF-FAIL: the listener on TCP 8080 did not answer on the guest's loopback, so its drop proves nothing"
        ans=$(inbound_probe "$dport" 15)
        echo "-- TCP 8080 from the host, through the forward: $ans"
        [ "$ans" = silent ] || echo "PROOF-FAIL: inbound to TCP 8080 was not dropped ($ans)"
        KNOWN_HOSTS=$known GUEST_USER=root gssh "$port" "kill $lpid 2>/dev/null; sleep 1
            if ss -Hltn 'sport = :8080' | grep -q .; then echo 'the listener still listens'; else echo 'the listener is stopped'; fi" 2>&1 || true
    fi
    ans=$(inbound_probe "$port" 10)
    echo "-- SSH from the host, through the forward: $ans"
    case $ans in "answered: SSH-2.0-"*) ;; *) echo "PROOF-FAIL: SSH did not answer through the forward ($ans)" ;; esac
}

# destroy: the build guest, only where its domain stands on DISK, then what a proof or a
# seal left (proof_cleanup's objects and the seal's offline read), then the build's
# directory and its pool.
destroy() {
    proof_lock "destroy waits until it ends"
    local d rc=0
    dom_drop "$NAME" "$DISK" || rc=$?
    case $rc in
        0) ;;
        1) die "$NAME is defined on another disk than $DISK; nothing removed" ;;
        *) die "$NAME was not stopped and undefined (above); nothing removed" ;;
    esac
    for d in proof first; do
        rc=0; dom_drop "$NAME-$d" "$VM_ROOT/$d/overlay.qcow2" || rc=$?
        case $rc in
            0) ;;
            1) die "$NAME-$d is defined on another disk than its overlay; nothing more removed" ;;
            *) die "$NAME-$d was not stopped and undefined (above); nothing more removed" ;;
        esac
    done
    PROOF_OWN="read proof first"; proof_cleanup
    image_close "$VM_ROOT/seal-read"; rm -rf -- "${VM_ROOT:?}/seal-read"
    pool_drop "$DIR"
    chmod -R u+w "$DIR" 2>/dev/null || true
    rm -rf "$DIR"
}

case ${1:-} in
create) create ;;
step) shift; step "$@" ;;
mock) mock ;;
seal) seal ;;
verdict) seal_verdict ;;
proof) proof ;;
tag) tag ;;
destroy) destroy ;;
*) awk 'NR > 1 && !/^#/ { exit } NR > 1' "$0" >&2; exit 2 ;;   # the header, to its end
esac
