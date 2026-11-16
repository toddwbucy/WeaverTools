#!/usr/bin/env bash
# Build the appliance image from the Ubuntu cloud image, one step at a time, each step's
# output and status kept under logs/. The binaries are compiled inside this guest from the
# two named commits; nothing built on the host enters the image (template-clone-Spec.md,
# section 10).
#
#   VM_ROOT=<dir> OS=26.04 DRIVER=<release> DRIVER_ROUTE=<route> COMPUTE_CAP=<cc> \
#     WA_COMMIT=<sha> WW_COMMIT=<sha> code/build-template.sh create
#                                              disk, seed, define and boot
#   ... code/build-template.sh step NAME [ARGS]  run guest/template-steps NAME
#                                              (GUEST_ENV="VAR=value ..." adds to its env)
#   ... code/build-template.sh mock              the offline mock, run inside the build guest
#   ... code/build-template.sh seal              seal, shut down, read the verdict, mark
#                                              read-only
#   ... code/build-template.sh verdict           read the verdict again from the shut-off
#                                              guest, where seal's own read failed
#   ... code/build-template.sh proof             read the sealed image; boot an overlay
#                                              with no seed, then one as a first boot
#   ... code/build-template.sh destroy           undefine and remove the build guest
#
# The six inputs are named, never defaulted: OS, DRIVER (the NVIDIA driver release),
# DRIVER_ROUTE (the route it comes by), COMPUTE_CAP (the compute capability the CUDA
# kernels are compiled for, without its dot), WA_COMMIT and WW_COMMIT (the WeaverAgent and
# WeaverWeb commits, in full). create needs all six. Every step is given those that are
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
        local v; for v in DRIVER DRIVER_ROUTE COMPUTE_CAP WA_COMMIT WW_COMMIT; do [ -n "${!v:-}" ] || die "name $v"; done
    fi
    [ -z "${WA_COMMIT:-}" ] || [[ "$WA_COMMIT" =~ ^[0-9a-f]{40}$ ]] || die "WA_COMMIT is a commit in full, 40 hex digits"
    [ -z "${WW_COMMIT:-}" ] || [[ "$WW_COMMIT" =~ ^[0-9a-f]{40}$ ]] || die "WW_COMMIT is a commit in full, 40 hex digits"
    # One form for DRIVER, the same regex guest/template-steps' driver step checks.
    [ -z "${DRIVER:-}" ] || [[ "$DRIVER" =~ ^[0-9]{3}\.[0-9]+(\.[0-9]+)?$ ]] || die "DRIVER is a driver release such as 615.71.09"
    [ -z "${DRIVER_ROUTE:-}" ] || [ "$DRIVER_ROUTE" = ubuntu-prebuilt ] || die "DRIVER_ROUTE is a route this tree builds: ubuntu-prebuilt"
    [ -z "${COMPUTE_CAP:-}" ] || [[ "$COMPUTE_CAP" =~ ^[0-9]{2,3}$ ]] || die "COMPUTE_CAP is a compute capability without its dot, such as 86"
}
# inputs_env: the guest command's environment prefix: the inputs that are set, and the base
# image by name and digest, which the stamp records.
inputs_env() {
    BASE_IMAGE="$IMAGE_NAME $IMAGE_SHA256"
    guest_env WA_COMMIT WW_COMMIT DRIVER DRIVER_ROUTE COMPUTE_CAP BASE_IMAGE
}
steps_in() {
    gscp "$PORT" "$HERE/guest/template-steps" /tmp/template-steps
    gssh "$PORT" 'sudo install -m 0755 /tmp/template-steps /usr/local/sbin/template-steps'
}

create() {
    inputs_ok all
    virsh_ dominfo "$NAME" >/dev/null 2>&1 && die "$NAME is already defined"
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
    [ -n "${WA_COMMIT:-}" ] && [ -n "${WW_COMMIT:-}" ] || die "name WA_COMMIT and WW_COMMIT: the verdict sets the stamp's commits against them"
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
    [ -n "${WA_COMMIT:-}" ] && [ -n "${WW_COMMIT:-}" ] || die "name WA_COMMIT and WW_COMMIT: the verdict sets the stamp's commits against them"
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
    # The sealed image is never booted again as itself: its domain is undefined and the
    # disk made read-only; proof boots an overlay.
    virsh_ undefine "$NAME" --nvram >/dev/null
    chmod 0444 "$DISK"
    # The build guest's host key pair, its pin and its seed image, which carries the private
    # key, go with it.
    rm -rf -- "${DIR:?}/hostkey" "${DIR:?}/known_hosts" "${DIR:?}/seed.iso"
    sha256sum "$DISK" | tee "$LOG/appliance-sha256.txt"
    qemu-img info "$DISK" >"$LOG/appliance-qemu-img-info.txt"
}

# proof: what the sealed image holds, read offline, and what one boot of it sends.
#  1. The stamp, the MANIFEST (its sha256 is the image's identity), the seal's probe
#     record or its refusal, the network and update configuration, and what the seal's
#     power-off wrote after its final probe (image_late_writes: a random seed, systemd's
#     credential secret, a non-empty log), read out of the image through image_open.
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
        for f in console.log boot.pcap; do
            [ ! -s "$VM_ROOT/$d/$f" ] || cp "$VM_ROOT/$d/$f" "$LOG/proof/interrupted-$d-$f" 2>/dev/null || true
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
    image_close "$r"
    # An image with no MANIFEST, or with a refusal, is not sealed: nothing is booted.
    if [ -f "$out/SEAL-REFUSED" ] || [ ! -f "$out/MANIFEST" ]; then
        [ ! -f "$out/SEAL-REFUSED" ] || cat "$out/SEAL-REFUSED"
        die "no MANIFEST in the image, or a seal refusal: not a sealed image; nothing booted"
    fi
    sha256sum "$out/MANIFEST" | tee "$out/identity.txt"
    echo "MANIFEST: $(M=$LINKS_MARK awk '$0 == ENVIRON["M"] { exit } { n++ } END { print n + 0 }' "$out/MANIFEST") files, $(manifest_links "$out/MANIFEST" | wc -l) links" | tee -a "$out/identity.txt"
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
    # The verdict: every finding of the offline read's late writes, of the no-seed boot's
    # capture, of the first boot and of the disk's owner and mode fails the proof, after
    # both boots' records are kept.
    grep -h '^PROOF-FAIL: ' "$out/late-writes.txt" "$out/boot-frames.txt" "$out/first-boot.txt" "$out/disk.txt" >"$out/PROOF-FAILED" || true
    if [ -s "$out/PROOF-FAILED" ]; then
        cat "$out/PROOF-FAILED"
        die "the proof failed on $(wc -l <"$out/PROOF-FAILED") finding(s); see $out/late-writes.txt, $out/boot-frames.txt, $out/first-boot.txt and $out/disk.txt"
    fi
    rm -f "$out/PROOF-FAILED"
    echo "proof passed: $(cut -c1-16 "$out/identity.txt" | head -1)"
}

# proof_first_boot OUT: a second overlay, booted as an operator's first boot is: a seed
# whose meta-data names an SSH key and a hostname and whose network-config names a static
# address, gateway and resolver, with no host key. Its user-data asks for a bootcmd, a runcmd, a package and an
# account, which the image's cloud-init must discard (the operator's ruling of 2026-11-14:
# SSH key, network and hostname only). It measures what trust on first use rests on: the
# host keys' fingerprints keys_to_console prints on the serial console, set against the
# keys SSH presents. Only on a match does it log in, under those keys, as the default
# account, and read cloud-init's verdict and modules, what became of the user-data's
# asks, the network as the seed named it, SSH's units and the stamp's commits, and it sets
# the MANIFEST against the booted guest's files and links (manifest_compare). The network is QEMU's restrict=on: the guest reaches
# neither the host nor beyond it, and the SSH forward still answers. restrict=on's DHCP
# offers no gateway and no resolver, and netplan's wait for the network on 26.04 waits for
# a resolver (--dns), so a seed with no network-config waited 120 s on every boot and
# cloud-init judged the boot degraded (measured on the fourth build); the seed's static
# network is also the third setting the operator ruled cloud-init takes.
proof_first_boot() {
    local out=$1 pn=$NAME-first pdir=$VM_ROOT/first port=${PROOF_PORT:-$((PORT + 100))}
    local t=0 limit=${PROOF_FIRST_BOOT:-300} cfp sfp
    mkdir -- "$pdir"; PROOF_OWN="$PROOF_OWN first"
    mkdir -- "$pdir/seed"
    proof_overlay "$pdir/overlay.qcow2"
    qemu_file "$pdir/console.log"
    printf 'instance-id: %s\nlocal-hostname: %s\npublic-keys:\n  - %s\n' \
        "$pn-1" "$pn" "$(cut -d' ' -f1,2 "$KEY.pub")" >"$pdir/seed/meta-data"
    cat >"$pdir/seed/user-data" <<'EOF'
#cloud-config
bootcmd:
  - [sh, -c, "echo bootcmd ran > /var/tmp/weaver-proof-bootcmd"]
runcmd:
  - [sh, -c, "echo runcmd ran > /var/tmp/weaver-proof-runcmd"]
packages:
  - hello
users:
  - default
  - name: weaver-proof-extra
    lock_passwd: true
EOF
    cat >"$pdir/seed/network-config" <<'EOF'
version: 2
ethernets:
  nic0:
    match:
      macaddress: "52:54:00:57:45:01"
    set-name: nic0
    addresses: [10.0.2.15/24]
    routes:
      - to: default
        via: 10.0.2.2
    nameservers:
      addresses: [10.0.2.3]
EOF
    cp "$pdir/seed/meta-data" "$out/first-boot-meta-data.txt"
    cp "$pdir/seed/user-data" "$out/first-boot-user-data.txt"
    cp "$pdir/seed/network-config" "$out/first-boot-network-config.txt"
    xorriso -as mkisofs -quiet -o "$pdir/seed.iso" -V cidata -J -r \
        "$pdir/seed/user-data" "$pdir/seed/meta-data" "$pdir/seed/network-config" 2>/dev/null
    virt-install --connect "$URI" --name "$pn" --memory 4096 --vcpus 2 \
        --osinfo "$OS_VARIANT" --machine q35 --cpu host-passthrough \
        --boot "firmware=efi,firmware.feature0.name=secure-boot,firmware.feature0.enabled=no,nvram=$pdir/VARS.fd" \
        --disk "path=$pdir/overlay.qcow2,format=qcow2,bus=virtio,$PROOF_NORELABEL" \
        --disk "path=$pdir/seed.iso,device=cdrom,bus=sata" \
        --network none \
        --qemu-commandline="-netdev user,id=un0,ipv6=off,restrict=on,hostfwd=tcp:127.0.0.1:$port-:22 -device virtio-net-pci,netdev=un0,mac=52:54:00:57:45:01,bus=pcie.0,addr=0x0a -chardev file,id=con0,path=$pdir/console.log -serial chardev:con0" \
        --graphics none --console none --noautoconsole --import \
        >"$out/first-boot-virt-install.txt" 2>&1
    # Wait for the console's fingerprint block, then scan once. Each scan connection leaves
    # without authenticating, and OpenSSH's PerSourcePenalties (on by default since 9.8)
    # counts each against its source; through QEMU's user network every host connection
    # comes from one address, so polling with scans got the host refused and lost a key
    # type (measured on the fourth build). The scan names the three types the image's
    # cloud-init makes; one short scan is made again after the penalty lapses.
    until grep -q 'END SSH HOST KEY FINGERPRINTS' "$pdir/console.log" 2>/dev/null; do
        t=$((t + 2)); [ "$t" -lt "$limit" ] || break; sleep 2
    done
    local want n=0
    want=$(sed -n '/BEGIN SSH HOST KEY FINGERPRINTS/,/END SSH HOST KEY FINGERPRINTS/p' "$pdir/console.log" \
           | grep -cE 'SHA256:' || true)
    ssh-keyscan -T 10 -t rsa,ecdsa,ed25519 -p "$port" 127.0.0.1 >"$pdir/scanned" 2>/dev/null || true
    if [ "$(ssh-keygen -lf "$pdir/scanned" 2>/dev/null | grep -c 'SHA256:' || true)" -lt "${want:-0}" ]; then
        proof_sleep 30; n=1
        ssh-keyscan -T 10 -t rsa,ecdsa,ed25519 -p "$port" 127.0.0.1 >"$pdir/scanned" 2>/dev/null || true
    fi
    {
        echo "== first boot, $t s to the console block (bound $limit s); $n more scans"
        echo "== the console's fingerprint block"
        sed -n '/BEGIN SSH HOST KEY FINGERPRINTS/,/END SSH HOST KEY FINGERPRINTS/p' "$pdir/console.log" | tr -d '\r'
        cfp=$(sed -n '/BEGIN SSH HOST KEY FINGERPRINTS/,/END SSH HOST KEY FINGERPRINTS/p' "$pdir/console.log" \
              | grep -oE 'SHA256:[A-Za-z0-9+/]{43}' | sort -u || true)
        sfp=$(ssh-keygen -lf "$pdir/scanned" 2>/dev/null | grep -oE 'SHA256:[A-Za-z0-9+/]{43}' | sort -u || true)
        echo "== the keys SSH presents"; ssh-keygen -lf "$pdir/scanned" 2>&1 || true
        # The operator's check: every key SSH presents is named on the console. Whether
        # SSH presents all of them is reported beside it.
        if [ -n "$sfp" ] && [ -z "$(comm -13 <(printf '%s\n' "$cfp") <(printf '%s\n' "$sfp"))" ]; then
            if [ "$cfp" = "$sfp" ]; then
                echo "trust on first use: MATCH, the console names each key SSH presents, and SSH presents each the console names ($(wc -l <<<"$cfp") keys)"
            else
                echo "trust on first use: MATCH for the keys SSH presents ($(wc -l <<<"$sfp") of the console's $(wc -l <<<"$cfp")); SSH did not present: $(comm -23 <(printf '%s\n' "$cfp") <(printf '%s\n' "$sfp") | tr '\n' ' ')"
            fi
            KNOWN_HOSTS=$pdir/scanned GUEST_USER=ubuntu gssh "$port" '
                # cloud-init status exits 0 for running, disabled and not started too, so the
                # proof waits for it to finish and requires the text "done" as well as exit 0.
                echo "== cloud-init status"
                timeout 600 cloud-init status --wait >/dev/null 2>&1; wrc=$?
                cloud-init status --long; rc=$?
                st=$(cloud-init status 2>/dev/null | sed -n "s/^status: //p")
                echo "(status ${st:-unread}, exit $rc; the wait exited $wrc)"
                [ "$wrc" != 124 ] || echo "PROOF-FAIL: cloud-init did not finish within 600 s"
                [ "$st" = done ] || echo "PROOF-FAIL: cloud-init status is ${st:-unread}, not done"
                [ "$rc" = 0 ] || echo "PROOF-FAIL: cloud-init status exited $rc (1 an error, 2 degraded)"
                echo "== user-data and vendor-data"; sudo grep -E "allow_userdata|vendordata2? consumption|vendor_data" /var/log/cloud-init.log | tail -4
                echo "== modules cloud-init ran"; sudo grep -oE "running (config|modules)-[a-z_]+" /var/log/cloud-init.log | sort -u
                echo "== the user-data asks"
                for m in /var/tmp/weaver-proof-bootcmd /var/tmp/weaver-proof-runcmd; do
                    if [ -e "$m" ]; then echo "RAN: $m"; echo "PROOF-FAIL: the user-data ${m##*-} ran"; else echo "not run: ${m##*-}"; fi
                done
                if dpkg-query -W -f "\${db:Status-Abbrev} \${Package}\n" hello 2>/dev/null | grep -q "^ii"; then
                    echo "INSTALLED: hello"; echo "PROOF-FAIL: the user-data package hello was installed"
                else echo "not installed: hello"; fi
                # getent exits 2 where the account is not found. Any other failure is a read
                # that did not happen, not an account that was not made.
                getent passwd weaver-proof-extra >/dev/null; g=$?
                case $g in
                    0) echo "MADE: weaver-proof-extra"; echo "PROOF-FAIL: the user-data account weaver-proof-extra was made" ;;
                    2) echo "not made: weaver-proof-extra" ;;
                    *) echo "PROOF-FAIL: getent exited $g, so whether weaver-proof-extra was made is not known" ;;
                esac
                echo "== the hostname"; hostname
                echo "== the network as the seed named it"; ip -br addr show nic0; ip route; resolvectl dns nic0 2>&1
                echo "== the wait for the network"; systemd-analyze blame 2>/dev/null | grep -E "wait-online" || echo "(not in blame)"
                echo "== ubuntu.sources after the first boot"; grep -i "^Enabled" /etc/apt/sources.list.d/ubuntu.sources || echo "(no Enabled line)"
                echo "== keys_to_console in cloud-init.log"; sudo grep -E "keys_to_console|keys-to-console" /var/log/cloud-init.log | grep -iE "running|ran" | tail -3
                echo "== ssh units"; systemctl is-enabled ssh.service ssh.socket sshd-keygen.service; systemctl is-active ssh.service
                echo "== listening sockets (ruling 4: nothing but SSH listens beyond loopback)"
                # One listing, judged as printed. A listing that fails, or lists nothing (SSH
                # itself listens), fails the proof: a check that did not run never passes.
                if lsk=$(sudo ss -Hltunp 2>&1); then
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
                    echo "PROOF-FAIL: the listening sockets could not be listed (sudo ss exited non-zero)"
                fi
                echo "== the commits, from the stamp"; grep -E "^(weaveragent|weaverweb)\.(commit|origin) " /etc/weaver-appliance/stamp
                n=$(grep -cE "^(weaveragent|weaverweb)\.commit = [0-9a-f]{40}$" /etc/weaver-appliance/stamp)
                [ "$n" = 2 ] || echo "PROOF-FAIL: the stamp names $n of the two commits"
                echo "== the MANIFEST itself"; sha256sum /etc/weaver-appliance/MANIFEST' 2>&1 \
                || echo "PROOF-FAIL: the login as the default account, or its reads, exited $?"
            # The MANIFEST set against the booted guest, judged here: its files listed as the
            # seal lists them, less the host keys' private halves (no digest of a private key
            # is made), and its links formatted here as the seal formats them. A listing that
            # fails, a difference outside the first boot's expected list, and a MANIFEST
            # without its links section each fail the proof (manifest_compare). The files'
            # listing runs under pipefail, so a find error fails it, though sort and
            # sha256sum succeed. The booted listings stay in the overlay's directory and go
            # with it.
            echo "== the MANIFEST against the booted guest"
            if ! KNOWN_HOSTS=$pdir/scanned GUEST_USER=ubuntu gssh "$port" \
                    "sudo bash -o pipefail -c 'find $MANIFEST_SCOPE -xdev -type f -not -path \"/etc/weaver-appliance/MANIFEST*\" -not -path /boot/grub/grubenv -not -name \"ssh_host_*_key\" -print0 | LC_ALL=C sort -zu | xargs -0 sha256sum'" \
                    >"$pdir/files-booted" 2>"$pdir/files-booted.err"; then
                echo "PROOF-FAIL: the booted guest's files could not be listed: $(tail -2 "$pdir/files-booted.err" | tr '\n' ' ')"
            elif ! KNOWN_HOSTS=$pdir/scanned GUEST_USER=ubuntu gssh "$port" \
                    "sudo find $MANIFEST_SCOPE -xdev -type l -printf '%p\0%l\0'" 2>"$pdir/links-booted.err" \
                    | links_format >"$pdir/links-booted"; then
                echo "PROOF-FAIL: the booted guest's links could not be listed: $(tail -2 "$pdir/links-booted.err" | tr '\n' ' ')"
            else
                manifest_compare "$out/MANIFEST" "$pdir/files-booted" "$pdir/links-booted"
            fi
        else
            echo "trust on first use: NO MATCH; no login"
            echo "PROOF-FAIL: trust on first use: the console's fingerprints do not name the keys SSH presents"
            echo "console: ${cfp:-none}"; echo "ssh: ${sfp:-none}"
        fi
    } >"$out/first-boot.txt" 2>&1
    cat "$out/first-boot.txt"
    dom_drop "$pn" "$pdir/overlay.qcow2" || die "$pn was not stopped and undefined (above); proof's cleanup keeps $pdir where it still runs"
    tr -d '\r' <"$pdir/console.log" >"$out/first-boot-console.log" || true
    pool_drop "$pdir"
    rm -rf -- "${pdir:?}"; PROOF_OWN=${PROOF_OWN/ first/}
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
destroy) destroy ;;
*) awk 'NR > 1 && !/^#/ { exit } NR > 1' "$0" >&2; exit 2 ;;   # the header, to its end
esac
