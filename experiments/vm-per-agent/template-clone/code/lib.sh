# Sourced by build-template.sh and clone.sh: the measurement's pins, and the host-side
# helpers that define a guest, reach it over SSH and log each step.
#
# VM_ROOT names the directory that holds images/, template/, clones/, logs/ and keys/.
# It has no default here: a default would be a box path in the repository.

: "${VM_ROOT:?set VM_ROOT to the directory holding images/ template/ clones/ logs/ keys/}"

URI=qemu:///system
# The guest operating system: Ubuntu Server 26.04 LTS, its cloud image as released on
# 2026-10-20, checked against SHA256SUMS signed by the Ubuntu cloud image key.
IMAGE_NAME=ubuntu-26.04-server-cloudimg-amd64.img
IMAGE_URL=https://cloud-images.ubuntu.com/releases/resolute/release
IMAGE_SHA256=5645d0318ad21f643ebb1ac4567514620e9893380b07b0201fda163baa5b7eb5
IMAGE_KEY=D2EB44626FDDC30B513D5BB71A5D6C4C7DB87C81
OS_VARIANT=ubuntu26.04
IMAGE=$VM_ROOT/images/$IMAGE_NAME
KEY=${KEY:-$VM_ROOT/keys/measure_ed25519}
GUEST_USER=op

virsh_() { virsh -c "$URI" "$@"; }
say() { printf '\n== %s\n' "$*"; }
die() { printf '\nREFUSED: %s\n' "$*" >&2; exit 1; }

# fetch_image: the cloud image, its SHA256SUMS and their signature, each checked.
fetch_image() {
    local dir=$VM_ROOT/images gnupg
    mkdir -p "$dir"
    if [ ! -f "$IMAGE" ]; then
        curl -fsS --proto '=https' -o "$dir/SHA256SUMS" "$IMAGE_URL/SHA256SUMS"
        curl -fsS --proto '=https' -o "$dir/SHA256SUMS.gpg" "$IMAGE_URL/SHA256SUMS.gpg"
        gnupg=$(mktemp -d)
        GNUPGHOME=$gnupg gpg --batch -q --keyserver hkps://keyserver.ubuntu.com --recv-keys "$IMAGE_KEY"
        GNUPGHOME=$gnupg gpg --batch -q --verify "$dir/SHA256SUMS.gpg" "$dir/SHA256SUMS" \
            || die "SHA256SUMS does not verify against $IMAGE_KEY"
        rm -rf "$gnupg"
        grep -q "^$IMAGE_SHA256 \*$IMAGE_NAME\$" "$dir/SHA256SUMS" \
            || die "the signed SHA256SUMS does not carry the pinned digest for $IMAGE_NAME"
        curl -fsS --proto '=https' -o "$IMAGE.part" "$IMAGE_URL/$IMAGE_NAME"
        mv "$IMAGE.part" "$IMAGE"
    fi
    echo "$IMAGE_SHA256  $IMAGE" | sha256sum -c --quiet || die "$IMAGE is not the pinned image"
}

ensure_key() {
    [ -f "$KEY" ] || ssh-keygen -q -t ed25519 -N '' -C weaver-measure -f "$KEY"
}

# write_seed DIR NAME INSTANCE: a NoCloud seed at DIR/seed.iso. The guest's one login
# is GUEST_USER with the measurement key and passwordless sudo. Before any network
# service starts, the guest drops every new connection it would open to the addresses
# user-mode networking maps onto the host's loopback (10.0.2.2, and fec0::2 where IPv6
# is on), so the guest cannot reach a service the host binds to 127.0.0.1 or ::1;
# replies on connections the host opens (the SSH forward) still pass. The template and
# clone of 2026-11-11 were built with the IPv4 line alone (README, "The network").
write_seed() {
    local dir=$1 name=$2 instance=$3 pub
    pub=$(cat "$KEY.pub")
    mkdir -p "$dir/seed"
    printf 'instance-id: %s\nlocal-hostname: %s\n' "$instance" "$name" >"$dir/seed/meta-data"
    cat >"$dir/seed/user-data" <<EOF
#cloud-config
bootcmd:
  - [sh, -c, "nft list table inet hostguard >/dev/null 2>&1 || nft -f - <<'NFT'\ntable inet hostguard {\n  chain out {\n    type filter hook output priority 0; policy accept;\n    ct state established,related accept\n    ip daddr 10.0.2.2 counter drop\n    ip6 daddr fec0::2 counter drop\n  }\n}\nNFT"]
users:
  - name: $GUEST_USER
    shell: /bin/bash
    sudo: "ALL=(ALL) NOPASSWD:ALL"
    lock_passwd: true
    ssh_authorized_keys:
      - $pub
ssh_pwauth: false
package_update: false
package_upgrade: false
EOF
    rm -f "$dir/seed.iso"
    xorriso -as mkisofs -quiet -o "$dir/seed.iso" -V cidata -J -r \
        "$dir/seed/user-data" "$dir/seed/meta-data" 2>/dev/null
}

# define_guest NAME DISK SEED_ISO VCPUS MEMORY_MIB SSH_PORT NVRAM [extra virt-install args]
# CPUSET, where set, keeps the guest's vCPUs and QEMU's own threads on those host CPUs
# (libvirt's <vcpu cpuset>), so the guest stays off CPUs another measurement holds.
# User-mode networking given to QEMU directly, with SSH forwarded from host loopback
# port SSH_PORT: libvirt's NAT network gets no DHCP, DNS or forwarding on the measuring
# host (README, "The network"), and libvirt's own port forwarding needs passt. IPv6 is
# off: QEMU's user-mode IPv6 maps fec0::2 onto the host's ::1. The template and clone of
# 2026-11-11 were defined before ipv6=off was added and had it on.
define_guest() {
    local name=$1 disk=$2 seed=$3 vcpus=$4 mem=$5 port=$6 nvram=$7
    shift 7
    virt-install --connect "$URI" --name "$name" --memory "$mem" \
        --vcpus "$vcpus${CPUSET:+,cpuset=$CPUSET}" \
        --osinfo "$OS_VARIANT" --machine q35 --cpu host-passthrough \
        --boot "firmware=efi,firmware.feature0.name=secure-boot,firmware.feature0.enabled=no,nvram=$nvram" \
        --disk "path=$disk,format=qcow2,bus=virtio,discard=unmap" \
        --disk "path=$seed,device=cdrom,bus=sata" \
        --network none \
        --qemu-commandline="-netdev user,id=un0,ipv6=off,hostfwd=tcp:127.0.0.1:$port-:22 -device virtio-net-pci,netdev=un0,bus=pcie.0,addr=0x0a" \
        --graphics none --console pty,target.type=serial --noautoconsole --import "$@"
}

ssh_opts() { # ssh_opts PORT
    printf '%s\n' -i "$KEY" -p "$1" -o BatchMode=yes -o ConnectTimeout=5 \
        -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR \
        -o ServerAliveInterval=30 -o ServerAliveCountMax=10
}
gssh() { # gssh PORT command...
    local port=$1; shift
    local opts; mapfile -t opts < <(ssh_opts "$port")
    ssh "${opts[@]}" "$GUEST_USER@127.0.0.1" "$@"
}
gscp() { # gscp PORT src... dest  (dest is a guest path)
    local port=$1; shift
    local opts; mapfile -t opts < <(ssh_opts "$port")
    opts=("${opts[@]/#-p/-P}")
    local dest=${*: -1}
    scp -q "${opts[@]}" "${@:1:$#-1}" "$GUEST_USER@127.0.0.1:$dest"
}

wait_ssh() { # wait_ssh PORT [seconds]
    local port=$1 limit=${2:-300} t=0
    until gssh "$port" true 2>/dev/null; do
        t=$((t + 5)); [ "$t" -lt "$limit" ] || return 1; sleep 5
    done
    gssh "$port" 'cloud-init status --wait >/dev/null 2>&1; cloud-init status'
}

# run_step PORT LOG NAME command: runs a guest command, its output and exit status
# kept in LOG/NAME.txt; prints the status line. Never stops the caller.
run_step() {
    local port=$1 log=$2 name=$3; shift 3
    local rc=0 t0 t1
    t0=$(date -u +%s)
    {
        printf '# step %s, started %s\n# guest command: %s\n' "$name" "$(date -u +%FT%TZ)" "$*"
        gssh "$port" "$@" || rc=$?
        t1=$(date -u +%s)
        printf '\n# step %s, exit %s, %ss\n' "$name" "$rc" "$((t1 - t0))"
    } >"$log/$name.txt" 2>&1
    printf '%s: exit %s (%s)\n' "$name" "$rc" "$log/$name.txt"
    return 0
}
