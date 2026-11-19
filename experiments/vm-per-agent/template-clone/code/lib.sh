# Sourced by build-template.sh and clone.sh: the measurement's pins, and the host-side
# helpers that define a guest, reach it over SSH under its pinned host key, judge its
# boots and log each step. template-clone-Spec.md states what they do.
#
# VM_ROOT names the directory that holds images/, template/, appliance/, clones/, logs/ and
# keys/. It has no default here: a default would be a box path in the repository. Each
# script sets KNOWN_HOSTS to its guest's own known-hosts file before it reaches the guest.

: "${VM_ROOT:?set VM_ROOT to the directory holding images/ template/ appliance/ clones/ logs/ keys/}"
# Canonical (absolute, with no ., .., symbolic link or trailing slash), so that the paths
# built on it compare equal to the ones libvirt reports (a storage pool's path, a domain's
# disk) and to each other.
VM_ROOT=$(realpath -m -- "$VM_ROOT") || { printf '\nREFUSED: VM_ROOT %s cannot be made canonical\n' "$VM_ROOT" >&2; exit 1; }

URI=qemu:///system
# The guest operating system, a named input (OS): its cloud image by digest, checked
# against a digest file signed by the image's key, and the key pinned by fingerprint
# before the first fetch.
# - arch-20261115: the arch-boxes cloud image v20261115.625457, release 2.0's base (the
#   operator's ruling of 2026-11-16), which build-template.sh builds. Its .SHA256 file is
#   signed by the Arch Linux image signing key, its primary key's fingerprint below. The
#   key was read on 2026-11-16 from three sources that agree: the arch-boxes README (the
#   key block added on 2022-05-29 in its commit b3919744a4), keyserver.ubuntu.com by
#   fingerprint, and the Arch Linux infrastructure repository's misc/keys/README.md. Its
#   signing subkey expires on 2027-05-28: an image pinned later needs the key read again.
#   The digest file's signature must be made by a key under that primary (gpg's VALIDSIG
#   line names it last, sig_primary_ok).
# - 26.04: Ubuntu's cloud image as released on 2026-10-20, release 1.0's base, kept for
#   clone.sh, which names no OS and gets 26.04, and the #21 template it clones. Release 1.0
#   itself is rebuilt from this tree at WeaverTools main 06d42bd, its stack WeaverAgent at
#   the tag appliance/1.0.
# 24.04 is named and refused until its image is pinned (weaver-appliance-build-Spec,
# section 1).
OS=${OS:-26.04}
case $OS in
    arch-20261115)
        IMAGE_NAME=Arch-Linux-x86_64-cloudimg-20261115.625457.qcow2
        IMAGE_URL=https://geo.mirror.pkgbuild.com/images/v20261115.625457
        IMAGE_SHA256=dbd02830f6eb0fe07ef6e5edaddbae8b577ce4296d9e79118904704cce0a1f60
        IMAGE_KEY=1B9A16984A4E8CB448712D2AE0B78BF4326C6F8F
        OS_VARIANT=archlinux ;;
    26.04)
        IMAGE_NAME=ubuntu-26.04-server-cloudimg-amd64.img
        IMAGE_URL=https://cloud-images.ubuntu.com/releases/resolute/release
        IMAGE_SHA256=5645d0318ad21f643ebb1ac4567514620e9893380b07b0201fda163baa5b7eb5
        IMAGE_KEY=D2EB44626FDDC30B513D5BB71A5D6C4C7DB87C81
        OS_VARIANT=ubuntu26.04 ;;
    24.04)
        printf '\nREFUSED: no Ubuntu 24.04 cloud image is pinned yet (weaver-appliance-build-Spec, section 1)\n' >&2
        exit 1 ;;
    *)
        printf '\nREFUSED: OS is arch-20261115 or 26.04 (or 24.04 once pinned), not %s\n' "$OS" >&2
        exit 1 ;;
esac
IMAGE=$VM_ROOT/images/$IMAGE_NAME
KEY=${KEY:-$VM_ROOT/keys/measure_ed25519}
GUEST_USER=op
# The URL prefixes a driver round may fetch from (Spec section 6), space-separated, each
# ending "/": by default the template's NVIDIA repository (guest/template-steps' NV_REPO),
# which served every file of the 2026-11-12 slice's two rounds.
DEB_ORIGINS=${DEB_ORIGINS:-https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2604/x86_64/}

virsh_() { virsh -c "$URI" "$@"; }
say() { printf '\n== %s\n' "$*"; }
die() { printf '\nREFUSED: %s\n' "$*" >&2; exit 1; }

# fetch_image: the cloud image, its signed digest file and that file's signature, each
# checked, by the base's own route.
fetch_image() {
    case $OS in
        arch-*) fetch_image_arch ;;
        *) fetch_image_ubuntu ;;
    esac
}
# sig_primary_ok STATUS FPR: 0 where gpg's --status-fd output STATUS names a good signature
# (GOODSIG) made by a key whose primary key is FPR (VALIDSIG's last field), and no bad,
# expired, revoked or unchecked signature. Any other STATUS is 1.
sig_primary_ok() {
    awk -v want="$2" '
        $1 != "[GNUPG:]" { next }
        $2 == "GOODSIG" { good = 1 }
        $2 == "VALIDSIG" { if ($NF == want) valid = 1; else bad = 1 }
        $2 ~ /^(BADSIG|ERRSIG|EXPSIG|EXPKEYSIG|REVKEYSIG|NO_PUBKEY)$/ { bad = 1 }
        END { exit !(good && valid && !bad) }' <<<"$1"
}
# fetch_image_arch: the image's .SHA256 file and its detached signature fetched and kept
# beside the image, the signature checked against IMAGE_KEY's primary fingerprint in a
# keyring of this fetch's own, the file checked to name the pinned digest, then the image
# fetched and its digest checked.
fetch_image_arch() {
    local dir=$VM_ROOT/images gnupg st
    mkdir -p "$dir"
    if [ ! -f "$IMAGE" ]; then
        curl -fsS --proto '=https' -o "$IMAGE.SHA256" "$IMAGE_URL/$IMAGE_NAME.SHA256"
        curl -fsS --proto '=https' -o "$IMAGE.SHA256.sig" "$IMAGE_URL/$IMAGE_NAME.SHA256.sig"
        gnupg=$(mktemp -d)
        GNUPGHOME=$gnupg gpg --batch -q --keyserver hkps://keyserver.ubuntu.com --recv-keys "$IMAGE_KEY" \
            || { rm -rf -- "${gnupg:?}"; die "the image key $IMAGE_KEY could not be fetched"; }
        st=$(GNUPGHOME=$gnupg gpg --batch -q --status-fd 1 --verify "$IMAGE.SHA256.sig" "$IMAGE.SHA256" 2>/dev/null) || true
        rm -rf -- "${gnupg:?}"
        sig_primary_ok "$st" "$IMAGE_KEY" \
            || die "$IMAGE_NAME.SHA256 does not verify as signed under the primary key $IMAGE_KEY"
        [ "$(cat "$IMAGE.SHA256")" = "$IMAGE_SHA256  $IMAGE_NAME" ] \
            || die "the signed $IMAGE_NAME.SHA256 does not carry the pinned digest alone"
        curl -fsS --proto '=https' -o "$IMAGE.part" "$IMAGE_URL/$IMAGE_NAME"
        mv "$IMAGE.part" "$IMAGE"
    fi
    echo "$IMAGE_SHA256  $IMAGE" | sha256sum -c --quiet || die "$IMAGE is not the pinned image"
}
# fetch_image_ubuntu: the cloud image, its SHA256SUMS and their signature, each checked.
fetch_image_ubuntu() {
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

# ensure_key: the measurement's login key, made once where absent, its directory 0700.
ensure_key() {
    [ -f "$KEY" ] && return 0
    ( umask 077; mkdir -p -- "$(dirname -- "$KEY")" ) || die "no directory for $KEY"
    ssh-keygen -q -t ed25519 -N '' -C weaver-measure -f "$KEY"
}

# guest_keys DIR PORT: the guest's host key pair, made here under DIR/hostkey (the private
# key 0600), and DIR/known_hosts pinning its public key for the SSH forward on PORT. The
# seed hands the pair to the guest (template-clone-Spec.md, section 2).
guest_keys() {
    local dir=$1 port=$2 hk=$1/hostkey/ssh_host_ed25519_key
    [ ! -e "$dir/hostkey" ] || die "$dir/hostkey already stands"
    ( umask 077; mkdir -p "$dir/hostkey"
      ssh-keygen -q -t ed25519 -N '' -C "guest host key" -f "$hk" )
    printf '[127.0.0.1]:%s %s\n' "$port" "$(cut -d' ' -f1,2 "$hk.pub")" >"$dir/known_hosts"
}

# write_seed DIR NAME INSTANCE LOG: a NoCloud seed at DIR/seed.iso, made 0600.
# - The guest's one login is GUEST_USER with the measurement key and passwordless sudo.
# - Before any network service starts, the guest drops every new connection it would open
#   to the addresses user-mode networking maps onto the host's loopback (10.0.2.2, and
#   fec0::2 where IPv6 is on), so it cannot reach a service the host binds to 127.0.0.1 or
#   ::1; replies on connections the host opens (the SSH forward) still pass. The template
#   and clone of 2026-11-11 were built with the IPv4 line alone (README, "The network").
# - The guest's host key is the pair guest_keys made: cloud-init deletes any other, writes
#   this one, and generates only the type named, which this key already is. The template's
#   sshd-keygen.service mask (guest/template-steps, seal) keeps first boot from making the
#   other types; a clone of the template of 2026-11-11 still makes them.
# The seed's sources hold the private key and are removed once the image is made. LOG
# keeps the user-data with the private key replaced, and the meta-data.
write_seed() {
    local dir=$1 name=$2 instance=$3 log=$4 pub hk=$1/hostkey/ssh_host_ed25519_key
    [ -f "$hk" ] && [ -f "$hk.pub" ] || die "no host key at $hk; guest_keys makes it"
    pub=$(cat "$KEY.pub")
    (
        umask 077
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
ssh_deletekeys: true
ssh_genkeytypes: [ed25519]
ssh_keys:
  ed25519_private: |
$(sed 's/^/      /' "$hk")
  ed25519_public: $(cut -d' ' -f1,2 "$hk.pub")
package_update: false
package_upgrade: false
EOF
        rm -f "$dir/seed.iso"
        xorriso -as mkisofs -quiet -o "$dir/seed.iso" -V cidata -J -r \
            "$dir/seed/user-data" "$dir/seed/meta-data" 2>/dev/null
        sed '/ed25519_private/,/END OPENSSH PRIVATE KEY/c\  ed25519_private: <the host key, not kept here>' \
            "$dir/seed/user-data" >"$log/seed-user-data-redacted.txt"
        cp "$dir/seed/meta-data" "$log/seed-meta-data.txt"
        rm -rf "${dir:?}/seed"
    )
    [ -s "$dir/seed.iso" ] || die "the seed image was not made"
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

# SSH and scp under the guest's pinned host key (Spec section 2): checking on, the guest's
# own known-hosts file, no global one. A guest that answers with another key is refused.
ssh_opts() { # ssh_opts PORT
    printf '%s\n' -i "$KEY" -p "$1" -o BatchMode=yes -o ConnectTimeout=5 \
        -o StrictHostKeyChecking=yes -o UserKnownHostsFile="$KNOWN_HOSTS" \
        -o GlobalKnownHostsFile=/dev/null -o LogLevel=ERROR \
        -o ServerAliveInterval=30 -o ServerAliveCountMax=10
}
# known_hosts_set: checked in the caller's shell, not in ssh_opts's subshell, where a
# refusal would leave ssh with no options at all.
known_hosts_set() {
    [ -n "${KNOWN_HOSTS:-}" ] && [ -f "$KNOWN_HOSTS" ] \
        || die "KNOWN_HOSTS names no known-hosts file ('${KNOWN_HOSTS:-}'); the guest's host key is not pinned"
}
gssh() { # gssh PORT command...
    local port=$1; shift
    known_hosts_set
    local opts; mapfile -t opts < <(ssh_opts "$port")
    ssh "${opts[@]}" "$GUEST_USER@127.0.0.1" "$@"
}
gscp() { # gscp PORT src... dest  (dest is a guest path)
    local port=$1; shift
    known_hosts_set
    local opts; mapfile -t opts < <(ssh_opts "$port")
    opts=("${opts[@]/#-p/-P}")
    local dest=${*: -1}
    scp -q "${opts[@]}" "${@:1:$#-1}" "$GUEST_USER@127.0.0.1:$dest"
}

# put PORT SRCDIR GUEST_DEST: SRCDIR's contents into GUEST_DEST, a guest directory that
# does not yet stand, as a tar stream, so links arrive as links (Spec section 5). The
# slice's scp -r nested its source in a destination that stood, and followed links.
put() {
    [ "$#" = 3 ] || die "put PORT SRCDIR GUEST_DEST"
    local port=$1 src=$2 dest=$3
    [ -d "$src" ] || die "put takes a directory: $src"
    [[ "$dest" =~ ^[A-Za-z0-9][A-Za-z0-9._/-]*$ ]] && [[ "$dest" != *..* ]] \
        || die "put will not write to '$dest'"
    tar -C "$src" -cf - . | gssh "$port" "test ! -e '$dest' && mkdir -p -- '$dest' && tar --warning=no-timestamp -C '$dest' -xf -" \
        || die "put: $src did not land as $dest in the guest (a $dest that already stands is refused)"
}

# ssh_up PORT SECONDS: 0 once a command runs under the pinned key, 1 if SECONDS pass first.
# SSH is a boot's gate; cloud-init is judged apart (Spec section 3).
ssh_up() {
    local port=$1 limit=$2 t=0
    # Here, not inside the loop: the probe's stderr goes to /dev/null, and a refusal there
    # would end the caller without a word.
    known_hosts_set
    until gssh "$port" true 2>/dev/null; do
        t=$((t + 5)); [ "$t" -lt "$limit" ] || return 1; sleep 5
    done
}

# ci_judge JSON: cloud-init's verdict on one boot, from `cloud-init status --format json`.
# Exit 0 when the status is done, errors are empty and every recoverable item is on the
# allow-list. It prints each allowed item's name, and the first line of each item that
# stops it. The seed names its key type, so a schema item is not expected and stops.
ci_judge() {
    python3 -I - "$1" <<'PY'
import json, re, sys
# The allow-list, by name: level, name, the pattern the item's text must match.
# "network-wait" cost about 120 s on each boot of the 2026-11-12 slice's clone; its cause
# is not proven.
ALLOWED = [
    ("ERROR", "network-wait",
     r"^Failed to wait for network: Unexpected error while running command\.\s+"
     r"Command: \['systemctl', 'start', 'systemd-networkd-wait-online\.service'\]\s+Exit code: 1\b"),
]
try:
    s = json.load(open(sys.argv[1]))
except (OSError, ValueError) as e:
    print(f"cloud-init: no status to judge: {e}")
    sys.exit(1)
if not isinstance(s, dict):
    print(f"cloud-init: the status is not a mapping: {s!r}")
    sys.exit(1)
if s.get("status") != "done" or s.get("errors"):
    print(f"cloud-init: status {s.get('status')!r}, errors {s.get('errors')!r}")
    sys.exit(1)
rec = s.get("recoverable_errors") or {}
if not isinstance(rec, dict):
    print(f"cloud-init: recoverable_errors is not a mapping: {rec!r}")
    sys.exit(1)
bad, seen = [], set()
for level, items in rec.items():
    for text in items if isinstance(items, list) else [items]:
        text = text if isinstance(text, str) else repr(text)
        name = next((n for lv, n, pat in ALLOWED if lv == level and re.search(pat, text, re.S)), None)
        if name is None:
            bad.append(f"{level}: {text.splitlines()[0] if text else repr(text)}")
        else:
            seen.add(name)
for b in bad:
    print(f"cloud-init: recoverable item not on the allow-list: {b}")
print(f"cloud-init: allowed recoverable items: {', '.join(sorted(seen)) or 'none'}")
sys.exit(1 if bad else 0)
PY
}

# cloud_init_verdict PORT LOG TAG: records cloud-init's status for one boot as
# LOG/cloud-init-TAG.txt (long, with its exit) and .json, and on exit 2 the schema's error
# lines as -schema.txt for the record, with no line that could carry key text: the stored
# user-data holds the guest's private host key. Exit 0 passes; exit 2 passes on ci_judge;
# any other exit stops. The wait has a deadline, CI_WAIT seconds (900 by default): past it
# the step exits 124 and stops, since a cloud-init still running would read exit 0.
cloud_init_verdict() {
    local port=$1 out=$2/cloud-init-$3 rc=0 wait=${CI_WAIT:-900}
    [[ "$wait" =~ ^[1-9][0-9]*$ ]] || die "CI_WAIT is a positive number of seconds, not '$wait' (timeout 0 sets no deadline)"
    gssh "$port" "timeout $wait cloud-init status --wait >/dev/null 2>&1; w=\$?; cloud-init status --long; s=\$?
                  [ \$w -ne 124 ] || { echo \"the wait passed its deadline, $wait s\"; exit 124; }; exit \$s" \
        >"$out.txt" 2>&1 || rc=$?
    printf 'exit %s\n' "$rc" >>"$out.txt"
    gssh "$port" 'cloud-init status --format json' >"$out.json" 2>/dev/null || true
    echo "cloud-init ($3): $(grep -m1 -E '^(extended_status|status):' "$out.txt"), exit $rc ($out.txt)"
    case $rc in
        0) return 0 ;;
        2)
            gssh "$port" 'bash -s' >"$out-schema.txt" 2>&1 <<'SCHEMA' || true
sudo -n cloud-init schema --system 2>&1 | grep -v -E 'BEGIN|END|^[[:space:]]*[A-Za-z0-9+/=]{16,}[[:space:]]*$' | grep -i -E 'error|invalid|not valid|deprecat|schema|E[0-9]+:'
SCHEMA
            ci_judge "$out.json" ;;
        *) return 1 ;;
    esac
}

# boot_ok PORT LOG TAG SECONDS: a boot's gate and its verdict, both kept in LOG/boot.txt.
# Stops the caller when SSH is not up under the pinned key within SECONDS (one attempt's
# error shown), or when cloud-init's verdict stops it.
boot_ok() {
    local port=$1 log=$2 tag=$3 limit=$4 opts
    if ! ssh_up "$port" "$limit"; then
        echo "no SSH under the pinned key within $limit s; one attempt with its error:"
        mapfile -t opts < <(ssh_opts "$port")
        ssh "${opts[@]/LogLevel=ERROR/LogLevel=INFO}" "$GUEST_USER@127.0.0.1" true 2>&1 | tail -5
        die "SSH did not come up"
    fi
    cloud_init_verdict "$port" "$log" "$tag" | tee -a "$log/boot.txt" \
        || die "SSH is up, but cloud-init's verdict on this boot stops here; see $log/boot.txt"
}

# netdev_ok VALUE PORT: 0 when VALUE is a user-mode -netdev argument the scripts accept:
# it starts user,id=un0, every hostfwd in it binds the host's 127.0.0.1, and one of them is
# the SSH forward, hostfwd=tcp:127.0.0.1:PORT-:22. A forward bound to any other host
# address would offer the guest's port beyond this host.
netdev_ok() {
    local value=$1 port=$2 opt ssh=no
    [[ "$value" == user,id=un0,* ]] || return 1
    local IFS=,
    for opt in $value; do
        case $opt in
            hostfwd=*)
                [[ "$opt" =~ ^hostfwd=(tcp|udp):127\.0\.0\.1:[0-9]+-[^,]*:[0-9]+$ ]] || return 1
                [ "$opt" != "hostfwd=tcp:127.0.0.1:$port-:22" ] || ssh=yes ;;
        esac
    done
    [ "$ssh" = yes ]
}

# guest_env VAR...: an `env VAR=value ...` prefix for a guest command, each value quoted
# for the guest's shell, naming only the variables that are set here. The guest steps'
# parameters (gpu-steps' DRIVER_PIN, DRIVER_OPEN, HARNESS_DIR, TOOLCHAIN_READING) reach
# them this way, and a value with spaces arrives whole.
guest_env() {
    local v out=""
    for v in "$@"; do
        [[ "$v" =~ ^[A-Z_][A-Z0-9_]*$ ]] || die "guest_env takes variable names, not '$v'"
        [ -n "${!v+set}" ] || continue
        out+=" $v=$(printf '%q' "${!v}")"
    done
    [ -z "$out" ] || printf 'env%s ' "$out"
}

# image_open DISK DIR: DISK's root filesystem, read-only, as the file DIR/root.raw, which
# qemu-storage-daemon exports over FUSE as this user: a FUSE mount this user owns, with no
# root and no loop device. The root partition is the one GPT names "Linux root (x86-64)"
# (the arch-boxes image's type 8304 and Ubuntu's alike). Its filesystem's type, read by
# blkid from the export, is kept in DIR/fs, and the image_* reads below go by it: ext4
# (release 1.0, the #21 template) through debugfs, btrfs (release 2.0) through btrfs
# restore. Neither follows a symbolic link. Nothing here writes to an image: the proof
# hands its first boot a network as a credential, not a file. image_close stops the
# export.
image_open() {
    local disk=$1 dir=$2 part start size pid ro=on wr=off lock=off fs
    mkdir -p -- "$dir"
    : >"$dir/whole.raw"; : >"$dir/root.raw"
    qemu-storage-daemon \
        --blockdev "driver=file,node-name=f,filename=$disk,read-only=on,locking=off" \
        --blockdev driver=qcow2,node-name=q,file=f,read-only=on \
        --export "type=fuse,id=w,node-name=q,mountpoint=$dir/whole.raw,writable=off,allow-other=off" \
        --pidfile "$dir/qsd-whole.pid" --daemonize || die "no export of $disk"
    part=$(sfdisk -J "$dir/whole.raw" | python3 -I -c '
import json, sys
p = [p for p in json.load(sys.stdin)["partitiontable"]["partitions"]
     if p["type"].upper() == "4F68BCE3-E8CD-4DB1-96E7-FBCAF984B709"]
print(p[0]["start"], p[0]["size"]) if len(p) == 1 else sys.exit(1)') || part=""
    { pid=$(qsd_pid "$dir" qsd-whole) && kill "$pid"; } 2>/dev/null || true; sleep 1
    [ -n "$part" ] || die "$disk has no single Linux root partition"
    read -r start size <<<"$part"
    qemu-storage-daemon \
        --blockdev "driver=file,node-name=f,filename=$disk,read-only=$ro,locking=$lock" \
        --blockdev "driver=qcow2,node-name=q,file=f,read-only=$ro" \
        --blockdev "driver=raw,node-name=p,file=q,offset=$((start * 512)),size=$((size * 512)),read-only=$ro" \
        --export "type=fuse,id=r,node-name=p,mountpoint=$dir/root.raw,writable=$wr,allow-other=off" \
        --pidfile "$dir/qsd-root.pid" --daemonize || die "no export of $disk's root partition"
    fs=$(blkid -p -o value -s TYPE "$dir/root.raw" 2>/dev/null) || fs=""
    case $fs in
        ext4|btrfs) printf '%s\n' "$fs" >"$dir/fs" ;;
        *) image_close "$dir"; die "$disk's root partition holds '${fs:-no filesystem blkid knows}', not ext4 or btrfs" ;;
    esac
}
# image_fs DIR: the root filesystem's type image_open read, ext4 or btrfs. Otherwise a line
# on stderr and status 2.
image_fs() {
    local fs
    fs=$(cat "$1/fs" 2>/dev/null)
    case $fs in
        ext4|btrfs) printf '%s\n' "$fs" ;;
        *) echo "image reads: $1 was not opened by image_open (no filesystem type recorded)" >&2; return 2 ;;
    esac
}
# The reads, each by the filesystem's type: image_state, image_get, image_cat, image_ls,
# image_files and image_resolve below take the same arguments and give the same statuses
# on either. An expected absence is image_state's call (status 1) or image_get_present's,
# never an empty result: on either, image_get, image_cat and image_files refuse by name an
# absent path, status 2. image_ls answers 1 for one, as image_state does.
image_state() {
    local f
    f=$(image_fs "$1") || return 2
    "image_state_$f" "$@"
}
image_get() {
    local f
    f=$(image_fs "$1") || return 2
    "image_get_$f" "$@"
}
image_cat() {
    local f
    f=$(image_fs "$1") || return 2
    "image_cat_$f" "$@"
}
image_ls() {
    local f
    f=$(image_fs "$1") || return 2
    "image_ls_$f" "$@"
}
image_files() {
    local f
    f=$(image_fs "$1") || return 2
    "image_files_$f" "$@"
}
image_resolve() {
    local f
    f=$(image_fs "$1") || return 2
    "image_resolve_$f" "$@"
}
# image_get_present DIR PATH OUT: PATH copied to OUT where image_state finds it (0), nothing
# where image_state finds it absent (1), and 2 where either read fails. The one read for a
# file whose absence is expected (a seal's SEAL-REFUSED).
image_get_present() {
    local rc=0
    rm -f -- "$3"
    image_state "$1" "$2" || rc=$?
    [ "$rc" = 0 ] || return "$rc"
    image_get "$1" "$2" "$3" || return 2
}
# The ext4 reads, through debugfs. They give the btrfs reads' statuses row for row (the
# table in test/host/t-btrfs-reads.sh, which runs each row on both; the Planner's call of
# 2026-11-17 on CodeRabbit's review of pull request 35). debugfs exits 0 whatever it
# finds, so each read goes by what its stat prints. debugfs resolves a link above a path
# inside the image, so each read finds the path with image_find_ext4, which tests the
# directories above it as image_came_out does on btrfs and refuses a path under a link.
#
# image_stat_ext4 DIR PATH: debugfs's stat of PATH on stdout. stat follows no link at PATH
# itself. 0 where PATH is found, 1 where it is not, 2, the cause on stderr, where the image
# could not be read (no debugfs, a filesystem it cannot open, any other error).
image_stat_ext4() {
    local o
    command -v debugfs >/dev/null 2>&1 || { echo "image reads: no debugfs on this host" >&2; return 2; }
    o=$(debugfs -c -R "stat \"$2\"" "$1/root.raw" 2>&1) || { printf 'image reads: debugfs: %s\n' "$(tail -1 <<<"$o")" >&2; return 2; }
    if grep -q '^Inode: ' <<<"$o"; then printf '%s\n' "$o"; return 0; fi
    if grep -q 'File not found by ext2_lookup' <<<"$o"; then return 1; fi
    printf 'image reads: %s: %s\n' "$2" "$(grep -v '^debugfs [0-9]' <<<"$o" | tail -1)" >&2
    return 2
}
# image_find_ext4 DIR PATH: debugfs's stat of PATH on stdout, the directories above PATH
# tested first, from the top down, so that no stat passes through a link. 0 where PATH is
# found; 1 where it is absent: it is not there, or a directory above it is not, or is no
# directory; 2, the cause on stderr, where a directory above it is a link, or a read fails.
image_find_ext4() {
    local a="" rest=${2#/} o
    while [ "${rest#*/}" != "$rest" ]; do
        a=$a/${rest%%/*}; rest=${rest#*/}
        o=$(image_stat_ext4 "$1" "$a") || return $?
        if grep -q 'Type: symlink' <<<"$o"; then
            echo "image reads: $2 lies under $a, a link in the image, which the read does not follow" >&2
            return 2
        fi
        grep -q 'Type: directory' <<<"$o" || return 1
    done
    image_stat_ext4 "$1" "$2"
}
# image_state_ext4 DIR PATH: 0 where PATH is in the image, 1 where it is absent, 2 where
# that cannot be told: a read that fails, or a path under a link.
image_state_ext4() { image_find_ext4 "$1" "$2" >/dev/null; }
# image_get_ext4 DIR PATH OUT: PATH, a regular file, copied out of the image to OUT. 2, the
# cause on stderr, where PATH is absent (named, as on btrfs), lies under a link, is not a
# regular file, or could not be read whole (a copy short of the inode's size).
image_get_ext4() {
    local dir=$1 path=$2 out=$3 rc=0 o size
    rm -f -- "$out"
    o=$(image_find_ext4 "$dir" "$path") || rc=$?
    [ "$rc" != 1 ] || { echo "image_get: $path is absent from the image" >&2; return 2; }
    [ "$rc" = 0 ] || return 2
    grep -q 'Type: regular' <<<"$o" || { echo "image_get: $path is not a regular file" >&2; return 2; }
    size=$(sed -n 's/.*[[:space:]]Size: \([0-9][0-9]*\).*/\1/p' <<<"$o" | sed -n 1p)
    debugfs -c -R "dump \"$path\" \"$out\"" "$dir/root.raw" >/dev/null 2>&1
    if [ ! -f "$out" ] || [ -z "$size" ] || [ "$(stat -c %s "$out")" != "$size" ]; then
        echo "image_get: $path: copied $(stat -c %s "$out" 2>/dev/null || echo 0) of ${size:-unknown} bytes" >&2
        rm -f -- "$out"
        return 2
    fi
}
# image_cat_ext4 DIR PATH: PATH's bytes on stdout; image_get_ext4's refusals.
image_cat_ext4() {
    local t rc=0
    t=$(mktemp "$1/cat.XXXXXX") || return 2
    image_get_ext4 "$1" "$2" "$t" && cat -- "$t" || rc=2
    rm -f -- "$t"
    return "$rc"
}
# image_ls_ext4 DIR PATH: debugfs's "ls -p" lines for the directory PATH. 1 where PATH is
# absent, 2, the cause on stderr, where it lies under a link, is not a directory, or cannot
# be read or listed. A directory always lists . and .., so a listing with no line failed.
image_ls_ext4() {
    local o
    o=$(image_find_ext4 "$1" "$2") || return $?
    grep -q 'Type: directory' <<<"$o" || { echo "image_ls: $2 is not a directory in the image" >&2; return 2; }
    o=$(debugfs -c -R "ls -p \"$2\"" "$1/root.raw" 2>/dev/null)
    [ -n "$o" ] || { echo "image_ls: $2 could not be listed" >&2; return 2; }
    printf '%s\n' "$o"
}
# image_files_ext4 DIR PATH: "SIZE PATH" for each regular file under the directory PATH, one
# per line, depth bounded at 12. 2, the cause on stderr, where PATH is absent (named, as on
# btrfs), lies under a link, or is not a directory. debugfs ls -p prints
# /inode/mode/uid/gid/name/size/ per entry, the mode in six octal digits (040 a directory,
# 100 a regular file; a link, 120, is neither listed nor followed). debugfs exits 0 on a
# listing it cannot make and says so on stderr, so its lines are read together: its banner
# is passed over, and any other line, or a directory past the bound, ends the listing with
# status 2 and a line on stderr. A listing that did not complete is not an empty one. A
# directory debugfs lists as empty and reports nothing about is empty here too.
image_files_ext4() {
    local dir=$1 path=${2%/} depth=${3:-0} o l ino mode uid gid name size rest rc=0
    if [ "$depth" = 0 ]; then
        o=$(image_find_ext4 "$dir" "$path") || rc=$?
        [ "$rc" != 1 ] || { echo "image_files: $path is absent from the image" >&2; return 2; }
        [ "$rc" = 0 ] || return 2
        grep -q 'Type: directory' <<<"$o" || { echo "image_files: $path is not a directory" >&2; return 2; }
    fi
    [ "$depth" -lt 12 ] || { echo "image_files: $path is past the depth bound of 12" >&2; return 2; }
    o=$(debugfs -c -R "ls -p \"$path\"" "$dir/root.raw" 2>&1) \
        || { echo "image_files: debugfs exited non-zero on $path" >&2; return 2; }
    while IFS= read -r l; do
        case $l in
            /*/*/*/*/*/*/*) ;;
            '' | 'debugfs '[0-9]* | 'catastrophic mode'*) continue ;;
            *) printf 'image_files: %s: %s\n' "$path" "$l" >&2; return 2 ;;
        esac
        IFS=/ read -r _ ino mode uid gid name size rest <<<"$l"
        case $name in '' | . | ..) continue ;; esac
        case $mode in
            040*) image_files_ext4 "$dir" "$path/$name" $((depth + 1)) || return 2 ;;
            100*) printf '%s %s\n' "${size:-0}" "$path/$name" ;;
        esac
    done <<<"$o"
}
# The btrfs reads, through btrfs restore, which copies the paths its --path-regex matches out
# of the filesystem into a directory, links as links (-S), and says nothing of a path it
# did not find. Each read therefore restores into a new directory of its own under DIR,
# always with /etc beside what it asks for, and checks what came out: /etc not a directory
# there means the read itself did not work (status 2), and a path asked for that did not
# come out is refused by name, except by image_state, whose call an absence is. No owner,
# mode or time is restored or compared: no offline check reads one.
#
# image_path_regex PATH...: the POSIX extended regex that matches exactly each PATH and each
# directory above it, and, for a PATH ending "/**", everything under it. Each name is
# matched as written, with every regex metacharacter escaped. A PATH that is not absolute,
# is "/" itself, or holds an empty, "." or ".." component is refused, status 2.
image_path_regex() {
    python3 -I - "$@" <<'PY'
import sys
def esc(s):
    return "".join("\\" + c if c in ".[]()*+?{}|^$\\" else c for c in s)
alts = {""}
for p in sys.argv[1:]:
    sub = p.endswith("/**")
    q = p[:-3] if sub else p
    parts = q[1:].split("/") if q.startswith("/") else None
    if not parts or any(c in ("", ".", "..") for c in parts) or "\n" in q:
        print(f"image_path_regex: '{p}' is not an absolute path of plain names", file=sys.stderr)
        sys.exit(2)
    for i in range(1, len(parts) + 1):
        alts.add(esc("/".join(parts[:i])))
    if sub:
        alts.add(esc("/".join(parts)) + "/.*")
if len(sys.argv) < 2:
    print("image_path_regex: no path asked for", file=sys.stderr)
    sys.exit(2)
print("^/(" + "|".join(sorted(alts)) + ")$")
PY
}
# image_restore DIR PATH...: PATHs, and /etc, restored out of the btrfs root filesystem
# opened at DIR into a new directory under DIR, whose name it prints. btrfs restore asks
# on its terminal whether to go on with a file of many extents, so it is answered "a"
# (always) from a file. 2, the cause on stderr, where the regex cannot be made, the
# restore fails, or /etc did not come out as a directory.
image_restore() {
    local dir=$1 out re rc=0; shift
    command -v btrfs >/dev/null 2>&1 || { echo "image reads: no btrfs on this host" >&2; return 2; }
    re=$(image_path_regex /etc "$@") || return 2
    out=$(mktemp -d "$dir/restore.XXXXXX") || { echo "image reads: no directory under $dir" >&2; return 2; }
    printf 'a\n%.0s' {1..4096} >"$out.answers"
    btrfs restore -S --path-regex "$re" "$dir/root.raw" "$out" <"$out.answers" >"$out.log" 2>&1 || rc=$?
    rm -f -- "$out.answers"
    if [ "$rc" != 0 ] || [ ! -d "$out/etc" ] || [ -L "$out/etc" ]; then
        echo "image reads: btrfs restore exited $rc, and /etc $([ -d "$out/etc" ] && echo came out || echo did not come out): $(tail -2 "$out.log" | tr '\n' ' ')" >&2
        rm -rf -- "${out:?}" "$out.log"
        return 2
    fi
    rm -f -- "$out.log"
    printf '%s\n' "$out"
}
# image_came_out OUT PATH: 0 where PATH came out into OUT, as a file, a directory or a link;
# 1 where it did not; 2, the cause on stderr, where a directory above it came out as a
# link. Each directory above PATH is tested from the top down with -L, which follows no
# link at the name it tests, so no test here passes through a link: a link restored out of
# the image keeps its target as stored, and on the host an absolute one (/etc/d -> /etc)
# leads out of OUT, to the host's own files (CodeRabbit on pull request 35, 2026-11-17).
image_came_out() {
    local a="" rest=${2#/}
    while [ "${rest#*/}" != "$rest" ]; do
        a=$a/${rest%%/*}; rest=${rest#*/}
        if [ -L "$1$a" ]; then
            echo "image reads: $2 lies under $a, a link in the image, which the read does not follow" >&2
            return 2
        fi
    done
    [ -L "$1$2" ] || [ -e "$1$2" ]
}
# image_state_btrfs DIR PATH: 0 where PATH is in the image, 1 where it is absent, 2 where
# that cannot be told. Absent means PATH did not come out and no directory above it came
# out as a link (a path through a link is not read, status 2).
image_state_btrfs() {
    local out rc=0
    out=$(image_restore "$1" "$2") || return 2
    image_came_out "$out" "$2" || rc=$?
    rm -rf -- "${out:?}"
    return "$rc"
}
# image_get_btrfs DIR PATH OUT: PATH, a regular file, copied out of the image to OUT. 2, the
# cause on stderr, where PATH did not come out (named), lies under a link, or is not a
# regular file.
image_get_btrfs() {
    local dir=$1 path=$2 out=$3 r rc=0 c=0
    rm -f -- "$out"
    r=$(image_restore "$dir" "$path") || return 2
    image_came_out "$r" "$path" || c=$?
    if [ "$c" = 2 ]; then
        rc=2
    elif [ "$c" != 0 ]; then
        echo "image_get: $path did not come out of the image (absent, or not the name asked for)" >&2; rc=2
    elif [ -L "$r$path" ] || [ ! -f "$r$path" ]; then
        echo "image_get: $path is not a regular file" >&2; rc=2
    else
        cp -- "$r$path" "$out" || rc=2
    fi
    rm -rf -- "${r:?}"
    return "$rc"
}
# image_cat_btrfs DIR PATH: PATH's bytes on stdout; image_get_btrfs's refusals.
image_cat_btrfs() {
    local t rc=0
    t=$(mktemp "$1/cat.XXXXXX") || return 2
    image_get_btrfs "$1" "$2" "$t" && cat -- "$t" || rc=2
    rm -f -- "$t"
    return "$rc"
}
# image_ls_btrfs DIR PATH: one line for each entry of the directory PATH, "TYPE NAME" (find's
# %y letters), with " -> TARGET" for a link, in C-locale order. 1 where image_state finds
# PATH absent, 2 where it cannot tell, PATH did not come out, or is not a directory.
image_ls_btrfs() {
    local r rc=0
    image_state_btrfs "$1" "$2" || return $?
    r=$(image_restore "$1" "$2/**") || return 2
    if image_came_out "$r" "$2" && [ ! -L "$r$2" ] && [ -d "$r$2" ]; then
        find "$r$2" -mindepth 1 -maxdepth 1 -printf '%y %f\0%l\n' | sed 's/\x0\(.\)/ -> \1/; s/\x0$//' | LC_ALL=C sort
    else
        echo "image_ls: $2 did not come out of the image as a directory" >&2; rc=2
    fi
    rm -rf -- "${r:?}"
    return "$rc"
}
# image_files_btrfs DIR PATH: "SIZE PATH" for each regular file under the directory PATH,
# one a line, in C-locale order by path. 2, the cause on stderr, where PATH did not come out
# (named) or is not a directory. An empty directory lists nothing and is status 0.
image_files_btrfs() {
    local dir=$1 path=${2%/} r rc=0 c=0
    r=$(image_restore "$dir" "$path/**") || return 2
    image_came_out "$r" "$path" || c=$?
    if [ "$c" = 2 ]; then
        rc=2
    elif [ "$c" != 0 ]; then
        echo "image_files: $path did not come out of the image (absent, or not the name asked for)" >&2; rc=2
    elif [ -L "$r$path" ] || [ ! -d "$r$path" ]; then
        echo "image_files: $path is not a directory" >&2; rc=2
    else
        (cd "$r" && find ".$path" -type f -printf '%s %P\0' | LC_ALL=C sort -z -t ' ' -k 2) \
            | while IFS= read -r -d '' l; do printf '%s %s/%s\n' "${l%% *}" "$path" "${l#* }"; done
    fi
    rm -rf -- "${r:?}"
    return "$rc"
}
# image_link_climbs LINK TARGET: 0, the refusal on stderr, where TARGET holds a ".."
# component. The reads never normalize such a target: a climbing path is what they do not
# try to prove safe (the Planner's ruling of 2026-11-17, on CodeRabbit's review of pull
# request 35), so the link is refused, not resolved, and no read is made through it.
image_link_climbs() {
    case /$2/ in
        */../*) echo "image_resolve: $1 is a link to $2, which climbs with '..': refused, not resolved" >&2; return 0 ;;
    esac
    return 1
}
# image_resolve_btrfs DIR PATH: PATH with each symbolic link in the image followed, 8 at
# most, to a path that came out as no link. 1, the cause on stderr, where the path or a
# link's target did not come out. 2, refused, where a link's target climbs with "..", a
# path lies under a link, the read fails, or there are more than 8 links, so a refused
# link is never reported missing.
image_resolve_btrfs() {
    local dir=$1 p=$2 n=0 r t c
    while [ "$n" -lt 8 ]; do
        r=$(image_restore "$dir" "$p") || return 2
        c=0; image_came_out "$r" "$p" || c=$?
        if [ "$c" != 0 ]; then
            rm -rf -- "${r:?}"
            [ "$c" = 2 ] && return 2
            echo "image_resolve: $p did not come out of the image" >&2
            return 1
        fi
        if [ ! -L "$r$p" ]; then rm -rf -- "${r:?}"; printf '%s\n' "$p"; return 0; fi
        t=$(readlink -- "$r$p"); rm -rf -- "${r:?}"
        ! image_link_climbs "$p" "$t" || return 2
        case $t in /*) p=$t ;; *) p=${p%/*}/$t ;; esac
        n=$((n + 1))
    done
    echo "image_resolve: more than 8 links from $2: refused" >&2
    return 2
}
# image_late_writes DIR: what a sealed image holds that its seal's final probe refuses, so
# written after that probe (at the power-off): systemd's random seed, its credential secret,
# and any non-empty file under /var/log. Each is a PROOF-FAIL line, and so is a listing of
# /var/log that does not complete. The login records under /var/lib (wtmpdb, lastlog) are
# listed for the measure and not judged.
image_late_writes() {
    local dir=$1 f p rc files
    for f in /var/lib/systemd/random-seed /var/lib/systemd/credential.secret; do
        rc=0; image_state "$dir" "$f" 2>&1 || rc=$?
        case $rc in
            0) echo "PROOF-FAIL: $f is in the sealed image, written after the final probe" ;;
            1) echo "absent: $f" ;;
            *) echo "PROOF-FAIL: the offline read could not tell whether $f is in the sealed image" ;;
        esac
    done
    rc=0; image_state "$dir" /var/log 2>&1 || rc=$?
    if [ "$rc" != 0 ]; then
        echo "PROOF-FAIL: the offline read could not list /var/log in the sealed image"
    else
        # image_files's own line goes to stderr, which the proof's record also takes.
        rc=0; files=$(image_files "$dir" /var/log) || rc=$?
        if [ "$rc" != 0 ]; then
            echo "PROOF-FAIL: the offline read could not list every directory under /var/log in the sealed image"
        else
            printf '%s\n' "$files" | awk '$1 > 0 { s = $1; sub(/^[0-9]+ /, "")
                print "PROOF-FAIL: a log written after the final probe: " $0 " (" s " bytes)"; n++ }
                END { if (!n) print "no non-empty file under /var/log" }'
        fi
    fi
    for p in /var/lib/wtmpdb /var/lib/lastlog; do
        echo "== $p, listed and not judged"
        rc=0; image_state "$dir" "$p" 2>/dev/null || rc=$?
        case $rc in
            0) { image_files "$dir" "$p" 2>&1 || echo "(this listing did not complete)"; } | sed 's/^/  /' ;;
            1) echo "  (absent)" ;;
            *) echo "  (whether it stands could not be read)" ;;
        esac
    done
}
# The ggml CUDA library the image ships, and the device code it must carry (the operator's
# decision 6 of 2026-11-15): cubins for exactly these architectures and no PTX. The list
# is WeaverAgent's (CMAKE_CUDA_ARCHITECTURES "86-real;89-real;120a-real;121a-real" in its
# deploy scripts, from its commit 1f6ef77e), held here for the proof to check against. It
# moves when WeaverAgent's does. CUOBJDUMP lists the device code, from a CUDA toolkit that
# knows all four architectures.
GGML_CUDA_LIB=/opt/weaver/lib/libggml-cuda.so.0
GGML_CUBIN_ARCHS="sm_86 sm_89 sm_120a sm_121a"
CUOBJDUMP=${CUOBJDUMP:-cuobjdump}
# image_resolve_ext4 DIR PATH: PATH with each symbolic link in the image followed, 8 at
# most, to a path that is not a link, with image_resolve_btrfs's statuses: 1, the cause on
# stderr, where the path or a link's target is absent; 2, refused, where a target climbs
# with "..", cannot be read, a path lies under a link, a read fails, or there are more than
# 8 links. A link that is there is never reported missing, and an absent path never comes
# back as found. debugfs follows no link at the path it stats, so each target is read from
# the stat's "Fast link dest:" line, or, for a target over 60 bytes, which ext4 keeps in a
# block and stat does not print, from the link's own block with cat, its length checked
# against the inode's size.
image_resolve_ext4() {
    local dir=$1 p=$2 n=0 o t rc size
    while [ "$n" -lt 8 ]; do
        rc=0; o=$(image_find_ext4 "$dir" "$p") || rc=$?
        [ "$rc" != 1 ] || { echo "image_resolve: $p is absent from the image" >&2; return 1; }
        [ "$rc" = 0 ] || return 2
        grep -q 'Type: symlink' <<<"$o" || { printf '%s\n' "$p"; return 0; }
        t=$(sed -n 's/^Fast link dest: "\(.*\)"$/\1/p' <<<"$o" | sed -n 1p)
        if [ -z "$t" ]; then
            size=$(sed -n 's/.*[[:space:]]Size: \([0-9][0-9]*\).*/\1/p' <<<"$o" | sed -n 1p)
            t=$(debugfs -c -R "cat \"$p\"" "$dir/root.raw" 2>/dev/null)
            if [ -z "$size" ] || [ -z "$t" ] || [ "$(printf '%s' "$t" | wc -c)" != "$size" ]; then
                echo "image_resolve: $p is a link whose target cannot be read" >&2; return 2
            fi
        fi
        ! image_link_climbs "$p" "$t" || return 2
        case $t in /*) p=$t ;; *) p=${p%/*}/$t ;; esac
        n=$((n + 1))
    done
    echo "image_resolve: more than 8 links from $2: refused" >&2
    return 2
}
# device_code_check DIR TMP: the shipped ggml CUDA library, followed through its links in
# the image opened at DIR, copied to TMP, and listed with CUOBJDUMP: no PTX image, cubins
# for each architecture of GGML_CUBIN_ARCHS, the same count for each, and none for any
# other. Each architecture's count is printed, and no count is fixed. A library that cannot
# be read, a listing that cannot run (cuobjdump exits 255 on a file with no device code),
# and a cubin line out of form are findings too. Each finding is a PROOF-FAIL line. The last
# line, "device code: checked", is printed only where every step ran; the caller fails a
# record that has neither it nor a finding.
device_code_check() {
    local dir=$1 tmp=$2 real rc=0 ptx elf np ne archs a n fails=0
    real=$(image_resolve "$dir" "$GGML_CUDA_LIB") \
        || { echo "PROOF-FAIL: $GGML_CUDA_LIB could not be followed to a file in the image"; return 0; }
    image_get "$dir" "$real" "$tmp/libggml-cuda.so" || rc=$?
    [ "$rc" = 0 ] || { echo "PROOF-FAIL: $real could not be read out of the image (image_get exited $rc)"; return 0; }
    echo "$GGML_CUDA_LIB -> $real, $(stat -c %s "$tmp/libggml-cuda.so") bytes, sha256 $(sha256sum "$tmp/libggml-cuda.so" | cut -d' ' -f1)"
    if ! command -v "$CUOBJDUMP" >/dev/null 2>&1; then
        rm -f -- "$tmp/libggml-cuda.so"
        echo "PROOF-FAIL: no $CUOBJDUMP on this host, so the device code could not be listed"
        return 0
    fi
    echo "listed with $CUOBJDUMP $("$CUOBJDUMP" --version 2>&1 | grep -oE 'V[0-9][0-9.]*' | sed -n 1p || true)"
    rc=0; ptx=$("$CUOBJDUMP" --list-ptx "$tmp/libggml-cuda.so" 2>&1) || rc=$?
    [ "$rc" = 0 ] || { echo "PROOF-FAIL: $CUOBJDUMP --list-ptx exited $rc: $(tail -1 <<<"$ptx")"; fails=1; }
    rc=0; elf=$("$CUOBJDUMP" --list-elf "$tmp/libggml-cuda.so" 2>&1) || rc=$?
    [ "$rc" = 0 ] || { echo "PROOF-FAIL: $CUOBJDUMP --list-elf exited $rc: $(tail -1 <<<"$elf")"; fails=1; }
    rm -f -- "$tmp/libggml-cuda.so"
    [ "$fails" = 0 ] || return 0
    # Every line of each listing must be in a form this check reads, so a cuobjdump that
    # words its listing otherwise is a finding and never reads as "no PTX" (the Planner's
    # review of 2026-11-15, F7). With no PTX, cuobjdump says so in one info line, and with
    # no PTX image named that line must stand (cuobjdump 13.4.92, measured 2026-11-15).
    local odd
    odd=$(grep -vE '^(PTX file +[0-9]+: |cuobjdump info +: No PTX file found)' <<<"$ptx" | grep -v '^$' || true)
    [ -z "$odd" ] || echo "PROOF-FAIL: a --list-ptx line out of the forms 'PTX file N: NAME' and 'cuobjdump info : No PTX file found': $(head -1 <<<"$odd")"
    odd=$(grep -vE '^ELF file +[0-9]+: ' <<<"$elf" | grep -v '^$' || true)
    [ -z "$odd" ] || echo "PROOF-FAIL: a --list-elf line out of the form 'ELF file N: NAME': $(head -1 <<<"$odd")"
    np=$(grep -cE '^PTX file +[0-9]+: ' <<<"$ptx" || true)
    echo "PTX images: $np"
    [ "$np" != 0 ] || grep -qE '^cuobjdump info +: No PTX file found' <<<"$ptx" \
        || echo "PROOF-FAIL: --list-ptx named no PTX image and did not say it found none"
    grep -oE '\.sm_[0-9]+[a-z]?\.ptx$' <<<"$ptx" | sed 's/^\.//; s/\.ptx$//' | sort | uniq -c | sed 's/^/  /' || true
    [ "$np" = 0 ] || echo "PROOF-FAIL: libggml-cuda carries $np PTX image(s); the image ships none"
    ne=$(grep -cE '^ELF file +[0-9]+: ' <<<"$elf" || true)
    archs=$(sed -n 's/^ELF file *[0-9]*: .*\.\(sm_[0-9][0-9]*[a-z]*\)\.cubin$/\1/p' <<<"$elf")
    echo "cubins: $ne"
    sort <<<"$archs" | uniq -c | sed 's/^/  /' || true
    [ "$ne" -gt 0 ] || echo "PROOF-FAIL: libggml-cuda carries no cubin"
    [ "$(grep -c . <<<"$archs" || true)" = "$ne" ] || echo "PROOF-FAIL: a cubin line out of the form 'ELF file N: NAME.sm_ARCH.cubin'"
    # The same count of cubins for each architecture, whatever that count is: an object
    # compiled for fewer architectures than the rest leaves some cards without its kernels.
    # An absent architecture is its own finding, so only the present ones are compared.
    local per="" first=""
    for a in $GGML_CUBIN_ARCHS; do
        n=$(grep -cx -- "$a" <<<"$archs" || true)
        per="$per $a=$n"
        [ "$n" -gt 0 ] || { echo "PROOF-FAIL: libggml-cuda carries no cubin for $a"; continue; }
        [ -n "$first" ] || first=$n
        [ "$n" = "$first" ] || first=uneven
    done
    echo "cubins per architecture:$per"
    [ "$first" != uneven ] || echo "PROOF-FAIL: libggml-cuda carries a different count of cubins for its architectures:$per"
    for a in $(sort -u <<<"$archs"); do
        case " $GGML_CUBIN_ARCHS " in *" $a "*) ;; *) echo "PROOF-FAIL: libggml-cuda carries a cubin for $a, off the list ($GGML_CUBIN_ARCHS)" ;; esac
    done
    echo "device code: checked"
}
# qsd_pid DIR NAME: the PID DIR/NAME.pid names, printed only where that process is a
# qemu-storage-daemon whose command line exports this directory's file (DIR/whole.raw for
# qsd-whole, DIR/root.raw for qsd-root). A pidfile left by a crash or a SIGKILL may name
# another process by now, which is never sent a signal.
qsd_pid() {
    local pid f a0 c
    pid=$(cat "$1/$2.pid" 2>/dev/null) && [[ "$pid" =~ ^[0-9]+$ ]] || return 1
    case $2 in qsd-whole) f=$1/whole.raw ;; qsd-root) f=$1/root.raw ;; *) return 1 ;; esac
    [ -r "/proc/$pid/cmdline" ] || return 1
    c=$(tr '\0' '\n' 2>/dev/null <"/proc/$pid/cmdline") || return 1
    a0=${c%%$'\n'*}
    [ "${a0##*/}" = qemu-storage-daemon ] || return 1
    grep -qF -- "mountpoint=$f," <<<"$c" || return 1
    printf '%s\n' "$pid"
}
# image_close stops both exports where they still run (each only through qsd_pid), unmounts
# one whose daemon is gone, and never fails its caller (it runs from cleanup traps).
image_close() {
    local f pid pids="" t
    for f in qsd-whole qsd-root; do
        { pid=$(qsd_pid "$1" "$f") && kill "$pid" && pids+=" $pid"; } 2>/dev/null || true
    done
    sleep 1
    # A writable export flushes its writes as it exits: each daemon is waited for, 10 s at
    # most, before the file is opened again.
    for pid in $pids; do
        t=0; while kill -0 "$pid" 2>/dev/null && [ "$t" -lt 20 ]; do sleep 0.5; t=$((t + 1)); done
    done
    for f in whole.raw root.raw; do
        ! findmnt -n --mountpoint "$1/$f" >/dev/null 2>&1 || fusermount3 -u "$1/$f" 2>/dev/null || true
    done
    rm -f -- "$1/whole.raw" "$1/root.raw" "$1/qsd-whole.pid" "$1/qsd-root.pid" 2>/dev/null || true
}

# compact_check FILE: whether a qcow2 is compact (the operator's decision 18 of 2026-11-15):
# its end offset, as qemu-img check reads it, set against its allocated clusters times its
# cluster size. What lies beyond them is the format's own tables. The bound is
# COMPACT_SLACK, for the header, the first-level and refcount tables, plus the most the
# second-level tables can take, which grows with the disk: one 8-byte entry per cluster
# of the virtual size (15 MiB for 120 GiB in 64 KiB clusters, 64 MiB for 512 GiB). Prints
# the figures, and a PROOF-FAIL line where the file is longer than that, names a backing
# file, or cannot be checked, and then returns 1. qemu-img check opens the file read-only.
COMPACT_SLACK=$((64 * 1024 * 1024))
compact_check() {
    local f=$1 chk info rc=0
    chk=$(qemu-img check --output=json "$f" 2>&1) || rc=$?
    if [ "$rc" != 0 ]; then
        echo "PROOF-FAIL: qemu-img check exited $rc on $f (1 not checked, 2 corrupt, 3 leaked clusters): $(tail -2 <<<"$chk" | tr '\n' ' ')"
        return 1
    fi
    info=$(qemu-img info --output=json "$f" 2>&1) || { echo "PROOF-FAIL: qemu-img info could not read $f"; return 1; }
    python3 -I - "$chk" "$info" "$COMPACT_SLACK" <<'PY'
import json, sys
try:
    c, i, slack = json.loads(sys.argv[1]), json.loads(sys.argv[2]), int(sys.argv[3])
    if i.get("backing-filename"):
        print(f"PROOF-FAIL: the file names a backing file, {i['backing-filename']}")
        sys.exit(1)
    # qemu-img check leaves allocated-clusters out where none is allocated.
    end, alloc, cs = int(c["image-end-offset"]), int(c.get("allocated-clusters", 0)), int(i["cluster-size"])
    vs = int(i["virtual-size"])
    if cs <= 0:
        raise ValueError(f"cluster size {cs}")
except (ValueError, KeyError, TypeError) as e:
    print(f"PROOF-FAIL: qemu-img's figures could not be read: {e!r}")
    sys.exit(1)
data, l2 = alloc * cs, -(-vs // cs) * 8
bound = slack + l2
print(f"end offset {end} bytes; {alloc} clusters of {cs} bytes allocated ({data} bytes); "
      f"{end - data} bytes beyond them, bound {bound} ({slack}, and {l2} for the "
      f"second-level tables of a {vs}-byte disk)")
if end - data > bound:
    print(f"PROOF-FAIL: the file is not compact: its end lies {end - data} bytes beyond its allocated clusters, more than {bound}")
    sys.exit(1)
PY
}

# inbound_probe PORT SECONDS: what one TCP connection to the host's loopback PORT meets,
# through QEMU's user-mode forward to a guest port. It reads for 2 s first (a server that
# speaks first, as SSH does), then sends one HTTP request line and reads again, for SECONDS
# in all. It prints "answered: " and the first line the far end sent, "closed" where the
# far end ended the connection with nothing sent, or "silent" where nothing came. Through
# the forward the host's connection is accepted at once whatever the guest does, then a
# guest that refuses (a reset) ends it at once, and a guest that drops the connection
# leaves it silent until QEMU gives up, long after SECONDS.
inbound_probe() {
    python3 -I - "$1" "$2" <<'PY'
import socket, sys, time
port, secs = int(sys.argv[1]), float(sys.argv[2])
end = time.monotonic() + secs
try:
    s = socket.create_connection(("127.0.0.1", port), timeout=secs)
except OSError as e:
    print(f"closed (the connection was refused: {e.strerror or e})")
    sys.exit(0)
def read(until):
    while (left := until - time.monotonic()) > 0:
        s.settimeout(left)
        try:
            b = s.recv(4096)
        except socket.timeout:
            return None
        except OSError as e:
            return f"closed ({e.strerror or e})"
        if not b:
            return "closed"
        line = b.split(b"\n")[0].decode("ascii", "replace").strip()
        return "answered: " + "".join(c if c.isprintable() else "?" for c in line)[:120]
    return None
got = read(min(end, time.monotonic() + 2))
if got is None:
    try:
        s.sendall(b"GET / HTTP/1.0\r\n\r\n")
    except OSError as e:
        got = f"closed ({e.strerror or e})"
if got is None:
    got = read(end)
print(got or "silent")
PY
}

# refusal_judge OUT RC LABEL: one SSH login as root that must be refused, judged from
# ssh -v's output (the file OUT) and its exit status RC. Refused: exit 255, at least one
# "Authentications that can continue" line, each offering publickey alone, and "Permission
# denied (publickey)". A line saying the login went through is a finding, as is any other
# method offered or a server that named none (a connection cut before authentication, as
# sshd's PerSourcePenalties cuts one, proves nothing). Prints one line, "refused: LABEL ..."
# or a PROOF-FAIL line, and returns 0 either way: a missing methods line is a finding, not
# grep's status 1 ending the proof under errexit and pipefail before its verdict.
refusal_judge() {
    local out=$1 rc=$2 label=$3 m bad=""
    m=$({ grep -oE 'Authentications that can continue: [^[:space:]]*' "$out" || true; } | sed 's/^.*: //' | sort -u)
    if [ -z "$m" ]; then bad+="the server named no methods; "
    elif [ "$m" != publickey ]; then bad+="the server offered $(tr '\n' ' ' <<<"$m" | sed 's/ $//'); "
    fi
    grep -qE 'Authenticated to |Authentication succeeded' "$out" && bad+="the login went through; "
    grep -qF 'Permission denied (publickey)' "$out" || bad+="no 'Permission denied (publickey)'; "
    [ "$rc" = 255 ] || bad+="ssh exited $rc, not 255; "
    if [ -z "$bad" ]; then echo "refused: $label (the server offers publickey alone)"
    else echo "PROOF-FAIL: SSH as root with $label: ${bad%; }"; fi
}

# console_judge LOG HOST: the proof's first login at the console, judged from the driver's
# LOG (console-drive.py: the console's bytes, each send marked on a line of its own), with
# the terminal's escape sequences and carriage returns taken out. The login prompt before
# root is sent greets as HOST. After root: no password asked and no refusal, the forced
# change announced and a new password asked; after the first password, the same asked
# again; after the second, no error from the change. Then the session's read line names
# /dev/ttyS0, root, root's password usable (P) and one authorized key. Prints what it read,
# then a PROOF-FAIL line for each finding.
console_judge() {
    python3 -I - "$1" "$2" <<'PY'
import re, sys
log, host = sys.argv[1], sys.argv[2]
raw = open(log, "rb").read().decode("utf-8", "replace")
esc = re.compile(r"\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)|\x1bP.*?\x1b\\|\x1b\[[0-9;?<=>!]*[ -/]*[@-~]|\x1b[()][0-9A-Za-z]|\x1b[=>78cDEHM]", re.S)
text = esc.sub("", raw).replace("\r", "")
parts = re.split(r"\n\[sent: (.*?)\]\n", text)
outs, sents = parts[0::2], parts[1::2]
fails = []
try:
    i = sents.index("root\\r")
except ValueError:
    i = None
    fails.append("root was never sent at the console")
if i is not None:
    before, after_root = outs[i], outs[i + 1]
    prompts = re.findall(r"(?m)^(\S+ login:)", before)
    seen = prompts[-1] if prompts else "none"
    print("the prompt before root: %s" % seen)
    if seen != host + " login:":
        fails.append("the console's prompt before root is '%s', not '%s login:'" % (seen, host))
    if re.search(r"(?m)^\s*Password:", after_root):
        fails.append("login asked root for a password")
    if "Login incorrect" in after_root:
        fails.append("login refused root")
    if "You are required to change your password immediately" in after_root:
        print("the forced change: announced")
    else:
        fails.append("login did not announce the forced change")
    if "New password:" not in after_root:
        fails.append("no new password was asked")
    rest = sents[i + 1:]
    if len(rest) < 2 or not rest[0].startswith("<file ") or not rest[1].startswith("<file "):
        fails.append("the new password was not given twice")
    else:
        if "Retype new password:" not in outs[i + 2]:
            fails.append("the new password was not asked again")
        tail = "".join(outs[i + 3:])
        for e in ("Authentication token manipulation error", "BAD PASSWORD", "do not match",
                  "Login incorrect", "password unchanged", "maximum number of retries"):
            if e in outs[i + 2] + tail:
                fails.append("the change said '%s'" % e)
        reads = re.findall(r"(?m)^READ tty=(\S+) user=(\S+) root=(\S+) keys=(\S+)\s*$", tail)
        if not reads:
            fails.append("the session's read line is absent")
        else:
            tty, user, st, keys = reads[-1]
            print("the session: tty %s, user %s, root's password %s, %s authorized key(s)" % (tty, user, st, keys))
            if tty != "/dev/ttyS0":
                fails.append("the session's terminal is %s, not /dev/ttyS0" % tty)
            if user != "root":
                fails.append("the session's account is %s, not root" % user)
            if st != "P":
                fails.append("root's password reads %s after the change, not P" % st)
            if keys != "1":
                fails.append("root holds %s authorized key(s), not 1" % keys)
for m in fails:
    print("PROOF-FAIL: the first login: " + m)
PY
}

# qemu_file FILE: an empty FILE that its owner and QEMU's account (QEMU_USER) alone may
# write, for a file QEMU opens from its own command line (a serial console, a capture).
# A file libvirt opens for a device becomes its own, unreadable here without root; a mode
# open to every local account would let any of them write the console that trust on first
# use is read from. The grant is a POSIX ACL, which the filesystem must support. The file
# is made new, exclusively and without following a link: an existing file or a symbolic
# link at that path is refused, never reused or written through. Any ACL entry the file
# inherits from its directory's default ACL is removed before the grant: chmod only masks
# an inherited entry, and the grant recomputes the mask, which would let it write again.
qemu_file() {
    [ ! -e "$1" ] && [ ! -L "$1" ] || die "$1 already stands, or is a symbolic link: qemu_file makes a new file only"
    python3 -I -c 'import os, sys; os.close(os.open(sys.argv[1], os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600))' "$1" \
        && chmod 0600 "$1" && setfacl -b "$1" && setfacl -m "u:${QEMU_USER:-libvirt-qemu}:rw" "$1" \
        || die "cannot make $1, new, writable by its owner and ${QEMU_USER:-libvirt-qemu} alone"
}
# qemu_dir DIR: a new directory in which QEMU makes the console's socket. Only its owner and
# QEMU_USER may enter or write it, by a POSIX ACL, made as qemu_file makes a file; its
# default ACL lets the owner connect to the socket QEMU makes there. An existing path or a
# symbolic link is refused.
qemu_dir() {
    [ ! -e "$1" ] && [ ! -L "$1" ] || die "$1 already stands, or is a symbolic link: qemu_dir makes a new directory only"
    mkdir -m 0700 -- "$1" && setfacl -b "$1" && setfacl -k "$1" \
        && setfacl -m "u:${QEMU_USER:-libvirt-qemu}:rwx" "$1" && setfacl -d -m "u:$(id -un):rwx" "$1" \
        || die "cannot make $1, new, open to its owner and ${QEMU_USER:-libvirt-qemu} alone"
}

# The MANIFEST's form has one definition, guest/template-steps' LINKS_MARK and
# MANIFEST_SCOPE, read from that file here. links_format is the formatter text that file
# carries, word for word (the offline mock checks that the two copies are equal): it takes
# find's NUL-separated path and target pairs on stdin and writes the links section's lines.
# manifest_links prints a MANIFEST's links section.
LIB_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
LINKS_MARK=$(sed -n "s/^LINKS_MARK='\(.*\)'\$/\1/p" "$LIB_DIR/guest/template-steps")
# The ready layer as step 10 lays it, read for the proof's ready row from the one place it
# is defined (weaver-appliance-build-Spec section 2).
READY_ACCOUNTS=$(sed -n "s/^READY_ACCOUNTS='\(.*\)'\$/\1/p" "$LIB_DIR/guest/template-steps")
READY_DIRS=$(sed -n "s/^READY_DIRS='\(.*\)'\$/\1/p" "$LIB_DIR/guest/template-steps")
READY_SURVEY=$(sed -n "s/^READY_SURVEY=\(.*\)\$/\1/p" "$LIB_DIR/guest/template-steps")
READY_UNIT=$(sed -n "s/^READY_UNIT=\(.*\)\$/\1/p" "$LIB_DIR/guest/template-steps")
MANIFEST_SCOPE=$(sed -n "s/^MANIFEST_SCOPE='\(.*\)'\$/\1/p" "$LIB_DIR/guest/template-steps")
# The sealed image's hostname has its one definition there too.
IMAGE_HOSTNAME=$(sed -n "s/^IMAGE_HOSTNAME='\(.*\)'\$/\1/p" "$LIB_DIR/guest/template-steps")
# And the two data mount points the proof reads at the first boot.
DATA_MOUNTS=$(sed -n "s/^DATA_MOUNTS='\(.*\)'\$/\1/p" "$LIB_DIR/guest/template-steps")
[ -n "$LINKS_MARK" ] && [ -n "$MANIFEST_SCOPE" ] && [ -n "$IMAGE_HOSTNAME" ] && [ -n "$DATA_MOUNTS" ] \
    || die "no LINKS_MARK, MANIFEST_SCOPE, IMAGE_HOSTNAME or DATA_MOUNTS in $LIB_DIR/guest/template-steps"
links_format() {
    local p t
    while IFS= read -r -d '' p && IFS= read -r -d '' t; do
        p=${p//\\/\\\\}; p=${p//$'\t'/\\t}; p=${p//$'\n'/\\n}
        t=${t//\\/\\\\}; t=${t//$'\t'/\\t}; t=${t//$'\n'/\\n}
        printf '%s\t%s\n' "$p" "$t"
    done | LC_ALL=C sort -u
}
manifest_links() { M=$LINKS_MARK awk '$0 == ENVIRON["M"] { l = 1; next } l' "$1"; }

# The proof's own network, handed to its first boot as a systemd credential (proof_network),
# the credential's name, and the MAC address its NIC is given. systemd-network-generator
# writes a credential named network.network.NAME to /run/systemd/network/NAME.network, out
# of the MANIFEST's scope.
PROOF_CREDENTIAL=network.network.50-weaver-proof
PROOF_MAC=52:54:00:57:45:01
# What the proof's first boot changes in the MANIFEST's scope by design, with no seed and no
# cloud-init (decision 16 of 2026-11-15). Changed: the machine identity, which the seal
# leaves uninitialized, and the shadow file, where the first login at the console sets
# root's password (the operator's ruling of 2026-11-16). Added: the host keys' public
# halves, which sshdgenkeys makes (the private halves are never listed), and the account
# tools' lock file, which any account tool leaves where none stands. The proof's network
# comes as a credential and lands under /run. The fourth and fifth builds' first boots ran
# cloud-init from a seed, which also changed the account files and the hostname and added
# its own netplan file and sudoers rule; none of those is expected now. No link is expected
# to change. The offline mock names its own. Added by the request unit at the first boot
# (weaver-appliance-build-Spec section 2, the ready layer): each connector's config and its
# request. A config holds the connector's private key, so every file in the connectors' own
# folders (READY_PATH_ONLY) is listed by its path alone and never digested.
FIRST_BOOT_CHANGES=${FIRST_BOOT_CHANGES:-/etc/machine-id /etc/shadow}
FIRST_BOOT_ADDS=${FIRST_BOOT_ADDS:-/etc/ssh/ssh_host_*_key.pub /etc/.pwd.lock /etc/weaver-web/pairing/gate/gate-con.toml /etc/weaver-web/pairing/admin/admin-con.toml /etc/weaver-web/pairing/exchange/gate.req /etc/weaver-web/pairing/exchange/admin.req}
READY_PATH_ONLY='/etc/weaver-web/pairing/gate /etc/weaver-web/pairing/admin'
# manifest_compare SEALED FILES LINKS [PATHS]: the sealed MANIFEST set against a booted
# guest's listing of the same scope: FILES as sha256sum writes them, LINKS as links_format
# writes them, and PATHS, where given, files listed by their path alone, one a line, never
# digested (READY_PATH_ONLY's). Each difference is printed by its path, never with a digest: a file or link
# removed, added or changed, marked "(expected)" where FIRST_BOOT_CHANGES names a changed
# file or FIRST_BOOT_ADDS (shell patterns) an added one. Every other difference is a
# PROOF-FAIL line, as are a MANIFEST that does not carry the marker once (an earlier form,
# or a marker that drifted) and a line out of form on either side.
manifest_compare() {
    M=$LINKS_MARK FB_CHANGES=$FIRST_BOOT_CHANGES FB_ADDS=$FIRST_BOOT_ADDS python3 -I - "$1" "$2" "$3" ${4:+"$4"} <<'PY'
import fnmatch, os, re, sys
mark, changes, adds = os.environ["M"], set(os.environ["FB_CHANGES"].split()), os.environ["FB_ADDS"].split()
def read(p):
    t = open(p, encoding="utf-8", errors="surrogateescape").read().split("\n")
    return t[:-1] if t and t[-1] == "" else t
sealed = read(sys.argv[1])
if sealed.count(mark) != 1:
    print("PROOF-FAIL: the sealed MANIFEST does not carry the links marker once (an earlier form, or a marker that drifted)")
    sys.exit(0)
k, form, fails = sealed.index(mark), re.compile(r"^(\\?)([0-9a-f]{64})  (.+)$"), 0
def fail(msg):
    global fails
    print("PROOF-FAIL: " + msg); fails += 1
def files(lines, side):
    d = {}
    for l in lines:
        m = form.match(l)
        if m: d[m.group(1) + m.group(3)] = m.group(2)
        else: fail(f"a line of the {side} file list is out of form")
    return d
def links(lines, side):
    d = {}
    for l in lines:
        f = l.split("\t")
        if len(f) == 2: d[f[0]] = f[1]
        else: fail(f"a line of the {side} links list is out of form")
    return d
sf, sl = files(sealed[:k], "sealed"), links(sealed[k + 1:], "sealed")
bf, bl = files(read(sys.argv[2]), "booted"), links(read(sys.argv[3]), "booted")
# A file listed by its path alone stands with no digest, so it can be added or removed but
# never shown unchanged against a digest the seal recorded.
for p in (read(sys.argv[4]) if len(sys.argv) > 4 else []):
    if not p.startswith("/"): fail("a line of the booted path-only list is out of form")
    elif p in bf: fail(f"a file listed both with a digest and by its path alone: {p}")
    else: bf[p] = None
def report(what, kind, path, expected, extra=""):
    print(f"{kind} {what}: {path}{extra}" + (" (expected)" if expected else ""))
    if not expected: fail(f"the first boot {kind} a MANIFEST {what} outside the expected list: {path}")
for p in sorted(sf.keys() | bf.keys()):
    if p not in bf: report("file", "removed", p, False)
    elif p not in sf: report("file", "added", p, any(fnmatch.fnmatchcase(p, g) for g in adds), " (by its path alone)" if bf[p] is None else "")
    elif sf[p] != bf[p]: report("file", "changed", p, p in changes)
for p in sorted(sl.keys() | bl.keys()):
    if p not in bl: report("link", "removed", p, False, f" (sealed -> {sl[p]})")
    elif p not in sl: report("link", "added", p, False, f" (booted -> {bl[p]})")
    elif sl[p] != bl[p]: report("link", "changed", p, False, f" (sealed -> {sl[p]}, booted -> {bl[p]})")
print(f"MANIFEST against the booted guest: {len(sf)} files and {len(sl)} links sealed, "
      f"{len(bf)} files and {len(bl)} links booted, {fails} finding(s)")
PY
}

# ---- The proof's ready row and second boot (weaver-appliance-build-Spec section 2, step 13;
# the operator's rulings of 2026-11-19). Each judge prints what it read and a PROOF-FAIL line
# for each finding, and returns 0, as the first boot's other reads do; a judge that cannot
# run is a PROOF-FAIL line too, never an exit that ends the proof under errexit.

# ready_console_judge LOG WORD: the request unit's line on the console LOG (QEMU's logfile):
# exactly one weaver-appliance line, before the first login prompt, and it reads
# "weaver-appliance: ready (gate-con WORD, admin-con WORD)": made at the first boot, found
# at the second.
ready_console_judge() {
    python3 -I -B - "$1" "$2" <<'PY' || echo "PROOF-FAIL: the console's ready line could not be judged (exit $?)"
import re, sys
try:
    t = open(sys.argv[1], "rb").read().decode("utf-8", "replace")
except OSError as e:
    print(f"PROOF-FAIL: the console log could not be read: {e}"); sys.exit(0)
# CSI sequences (colours, cursor moves) and OSC sequences (systemd's 3008 context marks)
# are not text; neither is a carriage return.
t = re.sub(r"\x1b\][^\x07\x1b]*(\x07|\x1b\\)", "", t)
lines = re.sub(r"\x1b\[[0-9;?]*[A-Za-z]", "", t.replace("\r", "")).split("\n")
want = "weaver-appliance: ready (gate-con %s, admin-con %s)" % (sys.argv[2], sys.argv[2])
found = [(i, m.group(0)) for i, l in enumerate(lines)
         for m in [re.search(r"weaver-appliance: (ready \([^)]*\)|refused: .*)$", l)] if m]
prompt = next((i for i, l in enumerate(lines) if re.match(r"^[^\s]+ login:", l)), None)
for _, l in found:
    print("on the console: " + l)
if not found:
    print("PROOF-FAIL: no weaver-appliance line on the console")
else:
    i, l = found[0]
    if l != want:
        print(f"PROOF-FAIL: the console's weaver-appliance line is not '{want}'")
    if len(found) > 1:
        print(f"PROOF-FAIL: {len(found)} weaver-appliance lines on the console, not one")
    if prompt is None:
        print("PROOF-FAIL: no login prompt on the console, so the line's place is not known")
    elif i > prompt:
        print("PROOF-FAIL: the weaver-appliance line comes after the login prompt")
    else:
        print("the line comes before the login prompt")
PY
}

# ready_requests_judge DIR: DIR/gate.req and DIR/admin.req, as read off the guest: each one
# line of ASCII ending in its newline, within WeaverWeb's first frame bound (16 KiB, the
# newline included), a JSON object of exactly the members event, "request", and request, a
# PEM certificate request, which the host's openssl reads and checks against its own
# signature. The two are not the same request.
ready_requests_judge() {
    python3 -I -B - "$1" <<'PY' || echo "PROOF-FAIL: the requests could not be judged (exit $?)"
import json, os, subprocess, sys
d, pems = sys.argv[1], {}
BEGIN, END = "-----BEGIN CERTIFICATE REQUEST-----\n", "-----END CERTIFICATE REQUEST-----\n"
for plane in ("gate", "admin"):
    p = os.path.join(d, plane + ".req")
    try:
        raw = open(p, "rb").read()
    except OSError as e:
        print(f"PROOF-FAIL: the {plane} request could not be read off the guest: {e}"); continue
    if raw.count(b"\n") != 1 or not raw.endswith(b"\n"):
        print(f"PROOF-FAIL: the {plane} request is not one line ending in its newline"); continue
    if len(raw) > 16384:
        print(f"PROOF-FAIL: the {plane} request is {len(raw)} bytes, over the 16384 pair reads"); continue
    try:
        obj = json.loads(raw.decode("ascii"))
    except (UnicodeDecodeError, ValueError) as e:
        print(f"PROOF-FAIL: the {plane} request is not a line of ASCII JSON: {e}"); continue
    if not isinstance(obj, dict) or set(obj) != {"event", "request"} or obj["event"] != "request" \
            or not isinstance(obj["request"], str) or not obj["request"].startswith(BEGIN) \
            or not obj["request"].endswith(END):
        print(f"PROOF-FAIL: the {plane} request is not of the form {{\"event\":\"request\",\"request\":\"<PEM>\"}}"); continue
    try:
        r = subprocess.run(["openssl", "req", "-noout", "-verify", "-subject"], input=obj["request"].encode(),
                           capture_output=True, timeout=30)
    except (OSError, subprocess.TimeoutExpired) as e:
        print(f"PROOF-FAIL: openssl could not read the {plane} request: {e}"); continue
    said = " ".join((r.stdout + r.stderr).decode("utf-8", "replace").split())
    if r.returncode != 0:
        print(f"PROOF-FAIL: the {plane} request's PEM does not verify as a certificate request: {said}"); continue
    pems[plane] = obj["request"]
    print(f"{plane}: one line, {len(raw)} bytes, of the form pair reads; openssl: {said}")
if len(pems) == 2 and pems["gate"] == pems["admin"]:
    print("PROOF-FAIL: the two connectors' requests are the same request")
PY
}

# ready_survey_judge FILE STATUS: the survey's output FILE and its exit STATUS, run as nobody
# with --stage pairing: status 0, and one object the reader of weaver-survey-format section 3
# (guest/ready/test/survey_reader.py) accepts.
ready_survey_judge() {
    python3 -I -B - "$1" "$2" "$LIB_DIR/guest/ready/test/survey_reader.py" <<'PY' || echo "PROOF-FAIL: the survey's output could not be judged (exit $?)"
import importlib.util, sys
f, rc, rp = sys.argv[1:4]
spec = importlib.util.spec_from_file_location("survey_reader", rp)
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
try:
    raw = open(f, "rb").read()
except OSError as e:
    print(f"PROOF-FAIL: the survey's output could not be read: {e}"); sys.exit(0)
if rc != "0":
    print(f"PROOF-FAIL: the survey exited {rc} as nobody, not 0")
try:
    members = m.reader(raw)
    print(f"the survey: exit {rc}, {len(raw)} bytes, one object the reader accepts ({len(members)} members)")
except m.Refuse as e:
    print(f"PROOF-FAIL: the reader refuses the survey's output: {e}")
PY
}

# ready_attrs_compare FIRST SECOND: the configs' attributes and the requests' digests, as the
# first boot read them (FIRST) and the second (SECOND), one "attr PATH ..." or "sum PATH
# DIGEST" line each: the same four lines, so nothing was remade.
ready_attrs_compare() {
    python3 -I -B - "$1" "$2" <<'PY' || echo "PROOF-FAIL: the two boots' attributes could not be compared (exit $?)"
import sys
def read(p):
    try:
        return {l.split(" ", 2)[0] + " " + l.split(" ", 2)[1]: l for l in open(p).read().split("\n") if l.count(" ") >= 2}
    except OSError:
        return {}
a, b = read(sys.argv[1]), read(sys.argv[2])
if len(a) != 4:
    print(f"PROOF-FAIL: the first boot read {len(a)} of the four attribute and digest lines")
for k in sorted(a.keys() | b.keys()):
    what = ("the config's attributes" if k.startswith("attr ") else "the request's digest") + " at " + k.split(" ", 1)[1]
    if k not in b: print(f"PROOF-FAIL: the second boot did not read {what}")
    elif k not in a: print(f"PROOF-FAIL: the second boot read {what}, which the first did not")
    elif a[k] != b[k]: print(f"PROOF-FAIL: remade: {what} changed between the boots")
    else: print(f"unchanged: {what}")
PY
}

# ready_unit_read: guest text that prints the request unit's run as systemd holds it, and a
# PROOF-FAIL line where the run was not a success.
READY_UNIT_READ='
p=$(systemctl show -p Result -p ExecMainStatus -p ExecMainExitTimestampMonotonic "$U" 2>&1)
echo "the request unit: $(tr "\n" " " <<<"$p")"
{ grep -qx Result=success <<<"$p" && grep -qx ExecMainStatus=0 <<<"$p"; } || echo "PROOF-FAIL: the request unit'"'"'s run was not a success"'
# READY_ATTRS_READ: guest text that prints each config's attributes, read without opening it
# (inode, size, owner, mode and modification time), and each request's sha256.
READY_ATTRS_READ='
for c in gate:gate-con admin:admin-con; do
    f=/etc/weaver-web/pairing/${c%%:*}/${c#*:}.toml
    echo "attr $f $(stat -c "%i %s %U:%G %a %Y" -- "$f" 2>&1)"
done
for r in /etc/weaver-web/pairing/exchange/gate.req /etc/weaver-web/pairing/exchange/admin.req; do
    s=$(sha256sum -- "$r" 2>&1); echo "sum $r ${s%% *}"
done'

# ready_guest_read: the first boot's guest text for the ready row, as root: the unit's run
# and its exit before the first login, by the journal's monotonic clock (logind's first
# "New session"); the three accounts in the box's shape (template-steps' own shape_check);
# the six folders as step 10 lays them, on the root filesystem; each config a regular file
# of its own account, mode 0600, and each request root's, mode 0644, by their attributes
# alone; nothing else and nothing mounted under /etc/weaver-web; then READY_ATTRS_READ.
ready_guest_read() {
    printf 'U=%q\nREADY_ACCOUNTS=%q\nREADY_DIRS=%q\n' "$READY_UNIT" "$READY_ACCOUNTS" "$READY_DIRS"
    sed -n '/^shape_check() {/,/^}/p' "$LIB_DIR/guest/template-steps"
    printf '%s\n' "$READY_UNIT_READ"
    cat <<'EOF'
ue=$(sed -n 's/^ExecMainExitTimestampMonotonic=//p' <<<"$p")
le=$(journalctl -b -o json --no-pager _SYSTEMD_UNIT=systemd-logind.service 2>/dev/null | python3 -c '
import json, sys
ts = []
for l in sys.stdin:
    try:
        j = json.loads(l)
    except ValueError:
        continue
    if str(j.get("MESSAGE", "")).startswith("New session"):
        ts.append(int(j["__MONOTONIC_TIMESTAMP"]))
print(min(ts) if ts else "")')
echo "the unit exited at ${ue:-unread} us; the first login session began at ${le:-unread} us (this boot's monotonic clock, by the journal)"
{ [[ $ue =~ ^[1-9][0-9]*$ ]] && [[ $le =~ ^[1-9][0-9]*$ ]] && [ "$ue" -lt "$le" ]; } || echo "PROOF-FAIL: the request unit's exit is not before the first login by the journal"
for a in $READY_ACCOUNTS; do
    o=$(shape_check "$a"); if [ -z "$o" ]; then echo "$a: in the box's shape"; else echo "PROOF-FAIL: $o"; fi
done
rd=$(stat -c %d /); want=""
for d in $READY_DIRS; do
    IFS=: read -r path owner mode <<<"$d"; want+="$path"$'\n'
    s=$(stat -c "%F %U:%G %a %d" -- "$path" 2>/dev/null); echo "$path: ${s:-absent}"
    [ "$s" = "directory $owner:$owner $mode $rd" ] || echo "PROOF-FAIL: $path is '${s:-absent}', not a directory $owner:$owner $mode on the root filesystem"
done
for c in gate-con:weaver-gatecon:gate admin-con:weaver-admincon:admin; do
    IFS=: read -r con acct plane <<<"$c"
    f=/etc/weaver-web/pairing/$plane/$con.toml; r=/etc/weaver-web/pairing/exchange/$plane.req; want+="$f"$'\n'"$r"$'\n'
    s=$(stat -c "%F %U:%G %a" -- "$f" 2>/dev/null); echo "$f: ${s:-absent}"
    [ "$s" = "regular file $acct:$acct 600" ] || echo "PROOF-FAIL: $f is '${s:-absent}', not a regular file $acct:$acct 600"
    s=$(stat -c "%F %U:%G %a" -- "$r" 2>/dev/null); echo "$r: ${s:-absent}"
    [ "$s" = "regular file root:root 644" ] || echo "PROOF-FAIL: $r is '${s:-absent}', not a regular file root:root 644"
done
got=$(find /etc/weaver-web -xdev 2>&1 | LC_ALL=C sort); want=$(printf "%s" "$want" | LC_ALL=C sort)
[ "$got" = "$want" ] || echo "PROOF-FAIL: /etc/weaver-web holds other than the six folders, the two configs and the two requests: $(LC_ALL=C comm -3 <(printf "%s\n" "$want") <(printf "%s\n" "$got") | tr -d "\t" | tr "\n" " ")"
m=$(findmnt -rn -o TARGET 2>&1 | awk '$0 == "/etc/weaver-web" || index($0, "/etc/weaver-web/") == 1')
[ -z "$m" ] || echo "PROOF-FAIL: mounted at or under /etc/weaver-web: $(tr "\n" " " <<<"$m")"
EOF
    printf '%s\n' "$READY_ATTRS_READ"
}

# pool_drop DIR: the storage pool virt-install made for DIR, where one stands, destroyed and
# undefined (virt-install makes one for each directory a --disk path names).
pool_drop() {
    local pool path want=${1%/}
    for pool in $(virsh_ pool-list --all --name 2>/dev/null || true); do
        # A pool that goes between the listing and its read is skipped, not fatal.
        path=$(virsh_ pool-dumpxml "$pool" 2>/dev/null | sed -n 's:.*<path>\(.*\)</path>.*:\1:p') || continue
        [ "${path%/}" = "$want" ] || continue
        virsh_ pool-destroy "$pool" >/dev/null 2>&1 || true
        virsh_ pool-undefine "$pool" >/dev/null && echo "pool $pool ($path) undefined"
    done
}

# dom_known NAME: 0 where domain NAME is defined, 1 where a domain list that was read does
# not name it, 2 where the list could not be read or searched. A query that fails
# (libvirt out of reach) is never taken for a domain that is gone.
dom_known() {
    local names rc=0
    names=$(virsh_ list --all --name 2>/dev/null) || return 2
    grep -qxF -- "$1" <<<"$names" || rc=$?
    case $rc in 0) return 0 ;; 1) return 1 ;; *) return 2 ;; esac
}

# dom_on_disk NAME DISK: 0 where one of domain NAME's disks (device "disk") has DISK, an
# absolute path, as its whole source file; 1 where none has; 2 where the domain's XML
# cannot be read or parsed. The source is read from the domain's XML and compared whole,
# never as the end of a listing's line, so a path that holds a space cannot pass for DISK.
# The parser names its verdict on stdout, "match" or "none", and only a parser that exits
# 0 with one of those two words is believed: any other exit (127 for no python3, 1 for an
# exception it did not catch, a signal) is 2, never a mismatch. dom_drop and seal_verdict
# act on a domain only through it, so a domain of the same name on another disk is never
# stopped or undefined.
dom_on_disk() {
    local x v
    x=$(virsh_ dumpxml "$1" 2>/dev/null) && [ -n "$x" ] || return 2
    v=$(printf '%s' "$x" | python3 -I -c '
import sys, xml.etree.ElementTree as E
try:
    r = E.fromstring(sys.stdin.read())
    found = False
    for d in r.iter("disk"):
        s = d.find("source")
        if d.get("device") == "disk" and s is not None and s.get("file") == sys.argv[1]:
            found = True
except Exception:
    sys.exit(2)
print("match" if found else "none")' "$2" 2>/dev/null) || return 2
    case $v in match) return 0 ;; none) return 1 ;; *) return 2 ;; esac
}

# dom_drop NAME DISK: domain NAME, where one is defined, stopped and undefined with its
# firmware store, only when one of its disks is DISK (an absolute path, as libvirt reports
# it). A domain of that name on any other disk is refused and left alone (exit 1). A
# domain list or definition that cannot be read, a domain still running after the stop,
# or an undefine that fails, is exit 2, its cause on stderr, and the caller keeps the
# domain's directory: an undefined domain that still runs goes on writing its disk.
dom_drop() {
    [[ "$2" == /* ]] || { echo "REFUSED: dom_drop takes an absolute disk path, not '$2'" >&2; return 1; }
    # Each status is matched whole: 0 and 1 are named, and any other is refused as 2.
    local rc=0 st
    dom_known "$1" || rc=$?
    case $rc in
        0) ;;
        1) return 0 ;;
        *) echo "REFUSED: the domain list could not be read, so $1 is not taken for gone" >&2; return 2 ;;
    esac
    rc=0; dom_on_disk "$1" "$2" || rc=$?
    case $rc in
        0) ;;
        1) echo "REFUSED: $1 is defined, but on no disk $2; left alone" >&2; return 1 ;;
        *) echo "REFUSED: $1's definition could not be read or parsed; left alone" >&2; return 2 ;;
    esac
    # destroy fails on a domain that is already shut off, so its exit is not the test: the
    # domain's state after it is.
    virsh_ destroy "$1" >/dev/null 2>&1 || true
    rc=0; dom_known "$1" || rc=$?
    case $rc in
        0) ;;
        1) return 0 ;;
        *) echo "REFUSED: the domain list could not be read after destroy, so $1 is not taken for gone" >&2; return 2 ;;
    esac
    st=$(virsh_ domstate "$1" 2>/dev/null) || st=unknown
    [ "$st" = "shut off" ] || { echo "REFUSED: $1 is not shut off after destroy (${st:-unknown}); not undefined" >&2; return 2; }
    virsh_ undefine "$1" --nvram >/dev/null || { echo "REFUSED: $1 could not be undefined" >&2; return 2; }
}

# step_ok LOG NAME: 0 when LOG/NAME.txt, as run_step wrote it, ends with exit 0.
step_ok() {
    local l
    [ -f "$1/$2.txt" ] && l=$(tail -n 1 "$1/$2.txt") && grep -qE "^# step $2, exit 0, [0-9]+s\$" <<<"$l"
}

# origin_ok URL: 0 when URL starts with one of DEB_ORIGINS and the rest is a plain path,
# with no "..", query, fragment or user part. The guest names each URL a round fetches, so
# the host fetches only under the origins it was given. fetch_round follows no redirect:
# a redirect would take the host to a URL these origins never allowed.
origin_ok() {
    local o rest
    for o in $DEB_ORIGINS; do
        [[ "$o" =~ ^https?://[A-Za-z0-9.-]+(:[0-9]+)?/([A-Za-z0-9._+~%-]+/)*$ ]] \
            || die "DEB_ORIGINS holds '$o', not an http(s) URL prefix ending in /"
        [ "${1#"$o"}" != "$1" ] || continue
        rest=${1#"$o"}
        [[ "$rest" =~ ^[A-Za-z0-9._+~%/-]+$ ]] && [[ "/$rest/" != */../* ]]
        return
    done
    return 1
}

# hash_ok FILE SUM WANT SIZE: 0 when FILE's digest by SUM is WANT and its size is SIZE;
# sets GOT_HASH and GOT_SIZE. FILE is a file no other round names, so both readings are
# of the same bytes.
hash_ok() {
    GOT_HASH=$($2 "$1" | cut -d' ' -f1)
    GOT_SIZE=$(stat -c %s "$1")
    [ "$GOT_HASH" = "$3" ] && [ "$GOT_SIZE" = "$4" ]
}

# fetch_round PORT LOG CACHE ROUND: the host's half of one driver round (Spec section 6).
# The guest's driver-uris-ROUND names each file with its SHA256 from the guest's signed
# index. A URL outside DEB_ORIGINS is refused before anything is fetched. Each file is
# checked by SHA256 (or SHA512) and size in the round's own stage, so the bytes put into
# the guest are the bytes checked. A cached copy is used when its copy in the stage
# matches; otherwise the file is fetched to a temporary name of this fetch's own in CACHE,
# checked, copied into the stage, and renamed into CACHE. Two rounds sharing CACHE never
# share a temporary name, and nothing another round names is removed; a rename puts only
# a checked file in CACHE. The round's temporary file and stage go on every exit, a
# refusal or a signal. The round's files go into a new ~/debs by put, and
# driver-install-ROUND installs them. Any refusal stops the caller.
# It sets, then clears, the shell's EXIT, INT, TERM and HUP traps: a caller's own are lost.
fetch_round() {
    local port=$1 log=$2 cache=$3 round=$4 n=0 url file size hash want sum part="" list stage="" staged code
    case $round in pin|open) ;; *) die "a round is pin or open, not '$round'" ;; esac
    mkdir -p "$cache"
    run_step "$port" "$log" "driver-uris-$round" "$(guest_env DRIVER_PIN DRIVER_OPEN)gpu-steps driver-uris $round"
    step_ok "$log" "driver-uris-$round" || die "driver-uris $round did not pass; see $log/driver-uris-$round.txt"
    stage=$(mktemp -d) || die "no stage directory for the $round round"
    trap '[ -z "${part:-}" ] || rm -f -- "$part"; [ -z "${stage:-}" ] || rm -rf -- "$stage"' EXIT
    trap 'exit 130' INT
    trap 'exit 143' TERM
    trap 'exit 129' HUP
    list=$log/debs-$round.txt
    : >"$list"
    while read -r url file size hash; do
        url=${url//\'/}
        [[ "$url" =~ ^https?:// ]] || continue
        origin_ok "$url" || die "$url is not under DEB_ORIGINS ($DEB_ORIGINS); nothing is fetched from it"
        case ${hash%%:*} in
            SHA256) sum=sha256sum ;;
            SHA512) sum=sha512sum ;;
            *) die "$file carries a hash this script does not check: ${hash%%:*}" ;;
        esac
        want=${hash#*:}
        [[ "$file" =~ ^[A-Za-z0-9][A-Za-z0-9._+~%-]*\.deb$ ]] || die "apt named a file this script will not write: $file"
        [[ "$size" =~ ^[0-9]+$ ]] || die "$file has no size: $size"
        staged=$stage/$file
        if [ -f "$cache/$file" ] && cp -- "$cache/$file" "$staged" && hash_ok "$staged" "$sum" "$want" "$size"; then
            echo "$file: the cached copy matches the index's hash and size"
        else
            rm -f -- "$staged"
            [ ! -e "$cache/$file" ] || echo "$file: the cached copy does not match; fetching it again"
            part=$(mktemp "$cache/.$file.XXXXXX.part") || die "no temporary file in $cache"
            # No redirect is followed (no -L); anything but HTTP 200 stops the round.
            code=$(curl -fsS --proto '=https,http' -o "$part" -w '%{http_code}' "$url") || die "$url did not fetch"
            [ "$code" = 200 ] || die "$url answered HTTP $code; redirects are not followed"
            hash_ok "$part" "$sum" "$want" "$size" \
                || die "$file hashes $GOT_HASH and is $GOT_SIZE bytes; the index says $want and $size"
            cp -- "$part" "$staged" || die "$file did not copy into the round's stage"
            mv -f -- "$part" "$cache/$file" || die "$file did not move into $cache"
            part=""
            echo "$file: fetched; it matches the index's hash and size"
        fi
        echo "$file $size ${hash%%:*} $want" >>"$list"
        n=$((n + 1))
    done < <(grep -E "^'https?://" "$log/driver-uris-$round.txt")
    [ "$n" -gt 0 ] || die "apt named no .deb for the $round round"
    echo "$round round: $n .debs checked against the index's hashes ($list)"
    put "$port" "$stage" debs
    rm -rf -- "${stage:?}"
    stage=""
    trap - EXIT INT TERM HUP
    run_step "$port" "$log" "driver-install-$round" "$(guest_env DRIVER_PIN DRIVER_OPEN)gpu-steps driver-install $round"
    step_ok "$log" "driver-install-$round" || die "driver-install $round did not pass; see $log/driver-install-$round.txt"
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
