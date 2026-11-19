#!/usr/bin/env bash
# proof_first_boot's grading, with no guest: virt-install, xorriso, ssh-keyscan, ssh, the
# guest's answers (gssh) and the domain's removal stood in. The console is a stand-in
# server on the unix socket the proof names to QEMU, playing Arch's getty, login's forced
# change and a shell, or one of the ways they can go wrong. Real: the network credential
# (proof_network), the seed the image must ignore (the operator's ruling of 2026-11-16),
# the prompt check, proof_keyscan's merge
# (the Planner's review of 2026-11-15, F11), the SSH refusals and their judge, the first
# login through console-drive.py and console_judge (the operator's ruling of 2026-11-16:
# root, set at first login), the host-key match, and manifest_compare with lib.sh's lists.
# Each run has its own VM_ROOT. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need ssh-keygen python3
# Each stand-in proof's console socket lies under this run's directory, and proof_console
# refuses a socket path longer than a unix socket allows (107 bytes), rightly. Refuse here,
# once and by name, where TMPDIR is too long for that, not in each stand-in proof.
s=$T/vmroot.XXXXXX/first/con/console.sock
if [ "${#s}" -gt 107 ]; then
    no "TMPDIR is too long for the proof's console socket: $s is ${#s} bytes, over the 107 a unix socket allows; set TMPDIR shorter"
    finish; exit
fi
BT=$CODE/build-template.sh
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$BT" proof_first_boot proof_network proof_keyscan proof_console proof_refusal)"
eval "$(fn "$BT" proof_power_off | sed '1s/^proof_power_off()/real_proof_power_off()/')"
proof_power_off() { echo "== the power button (stand-in) $1 ${2##*/}"; }
# The ready row and the second boot have their own test (t-ready-row.sh). Here they stand
# in, and the second boot says it started (or, with T/no-second, that it could not), so its
# power button is pressed after it or not at all.
proof_ready() { echo "== the ready layer (stand-in) ${1##*/} $3"; }
proof_second_boot() { echo "== the second boot (stand-in) $3"; [ ! -e "$T/no-second" ]; }
NAME=mock-appliance PORT=2230 PROOF_OWN="" PROOF_FIRST_BOOT=4 PROOF_NORELABEL=stand-in
for t in rsa ecdsa ed25519; do ssh-keygen -q -t $t -N '' -C x -f "$T/hk_$t" </dev/null; done
ssh-keygen -q -t ed25519 -N '' -C x -f "$T/other_ed25519" </dev/null
ssh-keygen -q -t ed25519 -N '' -C x -f "$T/proofkey" </dev/null; KEY=$T/proofkey
proof_overlay() { : >"$1"; }
qemu_file() { :; }
qemu_dir() { mkdir -- "$1"; }
proof_sleep() { :; }
proof_inbound() { echo "== the base inbound policy (stand-in)"; }
dom_drop() { return 0; }
pool_drop() { return 0; }
# virt-install: records its arguments, writes the boot's console text where QEMU's logfile
# would be, and starts the stand-in console on the socket the -chardev names.
virt-install() { local a sock log
    printf '%s\n' "$@" >"$T/vi-args"
    a=$(grep -o -- '-chardev socket,[^ ]*' "$T/vi-args")
    sock=$(sed -n 's/.*,path=\([^,]*\),.*/\1/p' <<<"$a"); log=$(sed -n 's/.*,logfile=\([^,]*\).*/\1/p' <<<"$a")
    sed 's/$/\r/' <"$T/console" >"$log"
    python3 -I "$T/con-server.py" "$sock" "$log" "$(cat "$T/con-host")" "$(cat "$T/con-mode")" "$T/con-got" &
    echo $! >"$T/con-pid"; }
# The stand-in console: getty's prompt on an empty line; for root, login's forced change
# (pass), or a password asked (asks-password), or a shell at once (no-change), or a change
# the second entry does not match (mismatch); then a shell that echoes each line, answers
# the read line (on /dev/tty1 in wrong-tty), and logs out on exit. It appends what it sends
# to the logfile, as QEMU does, and records the passwords it was given in its last file. A
# way that goes wrong ends the connection, so the driver stops at once.
cat >"$T/con-server.py" <<'PY'
import os, socket, sys
path, logp, host, mode, gotp = sys.argv[1:6]
srv = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
srv.bind(path); srv.listen(1); srv.settimeout(60)
c, _ = srv.accept()
log = open(logp, "ab", buffering=0)
def out(t):
    b = t.encode(); c.sendall(b); log.write(b)
def line():
    b = b""
    while not b.endswith(b"\r"):
        d = c.recv(1)
        if not d:
            sys.exit(0)
        b += d
    return b[:-1].decode()
def prompt():
    out("[root@%s ~]# " % host)
while True:
    l = line()
    if l == "":
        out("\r\n\r\nArch Linux 7.3.4-arch1-2 (ttyS0)\r\n\r\n%s login: " % host)
    elif l == "root":
        out("root\r\n")
        break
if mode == "asks-password":
    out("Password: "); sys.exit(0)
if mode == "no-change":
    out("Last login: never\r\n"); prompt(); sys.exit(0)
out("You are required to change your password immediately (administrator enforced).\r\nNew password: ")
p1 = line(); out("\r\nRetype new password: ")
p2 = line()
open(gotp, "w").write(p1 + "\n" + p2 + "\n")
if mode == "mismatch":
    out("\r\nSorry, passwords do not match.\r\npasswd: Authentication token manipulation error\r\n"); sys.exit(0)
out("\r\n"); prompt()
keys = 0
while True:
    l = line()
    out(l + "\r\n")
    if l.startswith("install -d -m 0700 /root/.ssh && echo 'ssh-ed25519 "):
        keys = 1
    elif l.startswith("printf 'READ tty=%s"):
        out("READ tty=%s user=root root=P keys=%d\r\n" % ("/dev/tty1" if mode == "wrong-tty" else "/dev/ttyS0", keys))
    elif l == "exit":
        out("logout\r\n\r\nArch Linux 7.3.4-arch1-2 (ttyS0)\r\n\r\n%s login: " % host); break
    prompt()
PY
# ssh: one refused login, as OpenSSH's -v reports it, or with T/ssh-pw-offered a server that
# offers passwords too. A password try reads its helper, as ssh would, and records what the
# helper gave in T/askpass.log. setsid runs the function, not a program.
ssh() { local a k=0
    for a; do [ "$a" = -i ] && k=1; done
    if [ "$k" = 0 ]; then printf '%s\n' "$("$SSH_ASKPASS")" >>"$T/askpass.log"; fi
    if [ -e "$T/ssh-pw-offered" ]; then echo "debug1: Authentications that can continue: publickey,password"
    else echo "debug1: Authentications that can continue: publickey"; fi
    echo "debug1: No more authentication methods to try."
    echo "root@127.0.0.1: Permission denied (publickey)."
    return 255; }
setsid() { [ "$1" = -w ] && shift; "$@"; }
# xorriso: writes the image it is asked for and records its path in T/iso-path, or with
# T/no-iso fails. The proof removes its directory, the image with it, when it ends.
xorriso() { local o=""; [ -e "$T/no-iso" ] && return 1
    while [ $# -gt 0 ]; do [ "$1" = -o ] && { o=$2; shift; }; shift; done; : >"$o"; printf '%s\n' "$o" >"$T/iso-path"; }
# ssh-keyscan: the three keys, or with T/scan-other a foreign ed25519 key, or with
# T/scan-split the first two keys on the first scan and the third alone on the retry.
ssh-keyscan() { local p n=0
    [ -e "$T/scan-split" ] && { n=$(cat "$T/scan-count" 2>/dev/null || echo 0); echo $((n + 1)) >"$T/scan-count"; }
    for p in "$T"/hk_*.pub; do
        [ -e "$T/scan-other" ] && [ "${p##*/}" = hk_ed25519.pub ] && p=$T/other_ed25519.pub
        if [ -e "$T/scan-split" ]; then
            if [ "$n" = 0 ]; then [ "${p##*/}" = hk_ed25519.pub ] && continue; else [ "${p##*/}" = hk_ed25519.pub ] || continue; fi
        fi
        printf '[127.0.0.1]:2330 %s\n' "$(cut -d' ' -f1,2 "$p")"
    done; }
# The sealed MANIFEST: three files and one link. The booted listing changes machine-id and
# the shadow file (the first login's password), and adds the host keys' public halves, or
# (T/booted-extra) also changes /etc/passwd, which is not on the expected list.
D=0000000000000000000000000000000000000000000000000000000000000000
E=1111111111111111111111111111111111111111111111111111111111111111
mkdir -p "$T/out"
printf '%s  /etc/machine-id\n%s  /etc/passwd\n%s  /etc/shadow\n%s\n/etc/x\t/dev/null\n' "$D" "$D" "$D" "$LINKS_MARK" >"$T/out/MANIFEST"
gssh() { shift
    case $* in
        *"ssh_host_*_key.pub"*) [ -e "$T/no-login" ] && { echo "root@127.0.0.1: Permission denied (publickey)."; return 255; }
            for p in "$T"/hk_*.pub; do ssh-keygen -lf "$p"; done ;;
        *"cloud-init: not installed"*) printf '%s\n' "$*" >"$T/reads.sh"; echo "== cloud-init, with a seed attached"; echo "cloud-init: not installed"
            echo "== the seed's asks"; echo "not run: bootcmd"; echo "not run: runcmd"; echo "not made: the default account"
            echo "== the hostname"; echo "$IMAGE_HOSTNAME" ;;
        *"-xdev -type f -print") [ ! -e "$T/ready-files" ] || printf '%s\n' /etc/weaver-web/pairing/admin/admin-con.toml /etc/weaver-web/pairing/gate/gate-con.toml ;;
        *"-type f"*) printf '%s\n' "$*" >"$T/files-cmd"
            [ ! -e "$T/ready-files" ] || printf '%s  /etc/weaver-web/pairing/exchange/%s.req\n' "$E" admin "$E" gate
            printf '%s  /etc/machine-id\n' "$E"; printf '%s  %s\n' "$([ -e "$T/booted-extra" ] && echo "$E" || echo "$D")" /etc/passwd
            printf '%s  /etc/shadow\n%s  /etc/ssh/ssh_host_ed25519_key.pub\n' "$E" "$E" ;;
        *"-type l"*) printf '/etc/x\0/dev/null\0' ;;
        *) echo "unexpected: $*" ;;
    esac; }
run() { VM_ROOT=$(mktemp -d "$T/vmroot.XXXXXX"); rm -f -- "$T/out/first-boot.txt" "$T/scan-count" "$T/vi-args" "$T/out/first-boot-session.log" "$T/askpass.log" "$T/con-got"
    # build-template.sh's own shell options: under errexit an expected failure (a refused
    # ssh, 255) must be judged, not end the proof (the rehearsal of 2026-11-16).
    ( set -euo pipefail; proof_first_boot "$T/out" ) >"$T/run.out" 2>&1; local rc=$?
    kill "$(cat "$T/con-pid" 2>/dev/null)" 2>/dev/null; rm -f -- "$T/con-pid"; return "$rc"; }
echo pass >"$T/con-mode"; echo "$IMAGE_HOSTNAME" >"$T/con-host"
fb() { cat "$T/out/first-boot.txt"; }
match3='^the host keys: MATCH, the guest holds each key SSH presents, and SSH presents each the guest holds (3 keys)$'

printf 'Arch Linux 7.3.4-arch1-2 (ttyS0)\n\n%s login: ' "$IMAGE_HOSTNAME" >"$T/console"
run
check "a first boot that greets as the image, whose keys match, passes with no PROOF-FAIL" \
    test "$(fb | grep -c '^PROOF-FAIL')" = 0 -a "$(fb | grep -c "$match3")" = 1
check "and names no gap: the console login is proved, not stood in for" test "$(fb | grep -c '^GAP')" = 0
check "SSH refuses root three ways before the first login and two after, each offering publickey alone" \
    test "$(fb | grep -c '^refused: .* (the server offers publickey alone)$')" = 5 \
         -a "$(fb | sed -n '/^== SSH as root before the first login$/,/^== the first login, at the console$/p' | grep -c '^refused: ')" = 3 \
         -a "$(fb | sed -n '/^== SSH as root after the first login$/,$p' | grep -c '^refused: ')" = 2
check "the first login: greeted as the image, the change announced, root on ttyS0 with P and one key" \
    test "$(fb | grep -cx "the prompt before root: $IMAGE_HOSTNAME login:")" = 1 -a "$(fb | grep -cx 'the forced change: announced')" = 1 \
         -a "$(fb | grep -cx 'the session: tty /dev/ttyS0, user root, root.s password P, 1 authorized key(s)')" = 1
check "the console was given the password file's one line, twice" \
    test "$(sed -n 1p "$T/con-got")" = "$(sed -n 2p "$T/con-got")" -a "$(wc -l <"$T/con-got")" = 2 -a "$(sed -n 1p "$T/con-got" | wc -c)" = 25
pw=$(sed -n 1p "$T/con-got")
check "the SSH tries after the change offered that password, then a wrong one; before, an empty one and a wrong one" \
    test "$(sed -n 1p "$T/askpass.log")" = "" -a "$(sed -n 3p "$T/askpass.log")" = "$pw" \
         -a "$(grep -c '^not-the-password-' "$T/askpass.log")" = 2 -a "$(wc -l <"$T/askpass.log")" = 4
check "the password is in no record the proof keeps: not the first boot's, the session's log or its steps" \
    bash -c '! grep -rqF -- "$1" "$2"' _ "$pw" "$T/out"
check "the session log marks each password as sent from its file, by name alone" \
    test "$(grep -c '^\[sent: <file pw>\]$' "$T/out/first-boot-session.log")" = 2
check "and the proof's directory, the password file with it, is gone" test ! -e "$VM_ROOT/first"
check "the console is QEMU's socket, with its log beside it, under the first boot's own directory" \
    test "$(grep -c -- "-chardev socket,id=con0,path=$VM_ROOT/first/con/console.sock,server=on,wait=off,logfile=$VM_ROOT/first/con/console.log -serial chardev:con0" "$T/vi-args")" = 1
check "the first boot's console log is kept, the stand-in's session in it" \
    test "$(grep -c 'You are required to change your password immediately' "$T/out/first-boot-console.log")" = 1
check "and its MANIFEST comparison marks machine-id, the shadow file and the host key expected, with 0 findings" \
    test "$(fb | grep -c '(expected)$')" = 3 -a "$(fb | grep -c '/etc/shadow.*(expected)$')" = 1 -a "$(fb | grep -c ', 0 finding(s)$')" = 1
check "and reads the inbound policy" test "$(fb | grep -c 'base inbound policy (stand-in)')" = 1
check "the in-guest reads parse as bash" bash -n "$T/reads.sh"
check "they time the boot: systemd-analyze's time, both critical chains, time-wait-sync and the units running" \
    test "$(grep -cE 'systemd-analyze time|critical-chain --no-pager multi-user.target|critical-chain --no-pager serial-getty@ttyS0.service|systemd-time-wait-sync.service|--state=running' "$T/reads.sh")" = 6
# The boot-finished and time-wait-sync reads, as the guest runs them, against stand-ins.
BL=$(sed -n '/^                echo "== the boot, as systemd timed it"/,/^                case \$o in/p' "$T/reads.sh")
br() { bash -c 'systemd-analyze() { echo "$A"; }; systemctl() { echo "systemd-time-wait-sync.service start running"; }; '"$BL" _ ; }
check "a boot not finished is a PROOF-FAIL naming the waiting job; a finished one is read" \
    test "$(wc -l <<<"$BL")" = 2 -a "$(A='Startup finished in 1.2s (kernel) + 3.4s (userspace) = 4.6s' br | grep -c PROOF-FAIL)" = 0 \
         -a "$(A='Bootup is not yet finished (org.freedesktop.systemd1.Manager.FinishTimestampMonotonic=0).' br | grep -c '^PROOF-FAIL: the boot had not finished after the first login: systemd-time-wait-sync.service start running')" = 1
WL=$(grep -F 'systemctl is-enabled systemd-time-wait-sync.service' "$T/reads.sh")
check "time-wait-sync not masked is a PROOF-FAIL" \
    test -n "$WL" -a -z "$(bash -c 'systemctl() { echo masked; }; '"$WL")" \
         -a "$(bash -c 'systemctl() { echo enabled; }; '"$WL")" = "PROOF-FAIL: time-wait-sync is not masked"
check "the power button is pressed after the reads, on the first boot's guest and its console log" \
    test "$(cat "$T/out/power-off.txt")" = "== the power button (stand-in) mock-appliance-first console.log"
at() { grep -n -F -- "$1" "$T/out/first-boot.txt" | sed -n 1p | cut -d: -f1; }
check "the ready row is read after the first login and before the tries after it, on the proof's port" \
    eval 'r=$(at "== the ready layer (stand-in) out 2330"); [ -n "$r" ] && [ "$(at "== the first login, at the console")" -lt "$r" ] && [ "$r" -lt "$(at "== SSH as root after the first login")" ]'
check "the second boot follows the first press, and its own press follows it" \
    test "$(cat "$T/out/second-boot.txt")" = "== the second boot (stand-in) mock-appliance-first" \
         -a "$(cat "$T/out/second-power-off.txt")" = "== the power button (stand-in) mock-appliance-first console.log"
check "the booted listing leaves out the connectors' own folders, which are listed by path alone" \
    eval 'grep -qF -- "-not -path '"'"'/etc/weaver-web/pairing/gate/*'"'"' -not -path '"'"'/etc/weaver-web/pairing/admin/*'"'"' -print0" "$T/files-cmd"'
touch "$T/no-second"; run; rm -f -- "$T/no-second"
check "no second boot: no second press, and the record says so" \
    test "$(cat "$T/out/second-power-off.txt")" = "no second boot, so no second press of the power button"
touch "$T/ready-files"; run; rm -f -- "$T/ready-files"
check "the configs, by path alone, and the requests, by digest, are the first boot's expected additions" \
    eval '[ "$(fb | grep -c "^PROOF-FAIL")" = 0 ] && fb | grep -qx "added file: /etc/weaver-web/pairing/gate/gate-con.toml (by its path alone) (expected)" && fb | grep -qx "added file: /etc/weaver-web/pairing/admin/admin-con.toml (by its path alone) (expected)" && fb | grep -qx "added file: /etc/weaver-web/pairing/exchange/gate.req (expected)" && fb | grep -qx "added file: /etc/weaver-web/pairing/exchange/admin.req (expected)"'
# The ESP read, as the guest runs it, against a stand-in findmnt.
EL=$(sed -n '/^                echo "== the ESP at \/efi"/,/^                case ",\$(findmnt -nvo OPTIONS \/efi/p' "$T/reads.sh")
er() { bash -c 'eo=$1; findmnt() { case "$*" in "-nvo OPTIONS /efi") echo "$eo" ;; *) echo "/dev/vda2 vfat $eo" ;; esac; }; '"$EL" _ "$1"; }
check "the ESP read: ro passes, rw is a PROOF-FAIL naming the mount" \
    test "$(wc -l <<<"$EL")" = 2 -a "$(er ro,nosuid,umask=0077 | grep -c '^the ESP is mounted read-only$')" = 1 \
         -a "$(er rw,nosuid | grep -c '^PROOF-FAIL: /efi is not mounted read-only (/dev/vda2 vfat rw,nosuid)$')" = 1 \
         -a "$(er rw,nosuid | grep -c 'mounted read-only$')" = 0
check "the hostname reads are the kernel's (uname -n), with no hostname command" \
    eval '[ "$(grep -o "uname -n" "$T/reads.sh" | wc -l)" = 3 ] && ! grep -vE "^[[:space:]]*#" "$T/reads.sh" | grep -qE "(^|[;(])[[:space:]]*hostname[[:space:]]*([);]|\$)"'
# The sudo read, as the guest runs it, against a stand-in sudo.
SL=$(grep -F 'if sudo -n -u nobody true 2>&1; then' "$T/reads.sh")
sr() { bash -c 'sudo() { return '"$1"'; }; '"$SL"; }
check "sudo as nobody after the first login: read, and a refusal is a PROOF-FAIL" \
    test -n "$SL" -a "$(sr 0)" = "sudo runs a command as nobody" -a "$(sr 1 | grep -c '^PROOF-FAIL: sudo would not run a command as nobody')" = 1
check "a seed is attached, as a SATA cdrom beside the overlay" \
    test "$(cat "$T/iso-path")" = "$VM_ROOT/first/seed.iso" -a "$(grep -cxF -- "path=$VM_ROOT/first/seed.iso,device=cdrom,bus=sata" "$T/vi-args")" = 1
check "its meta-data names a hostname of its own and the proof key" \
    test "$(grep -cx 'local-hostname: weaver-proof-seed' "$T/out/first-boot-seed-meta-data.txt")" = 1 \
         -a "$(grep -cxF -- "  - $(cut -d' ' -f1,2 "$KEY.pub")" "$T/out/first-boot-seed-meta-data.txt")" = 1
check "its user-data asks for a bootcmd, a runcmd and the default account" \
    test "$(grep -c 'weaver-proof-bootcmd' "$T/out/first-boot-seed-user-data.txt")" = 1 -a "$(grep -c 'weaver-proof-runcmd' "$T/out/first-boot-seed-user-data.txt")" = 1 \
         -a "$(grep -cx '  - default' "$T/out/first-boot-seed-user-data.txt")" = 1
check "the in-guest reads require cloud-init absent, its directories gone, and none of the asks carried out" \
    test "$(grep -c 'PROOF-FAIL: cloud-init is installed' "$T/reads.sh")" = 1 -a "$(grep -c 'for d in /etc/cloud /var/lib/cloud; do' "$T/reads.sh")" = 1 \
         -a "$(grep -c '/var/tmp/weaver-proof-bootcmd /var/tmp/weaver-proof-runcmd' "$T/reads.sh")" = 1 -a "$(grep -c "default account was made" "$T/reads.sh")" = 1
# The network, handed in as a systemd credential through SMBIOS type 11: the argument names
# the credential and carries the network file the proof keeps, base64 and with no comma
# (QEMU's option separator); the file matches the NIC's MAC and names the static address,
# gateway and resolver QEMU's user network serves.
sm=$(grep -oE -- '-smbios [^ ]+' "$T/vi-args" | head -1); sm=${sm#-smbios }
check "the first boot is given one -smbios argument, a type 11 credential named $PROOF_CREDENTIAL" \
    test "$(grep -oE -- '-smbios ' "$T/vi-args" | wc -l)" = 1 -a "${sm%%=*}" = "type" \
         -a "$(grep -c "^type=11,value=io.systemd.credential.binary:$PROOF_CREDENTIAL=[A-Za-z0-9+/=]*\$" <<<"$sm")" = 1
check "its value decodes to the network file the proof keeps" \
    cmp -s <(base64 -d <<<"${sm##*:$PROOF_CREDENTIAL=}") "$T/out/first-boot-network.txt"
check "the network file matches the NIC's MAC, the same the -device names, and the static address, gateway and resolver" \
    test "$(grep -cx "MACAddress=$PROOF_MAC" "$T/out/first-boot-network.txt")" = 1 -a "$(grep -c "mac=$PROOF_MAC," "$T/vi-args")" = 1 \
         -a "$(grep -cxE 'Address=10\.0\.2\.15/24|Gateway=10\.0\.2\.2|DNS=10\.0\.2\.3' "$T/out/first-boot-network.txt")" = 3
check "and nothing is written into the overlay: no image_open, no placement" \
    bash -c '! grep -qE "image_open|image_put|proof_place" <<<"$1"' _ "$(fn "$BT" proof_first_boot proof_network)"
# The default account's read, as the guest runs it, against a stand-in getent: its status 2
# is an account not made, 0 one made, and any other a read that did not happen.
sed -n '/getent passwd arch/,/^ *esac$/p' "$T/reads.sh" >"$T/getent.sh"
ga() { bash -c 'getent() { return '"$1"'; }; . "$0"' "$T/getent.sh" 2>&1; }
check "the default account's read: getent's status 2 is not made, and no PROOF-FAIL" \
    test "$(ga 2)" = "not made: the default account"
check "getent's status 0 is the default account made, a PROOF-FAIL" \
    test "$(ga 0)" = "PROOF-FAIL: the seed's default account was made"
check "any other status is a read that did not happen, a PROOF-FAIL, not an account not made" \
    test "$(ga 3)" = "PROOF-FAIL: getent exited 3, so whether the default account exists is not known"
# The data mount points' read, as the guest runs it, against a stand-in tree: its paths
# pointed at the tree, root's ownership read as this account's, and the root filesystem as
# the tree's own.
me="$(id -u):$(id -g)"; MT=$T/mounts
sed -n '/== the data mount points, with no disk attached/,/^ *done$/p' "$T/reads.sh" \
    | sed -e "s|/mnt/weaver|$MT/mnt/weaver|g" -e "s|directory 0:0 755|directory $me 755|" -e "s|stat -c %d -- /)|stat -c %d -- $MT)|" >"$T/mounts.sh"
check "the in-guest reads include the data mount points, both of them" \
    test "$(grep -c "for d in $MT/mnt/weaver/models $MT/mnt/weaver/state; do" "$T/mounts.sh")" = 1
mt() { rm -rf -- "${MT:?}"; mkdir -p "$MT/mnt/weaver/models" "$MT/mnt/weaver/state"; chmod 0755 "$MT/mnt/weaver/models" "$MT/mnt/weaver/state"; }
mt
check "two empty directories 0755 on the root's filesystem: read, and no PROOF-FAIL" \
    test "$(bash "$T/mounts.sh" | grep -c '^PROOF-FAIL')" = 0 -a "$(bash "$T/mounts.sh" | grep -c ": directory $me 755, 0 entries$")" = 2
mt; : >"$MT/mnt/weaver/state/x"; rmdir "$MT/mnt/weaver/models"
check "a mount point absent, or one that holds a file: each a PROOF-FAIL" \
    test "$(bash "$T/mounts.sh" | grep -c '^PROOF-FAIL: .* is not an empty directory root owns with mode 755$')" = 2
mt
check "a mount point on another filesystem than the root's: a PROOF-FAIL" \
    test "$(sed "s|stat -c %d -- $MT)|stat -c %d -- /proc)|" "$T/mounts.sh" | bash | grep -c '^PROOF-FAIL: .* has a filesystem mounted on it, with no disk attached$')" = 2
touch "$T/no-iso"; run; rc=$?; rm -f -- "$T/no-iso"
check "a seed image that cannot be made refuses, and nothing boots" \
    test "$rc" != 0 -a ! -e "$T/vi-args" -a "$(grep -c "REFUSED: the first boot's seed image could not be made; nothing booted" "$T/run.out")" = 1
touch "$T/scan-split"; run; rm -f -- "$T/scan-split"
check "a first scan that misses a key and a retry that finds it: all three kept, and they match" \
    test "$(fb | grep -c "$match3")" = 1 -a "$(fb | grep -c '^PROOF-FAIL')" = 0 -a "$(fb | grep -c '1 more scans$')" = 1
touch "$T/booted-extra"; run; rm -f -- "$T/booted-extra"
check "a first boot that changes /etc/passwd fails: the account files are off the expected list" \
    test "$(fb | grep -c '^PROOF-FAIL: the first boot changed a MANIFEST file outside the expected list: /etc/passwd$')" = 1
printf 'Arch Linux 7.3.4-arch1-2 (ttyS0)\n\nweaver-appliance-build5 login: ' >"$T/console"
run
check "a console that greets with the build guest's name fails" \
    test "$(fb | grep -c "^PROOF-FAIL: the console's login prompt is 'weaver-appliance-build5 login:', not the image's hostname")" = 1
{ printf 'Arch Linux 7.3.4-arch1-2 (ttyS0)\n\n%s login: \n' "$IMAGE_HOSTNAME"; head -c 400000 /dev/zero | tr '\0' x | fold -w 100; } >"$T/console"
run
check "a console with 400 kB after the prompt is read at once, not at the bound (the wait under pipefail)" \
    test "$(fb | grep -c '^== first boot, 0 s to the console.s login prompt')" = 1 -a "$(fb | grep -c '^PROOF-FAIL')" = 0
: >"$T/console"; run
check "a console with no login prompt fails, after the bound" \
    test "$(fb | grep -c "^PROOF-FAIL: the console's login prompt is 'none'")" = 1 -a "$(fb | grep -c '^== first boot, 4 s to')" = 1
printf '%s login: ' "$IMAGE_HOSTNAME" >"$T/console"
touch "$T/scan-other"; run; rm -f -- "$T/scan-other"
check "a key SSH presents that the guest does not hold fails, and nothing more is read" \
    test "$(fb | grep -c '^PROOF-FAIL: the host keys: the guest.s fingerprints do not name the keys SSH presents$')" = 1 \
         -a "$(fb | grep -c 'cloud-init, with a seed attached')" = 0 -a "$(fb | grep -c 'MANIFEST against')" = 0
touch "$T/no-login"; run; rm -f -- "$T/no-login"
check "a login as root that is refused fails the host-key match" \
    test "$(fb | grep -c '^PROOF-FAIL: the host keys:')" = 1 -a "$(fb | grep -c 'exited non-zero')" = 1
printf 'Arch Linux 7.3.4-arch1-2 (ttyS0)\n\n%s login: ' "$IMAGE_HOSTNAME" >"$T/console"
# The first login, gone wrong each way the stand-in console plays.
echo asks-password >"$T/con-mode"; run
check "a login that asks root for a password fails, and the driver stops at once" \
    test "$(fb | grep -c '^PROOF-FAIL: the first login: login asked root for a password$')" = 1 \
         -a "$(fb | grep -c '^PROOF-FAIL: the first login: the console driver stopped (exit 1)')" = 1
echo no-change >"$T/con-mode"; run
check "a login that lands in a shell with no change required fails" \
    test "$(fb | grep -c '^PROOF-FAIL: the first login: login did not announce the forced change$')" = 1 \
         -a "$(fb | grep -c '^PROOF-FAIL: the first login: no new password was asked$')" = 1
echo mismatch >"$T/con-mode"; run
check "a change that fails fails, by its own words" \
    test "$(fb | grep -c "^PROOF-FAIL: the first login: the change said 'Authentication token manipulation error'$")" = 1
echo wrong-tty >"$T/con-mode"; run
check "a session on another terminal than the serial console fails" \
    test "$(fb | grep -c '^PROOF-FAIL: the first login: the session.s terminal is /dev/tty1, not /dev/ttyS0$')" = 1
echo pass >"$T/con-mode"; touch "$T/ssh-pw-offered"; run; rm -f -- "$T/ssh-pw-offered"
check "a server that offers passwords fails each of the five tries" \
    test "$(fb | grep -c '^PROOF-FAIL: SSH as root with .*: the server offered publickey,password$')" = 5 \
         -a "$(fb | grep -c '^PROOF-FAIL: SSH as root with the proof key, not yet placed: the server offered publickey,password$')" = 1
# proof_power_off, against a stand-in virsh: the press appends the shutdown's console text
# to the log, as QEMU's logfile takes it; the state and the stop's reason are the case's.
LOGF=$T/acpi.log
OKOFF=$'[\e[0;32m  OK  \e[0m] Reached target \e[0;1;39mSystem Power Off\e[0m.\r\n[   41.002113] reboot: Power down\r\n'
virsh_() { case $1 in
    shutdown) printf '%s\n' "$*" >"$T/acpi-args"; [ -e "$T/acpi-refuse" ] && return 1; printf '%s' "$ACPI_TAIL" >>"$LOGF" ;;
    domstate) if [ "${3:-}" = --reason ]; then echo "$ACPI_REASON"; else echo "$ACPI_STATE"; fi ;;
    esac; }
pr() { printf '[\e[0;1;31mFAILED\e[0m] Failed to start an earlier unit.\r\nweaver-appliance login: ' >"$LOGF"
    ( set -euo pipefail; real_proof_power_off mock-appliance-first "$LOGF" ) 2>&1; }
ACPI_STATE="shut off" ACPI_REASON="shut off (shutdown)" ACPI_TAIL=$OKOFF
check "a clean ACPI power-off: no PROOF-FAIL, the reason read, a failure before the press not counted" \
    eval 'o=$(pr); [ "$(grep -c "^PROOF-FAIL" <<<"$o")" = 0 ] && grep -qx "the domain: shut off (shutdown), 0s after the press" <<<"$o" && grep -q "reboot: Power down" <<<"$o"'
check "the press is ACPI's, on the named domain" test "$(cat "$T/acpi-args")" = "shutdown mock-appliance-first --mode acpi"
ACPI_TAIL=$'[  OK  ] Reached target System Power Off.\r\n'
check "no kernel power-down line is a PROOF-FAIL" eval 'o=$(pr); [ "$(grep -c "^PROOF-FAIL" <<<"$o")" = 1 ] && grep -q "^PROOF-FAIL: the serial log after the power button holds no kernel power-down line" <<<"$o"'
ACPI_TAIL=$'[\e[0;1;31mFAILED\e[0m] Failed unmounting /efi.\r\n[ ***  ] A stop job is running for x (5s / 1min 30s)\r\n'"$OKOFF"
check "a unit failing and a stop job waited on: one PROOF-FAIL counting both" \
    eval 'grep -q "^PROOF-FAIL: the power-off was not clean: 2 unit(s) failed or a stop job was waited on" <<<"$(pr)"'
ACPI_TAIL=""; ACPI_STATE=running ACPI_REASON="running (booted)"
check "a guest that does not power off: PROOF-FAILs for the reason, the target and the kernel line, at the bound" \
    eval 'o=$(PROOF_ACPI=6 pr); grep -q "^PROOF-FAIL: mock-appliance-first did not power itself off within 6s of the power button (running (booted))" <<<"$o" && grep -qx "the domain: running (booted), 6s after the press" <<<"$o" && [ "$(grep -c "^PROOF-FAIL" <<<"$o")" = 3 ]'
touch "$T/acpi-refuse"
check "a press the hypervisor refuses: one PROOF-FAIL, and nothing more read" \
    eval 'o=$(pr); [ "$(grep -c "^PROOF-FAIL" <<<"$o")" = 1 ] && grep -q "could not press mock-appliance-first.s power button" <<<"$o" && ! grep -q "the domain:" <<<"$o"'
rm -f -- "$T/acpi-refuse"
check "and the proof gathers the power-offs' and the second boot's findings with the others" \
    eval 'grep -q "\"\$out/first-boot.txt\" \"\$out/power-off.txt\" \"\$out/second-boot.txt\" \"\$out/second-power-off.txt\" \"\$out/disk.txt\" >\"\$out/PROOF-FAILED\"" "$BT"'
# A VM_ROOT whose socket path a unix socket cannot hold refuses before anything boots.
long=$T/$(printf 'x%.0s' $(seq 120)); mkdir -p "$long"
rm -f -- "$T/vi-args"; ( set -euo pipefail; VM_ROOT=$long; proof_first_boot "$T/out" ) >"$T/long.out" 2>&1; rc=$?
check "a socket path past 107 bytes refuses, and nothing boots" \
    test "$rc" != 0 -a "$(grep -c "is longer than a unix socket allows (107 bytes); set VM_ROOT shorter; nothing booted" "$T/long.out")" = 1 \
         -a "$(grep -c "$long" "$T/vi-args" 2>/dev/null || echo 0)" = 0
finish
