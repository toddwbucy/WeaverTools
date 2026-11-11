#!/usr/bin/env bash
# Boot a throwaway guest on libvirt's default NAT network and measure what it reaches,
# before any template is built. The guest runs guest/net-probe at first boot and writes
# the result to a small raw disk, so the result is read even if SSH into the guest
# fails; the probe then runs a second time over SSH when the guest is reachable.
#
#   VM_ROOT=<dir with images/ clones/ logs/ keys/> code/net-probe.sh [run|destroy]
#
# Needs: virsh, virt-install, qemu-img, xorriso, ssh, and access to qemu:///system.
set -euo pipefail

: "${VM_ROOT:?set VM_ROOT to the directory holding images/ clones/ logs/ keys/}"
NAME=${NAME:-weaver-nettest}
IMAGE=${IMAGE:-$VM_ROOT/images/ubuntu-26.04-server-cloudimg-amd64.img}
KEY=${KEY:-$VM_ROOT/keys/measure_ed25519}
OS_VARIANT=${OS_VARIANT:-ubuntu26.04}
# NET=network=default,model=virtio is libvirt's NAT network; NET=user,model=virtio is
# QEMU user-mode networking (slirp), whose traffic leaves as the QEMU process's own;
# NET=user-fwd is slirp given directly to QEMU with SSH forwarded from host loopback
# port SSH_PORT, since libvirt's own port forwarding needs the passt backend.
NET=${NET:-network=default,model=virtio}
SSH_PORT=${SSH_PORT:-2222}
URI=qemu:///system
HERE=$(cd "$(dirname "$0")" && pwd)
WORK=$VM_ROOT/clones/$NAME
STAMP=$(date -u +%Y%m%dT%H%M%SZ)
LOG=$VM_ROOT/logs/$NAME-$STAMP
SSH_OPTS=(-i "$KEY" -o BatchMode=yes -o ConnectTimeout=5 -o StrictHostKeyChecking=no
    -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR)

virsh_() { virsh -c "$URI" "$@"; }

destroy() {
    if virsh_ dominfo "$NAME" >/dev/null 2>&1; then
        virsh_ destroy "$NAME" >/dev/null 2>&1 || true
        virsh_ undefine "$NAME" --nvram >/dev/null
    fi
    rm -rf "$WORK"
}

seed() {
    local pub b64 dir=$WORK/seed
    pub=$(cat "$KEY.pub")
    b64=$(base64 -w0 "$HERE/guest/net-probe")
    mkdir -p "$dir"
    printf 'instance-id: %s-%s\nlocal-hostname: %s\n' "$NAME" "$STAMP" "$NAME" >"$dir/meta-data"
    cat >"$dir/user-data" <<EOF
#cloud-config
users:
  - name: op
    shell: /bin/bash
    sudo: "ALL=(ALL) NOPASSWD:ALL"
    lock_passwd: true
    ssh_authorized_keys:
      - $pub
ssh_pwauth: false
package_update: false
package_upgrade: false
write_files:
  - path: /usr/local/sbin/net-probe
    permissions: "0755"
    encoding: b64
    content: $b64
runcmd:
  - [sh, -c, "/usr/local/sbin/net-probe >/run/net-probe.txt 2>&1; dd if=/run/net-probe.txt of=/dev/disk/by-id/virtio-results conv=notrunc,fsync status=none"]
EOF
    xorriso -as mkisofs -quiet -o "$WORK/seed.iso" -V cidata -J -r "$dir/user-data" "$dir/meta-data"
}

run() {
    destroy
    mkdir -p "$WORK" "$LOG"
    [ -f "$KEY" ] || ssh-keygen -q -t ed25519 -N '' -C weaver-measure -f "$KEY"
    qemu-img create -q -f qcow2 -F qcow2 -b "$IMAGE" "$WORK/disk.qcow2" 20G
    truncate -s 1M "$WORK/results.raw"
    seed
    local net=(--network "$NET")
    if [ "$NET" = user-fwd ]; then
        # As run on 2026-11-11: IPv6 on, and the loopback probe IPv4 only. The IPv6 alias
        # (fec0::2 onto the host's ::1) was measured later from the clone (README).
        net=(--network none --qemu-commandline="-netdev user,id=un0,hostfwd=tcp:127.0.0.1:$SSH_PORT-:22 -device virtio-net-pci,netdev=un0,bus=pcie.0,addr=0x0a")
    fi
    virt-install --connect "$URI" --name "$NAME" --memory 2048 --vcpus 2 \
        --osinfo "$OS_VARIANT" --machine q35 \
        --boot firmware=efi,firmware.feature0.name=secure-boot,firmware.feature0.enabled=no \
        --disk "path=$WORK/disk.qcow2,format=qcow2,bus=virtio" \
        --disk "path=$WORK/seed.iso,device=cdrom,bus=sata" \
        --disk "path=$WORK/results.raw,format=raw,bus=virtio,serial=results" \
        "${net[@]}" \
        --graphics none --console pty,target.type=serial --noautoconsole --import \
        >"$LOG/virt-install.txt" 2>&1
    virsh_ dumpxml "$NAME" >"$LOG/domain.xml"

    local addr="" i
    [ "$NET" = user-fwd ] && SSH_OPTS+=(-p "$SSH_PORT")
    for i in $(seq 1 60); do
        [ "$NET" = user-fwd ] && { addr=127.0.0.1; break; }
        addr=$(virsh_ domifaddr "$NAME" --source lease 2>/dev/null |
            awk '/ipv4/ {sub("/.*", "", $4); print $4; exit}')
        [ -n "$addr" ] && break
        sleep 2
    done
    echo "network $NET, lease after ~$((i * 2))s: ${addr:-none}" | tee "$LOG/lease.txt"
    virsh_ net-dhcp-leases default >>"$LOG/lease.txt" 2>&1 || true

    for i in $(seq 1 90); do
        [ -n "$(tr -d '\0' <"$WORK/results.raw" | head -c 1)" ] && break
        sleep 2
    done
    tr -d '\0' <"$WORK/results.raw" >"$LOG/first-boot-probe.txt"
    echo "first-boot probe after ~$((i * 2))s, $(wc -c <"$LOG/first-boot-probe.txt") bytes"

    if [ -n "$addr" ] && ssh "${SSH_OPTS[@]}" "op@$addr" true 2>"$LOG/ssh-error.txt"; then
        ssh "${SSH_OPTS[@]}" "op@$addr" sudo /usr/local/sbin/net-probe >"$LOG/ssh-probe.txt" 2>&1
        ssh "${SSH_OPTS[@]}" "op@$addr" 'cloud-init status --long; systemd-analyze' \
            >"$LOG/guest-boot.txt" 2>&1 || true
        if [ -n "${HOST_PORTS:-}" ]; then
            # TCP connect and close only, to each port given, at the address the guest
            # sees as its gateway; no protocol is spoken.
            ssh "${SSH_OPTS[@]}" "op@$addr" "gw=\$(ip -4 route show default | awk '{print \$3; exit}');
                for p in $HOST_PORTS; do
                    if timeout 3 bash -c \"exec 3<>/dev/tcp/\$gw/\$p\" 2>/dev/null; then echo \"\$gw:\$p connects\";
                    else echo \"\$gw:\$p no-connect\"; fi
                done" >"$LOG/gateway-ports.txt" 2>&1 || true
        fi
        echo "ssh probe: $LOG/ssh-probe.txt"
    else
        echo "ssh to guest failed: $(cat "$LOG/ssh-error.txt" 2>/dev/null)"
    fi
    echo "log: $LOG"
}

case ${1:-run} in
run) run ;;
destroy) destroy ;;
*) echo "usage: $0 [run|destroy]" >&2; exit 2 ;;
esac
