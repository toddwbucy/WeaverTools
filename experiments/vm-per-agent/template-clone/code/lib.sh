# Sourced by build-template.sh and clone.sh: the measurement's pins, and the host-side
# helpers that define a guest, reach it over SSH under its pinned host key, judge its
# boots and log each step. template-clone-Spec.md states what they do.
#
# VM_ROOT names the directory that holds images/, template/, appliance/, clones/, logs/ and
# keys/.
# It has no default here: a default would be a box path in the repository. Each script
# sets KNOWN_HOSTS to its guest's own known-hosts file before it reaches the guest.

: "${VM_ROOT:?set VM_ROOT to the directory holding images/ template/ clones/ logs/ keys/}"
# Absolute and without its trailing slashes, so that the paths built on it compare equal to
# the ones libvirt reports (a storage pool's path, a domain's disk).
case $VM_ROOT in /*) ;; *) VM_ROOT=$PWD/$VM_ROOT ;; esac
while [[ "$VM_ROOT" == */ && "$VM_ROOT" != / ]]; do VM_ROOT=${VM_ROOT%/}; done

URI=qemu:///system
# The guest operating system, a named input (OS): its cloud image by digest, checked
# against SHA256SUMS signed by the Ubuntu cloud image key. 26.04 is the cloud image as
# released on 2026-10-20. 24.04 is named and refused until its image is pinned
# (template-clone-Spec.md, section 10). clone.sh names no OS and gets 26.04.
OS=${OS:-26.04}
IMAGE_KEY=D2EB44626FDDC30B513D5BB71A5D6C4C7DB87C81
case $OS in
    26.04)
        IMAGE_NAME=ubuntu-26.04-server-cloudimg-amd64.img
        IMAGE_URL=https://cloud-images.ubuntu.com/releases/resolute/release
        IMAGE_SHA256=5645d0318ad21f643ebb1ac4567514620e9893380b07b0201fda163baa5b7eb5
        OS_VARIANT=ubuntu26.04 ;;
    24.04)
        printf '\nREFUSED: no Ubuntu 24.04 cloud image is pinned yet (template-clone-Spec.md, section 10)\n' >&2
        exit 1 ;;
    *)
        printf '\nREFUSED: OS is 26.04 (or 24.04 once pinned), not %s\n' "$OS" >&2
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
# root and no loop device.
# The root partition is the one GPT names "Linux root (x86-64)". image_cat and image_ls
# read through debugfs, which follows no symbolic link; image_close stops the export.
image_open() {
    local disk=$1 dir=$2 part start size
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
    kill "$(cat "$dir/qsd-whole.pid" 2>/dev/null)" 2>/dev/null || true; sleep 1
    [ -n "$part" ] || die "$disk has no single Linux root partition"
    read -r start size <<<"$part"
    qemu-storage-daemon \
        --blockdev "driver=file,node-name=f,filename=$disk,read-only=on,locking=off" \
        --blockdev driver=qcow2,node-name=q,file=f,read-only=on \
        --blockdev "driver=raw,node-name=p,file=q,offset=$((start * 512)),size=$((size * 512)),read-only=on" \
        --export "type=fuse,id=r,node-name=p,mountpoint=$dir/root.raw,writable=off,allow-other=off" \
        --pidfile "$dir/qsd-root.pid" --daemonize || die "no export of $disk's root partition"
}
image_cat() { debugfs -c -R "cat $2" "$1/root.raw" 2>/dev/null; }
# image_ls DIR PATH fails where PATH is absent. debugfs exits 0 on a missing path and prints
# nothing to stdout, and a directory always lists . and .., so no output is the absence.
image_ls() {
    local o
    o=$(debugfs -c -R "ls -p $2" "$1/root.raw" 2>/dev/null)
    [ -n "$o" ] || return 1
    printf '%s\n' "$o"
}
# image_has DIR PATH: whether PATH is in the image (debugfs stat prints nothing for a path
# that is not).
image_has() { [ -n "$(debugfs -c -R "stat \"$2\"" "$1/root.raw" 2>/dev/null | head -1)" ]; }
# image_files DIR PATH: "SIZE PATH" for each regular file under PATH, one per line, depth
# bounded at 12. debugfs ls -p prints /inode/mode/uid/gid/name/size/ per entry, the mode in
# six octal digits (040 a directory, 100 a regular file). Nothing where PATH is absent.
image_files() {
    local dir=$1 path=${2%/} depth=${3:-0} ino mode uid gid name size rest
    [ "$depth" -lt 12 ] || return 0
    debugfs -c -R "ls -p \"$path\"" "$dir/root.raw" 2>/dev/null |
        while IFS=/ read -r _ ino mode uid gid name size rest; do
            case $name in '' | . | ..) continue ;; esac
            case $mode in
                040*) image_files "$dir" "$path/$name" $((depth + 1)) ;;
                100*) printf '%s %s\n' "${size:-0}" "$path/$name" ;;
            esac
        done
}
# image_late_writes DIR: what a sealed image holds that its seal's final probe refuses, so
# written after that probe (at the power-off): systemd's random seed, its credential secret,
# and any non-empty file under /var/log. Each is a PROOF-FAIL line. The login records under
# /var/lib (wtmpdb, lastlog) are listed for the measure and not judged.
image_late_writes() {
    local dir=$1 f p
    for f in /var/lib/systemd/random-seed /var/lib/systemd/credential.secret; do
        if image_has "$dir" "$f"; then
            echo "PROOF-FAIL: $f is in the sealed image, written after the final probe"
        else
            echo "absent: $f"
        fi
    done
    image_files "$dir" /var/log | awk '$1 > 0 { s = $1; sub(/^[0-9]+ /, "")
        print "PROOF-FAIL: a log written after the final probe: " $0 " (" s " bytes)"; n++ }
        END { if (!n) print "no non-empty file under /var/log" }'
    for p in /var/lib/wtmpdb /var/lib/lastlog; do
        echo "== $p, listed and not judged"
        image_files "$dir" "$p" | sed 's/^/  /'
    done
}
# image_close stops both exports where they still run, unmounts one whose daemon is gone,
# and never fails its caller (it runs from cleanup traps).
image_close() {
    local f
    for f in qsd-whole qsd-root; do
        [ ! -f "$1/$f.pid" ] || kill "$(cat "$1/$f.pid" 2>/dev/null)" 2>/dev/null || true
    done
    sleep 1
    for f in whole.raw root.raw; do
        ! findmnt -n --mountpoint "$1/$f" >/dev/null 2>&1 || fusermount3 -u "$1/$f" 2>/dev/null || true
    done
    rm -f -- "$1/whole.raw" "$1/root.raw" "$1/qsd-whole.pid" "$1/qsd-root.pid" 2>/dev/null || true
}

# qemu_file FILE: an empty FILE that its owner and QEMU's account (QEMU_USER) alone may
# write, for a file QEMU opens from its own command line (a serial console, a capture).
# A file libvirt opens for a device becomes its own, unreadable here without root; a mode
# open to every local account would let any of them write the console that trust on first
# use is read from. The grant is a POSIX ACL, which the filesystem must support.
qemu_file() {
    : >"$1" && chmod 0600 "$1" && setfacl -m "u:${QEMU_USER:-libvirt-qemu}:rw" "$1" \
        || die "cannot make $1 writable by its owner and ${QEMU_USER:-libvirt-qemu} alone"
}

# The MANIFEST's links section (guest/template-steps, links_list and LINKS_MARK; the two
# copies change together). links_format takes find's NUL-separated path and target pairs
# on stdin and writes the section's lines; manifest_links prints a MANIFEST's section, empty
# for a MANIFEST of the earlier form (regular files only).
LINKS_MARK='# links: path TAB target, C-locale order'
MANIFEST_SCOPE='/usr /etc /opt /boot /boot/efi'
links_format() {
    local p t
    while IFS= read -r -d '' p && IFS= read -r -d '' t; do
        p=${p//\\/\\\\}; p=${p//$'\t'/\\t}; p=${p//$'\n'/\\n}
        t=${t//\\/\\\\}; t=${t//$'\t'/\\t}; t=${t//$'\n'/\\n}
        printf '%s\t%s\n' "$p" "$t"
    done | LC_ALL=C sort -u
}
manifest_links() { M=$LINKS_MARK awk '$0 == ENVIRON["M"] { l = 1; next } l' "$1"; }

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

# dom_drop NAME DISK: domain NAME, where one is defined, destroyed and undefined with its
# firmware store, only when one of its disks is DISK (an absolute path, as libvirt reports
# it). A domain of that name on any other disk is refused and left alone (exit 1).
dom_drop() {
    [[ "$2" == /* ]] || { echo "REFUSED: dom_drop takes an absolute disk path, not '$2'" >&2; return 1; }
    virsh_ dominfo "$1" >/dev/null 2>&1 || return 0
    if ! virsh_ domblklist "$1" --details 2>/dev/null | D=$2 awk '
            $2 == "disk" && substr($0, length($0) - length(ENVIRON["D"])) == " " ENVIRON["D"] { f = 1 }
            END { exit !f }'; then
        echo "REFUSED: $1 is defined, but on no disk $2; left alone" >&2
        return 1
    fi
    virsh_ destroy "$1" >/dev/null 2>&1 || true
    virsh_ undefine "$1" --nvram >/dev/null
}

# step_ok LOG NAME: 0 when LOG/NAME.txt, as run_step wrote it, ends with exit 0.
step_ok() {
    [ -f "$1/$2.txt" ] && tail -n 1 "$1/$2.txt" | grep -qE "^# step $2, exit 0, [0-9]+s\$"
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
