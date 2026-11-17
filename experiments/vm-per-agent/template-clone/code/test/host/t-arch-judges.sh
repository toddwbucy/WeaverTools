#!/usr/bin/env bash
# The Arch base's judges (release 2.0), each from its own file's text, on stand-ins: the
# signature check of the cloud image's SHA256 file (lib.sh's sig_primary_ok), and from
# guest/template-steps the removal judge, pacman's sources and hold, the one kernel, unit
# links that lead nowhere, the CUDA runtime's file list and the tuple's runtime files, and
# the firewall, resolver, imds and update checks. Each function's root prefix is dropped,
# its paths pointed at the case's own tree, and pacman and systemctl stood in where it asks
# them. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
STEPS=$CODE/guest/template-steps
P='s''udo '
steps() { fn "$STEPS" "$@" | sed -e "s|$P||g"; }
const() { eval "$(sed -n "/^$1=/p" "$STEPS")"; }   # a one-line constant, as the file sets it
# has PATTERN CMD...: CMD's output holds PATTERN. Read whole, then searched: under pipefail a
# judge's own status 1, or a write cut short by grep -q, must not decide the case.
has() { local pat=$1; shift; grep -q -- "$pat" <<<"$("$@" 2>&1)"; }

# ---- 1. sig_primary_ok: gpg's status lines, the primary key's fingerprint last on VALIDSIG.
eval "$(fn "$CODE/lib.sh" sig_primary_ok)" || exit 1
PRI=1B9A16984A4E8CB448712D2AE0B78BF4326C6F8F SUB=0123456789ABCDEF0123456789ABCDEF01234567
# status SIGNING PRIMARY: gpg's status for a good signature by SIGNING, whose primary is PRIMARY.
status() { printf '%s\n' '[GNUPG:] NEWSIG' '[GNUPG:] GOODSIG 2AE0B78BF4326C6F8F Arch Linux Release Engineering' \
    "[GNUPG:] VALIDSIG $1 2026-11-15 1763164800 0 4 0 22 10 00 $2" '[GNUPG:] TRUST_UNDEFINED 0 pgp'; }
GOOD=$(status "$PRI" "$PRI")
check "GOODSIG, and VALIDSIG naming the primary last: passes" sig_primary_ok "$GOOD" "$PRI"
check "a subkey's signature whose primary is the pinned key: passes" sig_primary_ok "$(status "$SUB" "$PRI")" "$PRI"
check "a signature by the pinned key whose primary is another key: refused" eval '! sig_primary_ok "$(status "$PRI" "$SUB")" "$PRI"'
check "a signature by another key with its own primary: refused" eval '! sig_primary_ok "$(status "$SUB" "$SUB")" "$PRI"'
check "GOODSIG with no VALIDSIG: refused" eval '! sig_primary_ok "$(grep -v VALIDSIG <<<"$GOOD")" "$PRI"'
check "VALIDSIG with no GOODSIG: refused" eval '! sig_primary_ok "$(grep -v GOODSIG <<<"$GOOD")" "$PRI"'
for s in "BADSIG 2AE0B78BF4326C6F8F x" "ERRSIG 2AE0B78BF4326C6F8F 22 10 00 1763164800 9 -" \
         "EXPSIG 2AE0B78BF4326C6F8F x" "EXPKEYSIG 2AE0B78BF4326C6F8F x" "REVKEYSIG 2AE0B78BF4326C6F8F x" "NO_PUBKEY 2AE0B78BF4326C6F8F"; do
    check "${s%% *} beside a good signature: refused" eval '! sig_primary_ok "$GOOD
[GNUPG:] $s" "$PRI"'
done
check "the words outside gpg's status lines count for nothing" eval '! sig_primary_ok "GOODSIG x
VALIDSIG $PRI x $PRI" "$PRI"'
check "and an empty status: refused" eval '! sig_primary_ok "" "$PRI"'

# ---- 2. removal_judge: a simulated removal against the kept set and the base's packages.
eval "$(steps removal_judge)" || exit 1
J=$T/judge; mkdir -p "$J"
printf '%s\n' weaver-cuda-runtime acl gcc-libs python util-linux >"$J/keep"
printf '%s\n' base linux systemd sudo openssh cloud-init cloud-guest-utils python util-linux >"$J/base"
printf '%s\n' cloud-init cloud-guest-utils >"$J/purge"
rj() { printf '%s\n' "$@" >"$J/sim"; removal_judge "$J/sim" "$J/keep" "$J/base" "$J/purge" 2>&1; }
check "build packages alone: no finding" test -z "$(rj gcc15 cuda cmake clang llvm)"
check "the purged base packages, named: no finding" test -z "$(rj cloud-init cloud-guest-utils python-jsonschema)"
check "a kept package in the simulation: refused, named" eval 'o=$(rj gcc15 acl); [ $? = 1 ] && grep -qx "the simulation would remove acl, which is kept. *" <<<"$o"'
check "a kept package that is also a base package: both named" eval 'o=$(rj python); grep -q "remove python, which is kept" <<<"$o" && grep -q "base packages not named: python " <<<"$o"'
check "a base package not named: refused, named" eval 'grep -q "base packages not named: sudo " <<<"$(rj cmake sudo)"'
check "a name that only contains a kept one is not it" test -z "$(rj python-setuptools acl-extra)"

# ---- 3. pacman's sources: [core] and [extra] from the snapshot's mirrorlist alone.
const ARCHIVE_DAY; ARCHIVE=$(sed -n 's/^ARCHIVE=//p' "$STEPS"); ARCHIVE=${ARCHIVE//\$ARCHIVE_DAY/$ARCHIVE_DAY}
eval "$(steps pacman_sources_check)" || exit 1
check "the archive is the snapshot's day" test "$ARCHIVE" = "https://archive.archlinux.org/repos/2026/11/15"
S=$T/src; mkdir -p "$S"
conf() {
    printf '%s\n' '[options]' 'HoldPkg = pacman glibc' '#IgnorePkg =' 'Architecture = auto' '' \
        '#[core-testing]' '#Include = /etc/pacman.d/mirrorlist' '' \
        '[core]' "Include = $S/mirrorlist" '' '[extra]' "Include = $S/mirrorlist" >"$S/pacman.conf"
    printf '%s\n' '## the snapshot' "Server = $ARCHIVE/\$repo/os/\$arch" >"$S/mirrorlist"
}
ps() { pacman_sources_check "$S/pacman.conf" "$S/mirrorlist"; }
conf
check "core and extra from the snapshot alone, the testing repositories commented: no finding" test -z "$(ps)"
conf; printf '%s\n' '[multilib]' "Include = $S/mirrorlist" >>"$S/pacman.conf"
check "a third repository: named" test "$(ps)" = "pacman.conf names the repository multilib"
conf; sed -i "/^\[core\]$/a Server = https://mirror.example/\$repo/os/\$arch" "$S/pacman.conf"
check "a server of its own in [core]: named" test "$(ps)" = "pacman.conf names a server of its own in [core]"
conf; sed -i "0,/^Include = /s|^Include = .*|Include = /etc/pacman.d/mirrorlist|" "$S/pacman.conf"
check "[core] from another mirrorlist: named" test "$(ps)" = "[core] includes /etc/pacman.d/mirrorlist, not $S/mirrorlist"
conf; sed -i '/^\[extra\]$/,$d' "$S/pacman.conf"
check "no [extra]: named" test "$(ps)" = "pacman.conf names no [extra]"
conf; echo 'Server = https://geo.mirror.pkgbuild.com/$repo/os/$arch' >>"$S/mirrorlist"
check "a mirror beside the snapshot: named" test "$(ps)" = "$S/mirrorlist is not the archive's snapshot alone (Server = $ARCHIVE/\$repo/os/\$arch)"
conf; sed -i 's|/repos/2026/11/15/|/repos/2026/11/16/|' "$S/mirrorlist"
check "another day's snapshot: named" test -n "$(ps)"

# ---- 4. The hold: pacman_ignore writes one IgnorePkg line, ignored_check reads it.
eval "$(steps pacman_ignore ignored_check)" || exit 1
const PACMAN_IGNORE
check "the hold names the kernel, the driver's two packages and the runtime" test "$PACMAN_IGNORE" = "linux nvidia-open nvidia-utils weaver-cuda-runtime"
conf
# shellcheck disable=SC2086
pacman_ignore "$S/pacman.conf" $PACMAN_IGNORE
check "pacman_ignore replaces the commented line with one, under [options]" \
    test "$(grep -c IgnorePkg "$S/pacman.conf")" = 1 -a "$(sed -n '/^\[options\]$/{n;p}' "$S/pacman.conf")" = "IgnorePkg = $PACMAN_IGNORE"
# shellcheck disable=SC2086
pacman_ignore "$S/pacman.conf" $PACMAN_IGNORE
check "and run again, still one line" test "$(grep -c IgnorePkg "$S/pacman.conf")" = 1
# shellcheck disable=SC2086
check "ignored_check passes it" ignored_check "$S/pacman.conf" $PACMAN_IGNORE
check "a name the line lacks is named" eval '[ "$(ignored_check "$S/pacman.conf" linux cuda)" = "IgnorePkg does not name cuda. " ]'
sed -i 's/^IgnorePkg = .*/IgnorePkg = linux nvidia nvidia-utils/' "$S/pacman.conf"
check "nvidia does not stand for nvidia-open" eval '! ignored_check "$S/pacman.conf" nvidia-open >/dev/null'
echo 'IgnorePkg = cuda' >>"$S/pacman.conf"
check "two IgnorePkg lines: named" eval 'grep -q "has 2 IgnorePkg lines, not one" <<<"$(ignored_check "$S/pacman.conf" linux)"'

# ---- 5. one_kernel: one modules tree, the pinned one, and /boot holding its kernel alone.
const PIN_KVER
eval "$(steps one_kernel | sed -e "s|/usr/lib/modules|\$K/usr/lib/modules|g" -e "s|/boot|\$K/boot|g")" || exit 1
kern() {
    K=$(mktemp -d "$T/k.XXXXXX"); KVER=$PIN_KVER
    mkdir -p "$K/usr/lib/modules/$KVER" "$K/boot/grub"
    echo kernel >"$K/usr/lib/modules/$KVER/vmlinuz"; cp "$K/usr/lib/modules/$KVER/vmlinuz" "$K/boot/vmlinuz-linux"
    : >"$K/boot/initramfs-linux.img"
}
kern; check "one tree, the pinned kernel, /boot as built: no finding" one_kernel
kern; mkdir -p "$K/usr/lib/modules/7.3.3-arch1-1"
check "a second modules tree: named" has "more than one kernel" one_kernel
kern; : >"$K/boot/initramfs-linux-fallback.img"
check "a fallback initramfs left in /boot: named" has "/boot holds initramfs-linux-fallback.img" one_kernel
kern; echo other >"$K/boot/vmlinuz-linux"
check "a kernel in /boot that is not the tree's: named" has "is not $K/usr/lib/modules/$KVER/vmlinuz" one_kernel
kern; KVER=7.3.3-arch1-1; mkdir -p "$K/usr/lib/modules/$KVER"
check "a running kernel that is not the pin: named" has "the running kernel is 7.3.3-arch1-1, not $PIN_KVER" one_kernel

# ---- 6. unit_links_check: links under /etc/systemd that lead nowhere; a mask leads to
# /dev/null, which stands.
eval "$(steps unit_links_check | sed -e "s|/etc/systemd|\$U/etc/systemd|g")" || exit 1
U=$T/units; mkdir -p "$U/etc/systemd/system/multi-user.target.wants" "$U/usr/lib/systemd/system"
: >"$U/usr/lib/systemd/system/sshd.service"
ln -s "$U/usr/lib/systemd/system/sshd.service" "$U/etc/systemd/system/multi-user.target.wants/sshd.service"
ln -s /dev/null "$U/etc/systemd/system/systemd-sysupdate.timer"
check "a link to a unit and a mask: no finding" test -z "$(unit_links_check)"
ln -s "$U/usr/lib/systemd/system/cloud-init-main.service" "$U/etc/systemd/system/multi-user.target.wants/cloud-init-main.service"
check "a link to a unit removed with its package: named" \
    test "$(unit_links_check)" = "$U/etc/systemd/system/multi-user.target.wants/cloud-init-main.service -> $U/usr/lib/systemd/system/cloud-init-main.service (leads nowhere)"

# ---- 7. The CUDA runtime's file list: the six libraries' real files, the links to them,
# the lib64 link and the loader's file; nothing else of the toolkit.
eval "$(steps runtime_list runtime_resolved_check runtime_hashes tuple_runtime_check sha | sed -e "s|/etc/ld.so.conf.d|\$C/etc/ld.so.conf.d|g")" || exit 1
const RUNTIME_LIBS; const RUNTIME_PKG
C=$T/cuda-tree; CUDA_HOME=$C/opt/cuda; L=$CUDA_HOME/targets/x86_64-linux/lib; RUNTIME_DIR=$T/runtime
mkdir -p "$L" "$RUNTIME_DIR" "$C/etc/ld.so.conf.d"; ln -s targets/x86_64-linux/lib "$CUDA_HOME/lib64"
for n in cudart.so.13.4.39 cublas.so.13.4.1.2 cublasLt.so.13.4.1.2 curand.so.10.4.1.9 nvrtc.so.13.4.42 nvrtc-builtins.so.13.4.42 \
         cufft.so.12.4.0.7 cudart_static.a; do head -c 64 /dev/urandom >"$L/lib$n"; done
ln -s libcudart.so.13.4.39 "$L/libcudart.so.13"; ln -s libcudart.so.13 "$L/libcudart.so"
ln -s libcublasLt.so.13.4.1.2 "$L/libcublasLt.so.13"; ln -s libcufft.so.12.4.0.7 "$L/libcufft.so.12"
echo "$CUDA_HOME/lib64" >"$C/etc/ld.so.conf.d/cuda.conf"
runtime_list "$L" >"$T/files"
want=$(printf '%s\n' "$CUDA_HOME/lib64" "$L/libcublas.so.13.4.1.2" "$L/libcublasLt.so.13" "$L/libcublasLt.so.13.4.1.2" \
       "$L/libcudart.so" "$L/libcudart.so.13" "$L/libcudart.so.13.4.39" "$L/libcurand.so.10.4.1.9" \
       "$L/libnvrtc-builtins.so.13.4.42" "$L/libnvrtc.so.13.4.42" | LC_ALL=C sort)
check "the list: the six real files, links to them at any depth and lib64; not the toolkit's loader file" test "$(cat "$T/files")" = "$want"
check "and nothing of cufft, the static archive, or the list's scratch file" \
    test "$(grep -c 'cufft\|_static' "$T/files")" = 0 -a ! -e "$RUNTIME_DIR/real.tmp"
printf '\tlibcudart.so.13 => %s/lib64/libcudart.so.13 (0x0)\n\tlibc.so.6 => /usr/lib/libc.so.6 (0x0)\n' "$CUDA_HOME" >"$T/ldd"
check "a library the stack resolves inside the list: no finding" test -z "$(runtime_resolved_check "$T/files" "$T/ldd")"
printf '\tlibcufft.so.12 => %s/lib64/libcufft.so.12 (0x0)\n' "$CUDA_HOME" >>"$T/ldd"
check "one outside it: named" has "libcufft.so.12 .* resolves for the stack and is not in the runtime package" runtime_resolved_check "$T/files" "$T/ldd"
runtime_hashes "$T/files" "$T/hashes"
check "the hashes: a sha256 line for each file, a LINK line for each link, nothing missing" \
    test "$(grep -c '^LINK ' "$T/hashes")" = 4 -a "$(grep -cE '^[0-9a-f]{64}  ' "$T/hashes")" = 6 -a "$(grep -c '^MISSING' "$T/hashes")" = 0
check "a link's line names its target as written" grep -qxF "LINK $L/libcudart.so -> libcudart.so.13" "$T/hashes"
# The tuple's runtime lines: "runtime.<name> = <sha256> <path>", each a file pacman says the
# runtime package owns, at that digest.
pacman() { [ "$1" = -Qoq ] && case $3 in "$L"/lib*) echo "$RUNTIME_PKG" ;; *) return 1 ;; esac; }
d=$(sha "$L/libcudart.so.13.4.39")
printf 'stack.commit = x\nruntime.cudart = %s %s\n' "$d" "$L/libcudart.so.13.4.39" >"$T/tuple"
check "a tuple naming the runtime's own file at its digest: no finding" test -z "$(tuple_runtime_check "$T/tuple")"
printf 'runtime.cufft = %s %s\n' "$(sha "$L/libcufft.so.12.4.0.7")" /usr/lib/libcufft.so.12 >>"$T/tuple"
check "a file that is not there: named" has "^runtime.cufft names /usr/lib/libcufft.so.12, which is not a file$" tuple_runtime_check "$T/tuple"
printf 'runtime.cudart = %s %s\n' "$d" "$L/libcudart.so.13.4.39" >"$T/tuple"; echo x >>"$L/libcudart.so.13.4.39"
check "a file not at the tuple's digest: named" has "is not the digest the tuple names" tuple_runtime_check "$T/tuple"
pacman() { return 1; }
check "a file no package owns: named" has "is not $RUNTIME_PKG.s" tuple_runtime_check "$T/tuple"
echo 'stack.commit = x' >"$T/tuple"
check "a tuple with no runtime line: named" test "$(tuple_runtime_check "$T/tuple")" = "$T/tuple names no runtime file"
unset -f pacman

# ---- 8. The firewall: the rules as built, and a unit that loads them and never unloads.
eval "$(awk '/^FIREWALL_RULES=/ { f = 1 } f { print } f && /^}'"'"'$/ { exit }' "$STEPS")"
# The rules file pointed at the case's tree where it is read; the unit's line is matched as written.
fwfn() { steps firewall_check | sed -e "s|) /etc/nftables.conf |) \$W/nftables.conf |" -e "s|-c -f /etc/nftables.conf|-c -f \$W/nftables.conf|"; }
eval "$(fwfn)" || exit 1
check "the firewall check reads the case's rules file in both places" test "$(fwfn | grep -c '\$W/nftables.conf')" = 2
W=$T/fw; mkdir -p "$W"
# Arch's nftables.service, read from the base image of 2026-11-15 (the survey of 2026-11-16).
arch_unit() { printf '%s\n' '# /usr/lib/systemd/system/nftables.service' '[Unit]' 'Description=Netfilter Tables' \
    'Documentation=man:nft(8)' 'Wants=network-pre.target' 'Before=network-pre.target' '' '[Service]' 'Type=oneshot' \
    'ExecStart=/usr/bin/nft -f /etc/nftables.conf' '' '[Install]' 'WantedBy=multi-user.target' >"$W/unit"; }
systemctl() { case $1 in cat) cat "$W/unit" ;; is-enabled) cat "$W/enabled" ;; esac; }
nft() { [ ! -e "$W/nft-refuses" ]; }
fwset() { arch_unit; printf '%s\n' "$FIREWALL_RULES" >"$W/nftables.conf"; echo enabled >"$W/enabled"; rm -f -- "$W/nft-refuses"; }
fwset; check "the rules as built, Arch's unit as shipped, enabled: no finding" firewall_check
fwset; printf '%s\n' '# /etc/systemd/system/nftables.service.d/10-weaver.conf' '[Service]' 'ExecStop=/usr/bin/nft delete table inet weaver_appliance' >>"$W/unit"
check "a drop-in that unloads the table on stop: named, since a oneshot stops at once" \
    has "nftables.service carries ExecStop=/usr/bin/nft delete table inet weaver_appliance" firewall_check
fwset; echo 'RemainAfterExit=yes' >>"$W/unit"
check "a RemainAfterExit set: named" has "carries RemainAfterExit=yes" firewall_check
fwset; sed -i 's|^ExecStart=.*|ExecStart=/usr/bin/nft -f /etc/nftables.d/main.nft|' "$W/unit"
check "a unit loading another file: named" has "does not load /etc/nftables.conf" firewall_check
fwset; echo '# one more line' >>"$W/nftables.conf"
check "rules not as built: named" has "is not the image.s rules" firewall_check
fwset; : >"$W/nft-refuses"
check "rules nft's check refuses: named" has "nft.s check refuses" firewall_check
fwset; echo disabled >"$W/enabled"
check "the unit disabled: named" has "nftables.service is not enabled" firewall_check

# ---- 9. The resolver: no address, no LLMNR or mDNS but the image's own "no".
eval "$(sed -n '/^RESOLVED_SETTINGS=/,/'"'"'$/p' "$STEPS")"; const RESOLVED_DROPIN
check "the settings drop LLMNR, mDNS and the fallback resolvers" \
    test "$(grep -v '^#' <<<"$RESOLVED_SETTINGS")" = "$(printf '[Resolve]\nFallbackDNS=\nLLMNR=no\nMulticastDNS=no')"
V=$T/res
eval "$(steps resolved_check | sed -e "s|/etc/systemd|\$V/etc/systemd|g" -e "s|/run/systemd|\$V/run/systemd|g" -e "s|/usr/lib/systemd|\$V/usr/lib/systemd|g")" || exit 1
res() { rm -rf -- "${V:?}"; mkdir -p "$V/etc/systemd/resolved.conf.d"; RESOLVED_DROPIN=$V/etc/systemd/resolved.conf.d/10-weaver-appliance.conf
    printf '%s\n' "$RESOLVED_SETTINGS" >"$RESOLVED_DROPIN"; printf '[Resolve]\n#DNS=\n#FallbackDNS=\n#LLMNR=yes\n' >"$V/etc/systemd/resolved.conf"; }
res; check "the drop-in as built, resolved.conf as shipped, no /run or /usr/lib drop-in: no finding" resolved_check
res; sed -i 's/^#DNS=$/DNS=192.0.2.53/' "$V/etc/systemd/resolved.conf"
check "a resolver address in resolved.conf: named" has "/resolved.conf names a resolver address" resolved_check
res; mkdir -p "$V/usr/lib/systemd/resolved.conf.d"; printf '[Resolve]\nMulticastDNS=yes\n' >"$V/usr/lib/systemd/resolved.conf.d/mdns.conf"
check "a drop-in under /usr/lib that sets mDNS, with none under /run (a pattern that matches nothing): named" has "/mdns.conf sets LLMNR or MulticastDNS" resolved_check
res; echo 'DNSStubListener=no' >>"$RESOLVED_DROPIN"
check "a drop-in not as built: named" has "is not the image.s settings" resolved_check

# ---- 10. imds and the update paths: each unit masked, and the generator and pacman-init
# masked by a link to /dev/null.
eval "$(sed -n '/^IMDS_UNITS=/,/"$/p; /^UPDATE_UNITS=/,/"$/p' "$STEPS")"; const IMDS_GENERATOR
eval "$(steps imds_check updates_check | sed -e "s|/etc/systemd|\$M/etc/systemd|g")" || exit 1
M=$T/masks; mkdir -p "$M/etc/systemd/system-generators" "$M/etc/systemd/system"
systemctl() { [ "$1" = is-enabled ] && { [ -e "$M/unmasked-$2" ] && echo enabled || echo masked; }; }
ln -s /dev/null "$M/etc/systemd/system-generators/$IMDS_GENERATOR"; ln -s /dev/null "$M/etc/systemd/system/pacman-init.service"
check "every imds unit masked, its generator masked: no finding" imds_check
check "every update unit masked, pacman-init a mask: no finding" updates_check
check "the lists name the imds generator's units and the sysupdate and keyring timers" \
    test "$(wc -w <<<"$IMDS_UNITS")" -ge 3 -a "$(grep -c systemd-sysupdate.timer <<<"$UPDATE_UNITS")" = 1 -a "$(grep -c archlinux-keyring-wkd-sync.timer <<<"$UPDATE_UNITS")" = 1
u=${IMDS_UNITS%% *}; : >"$M/unmasked-$u"
check "one imds unit enabled: named" has "$u is enabled, not masked" imds_check
rm -f -- "$M/etc/systemd/system-generators/$IMDS_GENERATOR"
check "the generator unmasked: named" has "$IMDS_GENERATOR is not masked" imds_check
rm -f -- "$M/etc/systemd/system/pacman-init.service"; : >"$M/etc/systemd/system/pacman-init.service"
check "pacman-init a file of its own, not a mask: named" has "pacman-init.service is not a mask" updates_check

# ---- 11. The update stamps: each names /usr's time to the nanosecond, as
# systemd-update-done writes it; and seal_final writes them after the last change to /usr,
# before the final probe, which reads them.
eval "$(steps updated_check | sed -e "s|stat -c %.9Y /usr|stat -c %.9Y \$Y/usr|" -e 's|"$d/.updated"|"$Y$d/.updated"|')" || exit 1
Y=$T/upd; mkdir -p "$Y/usr" "$Y/etc" "$Y/var"; touch -d '2026-11-16 12:00:00.123456789' "$Y/usr"
stamp() { printf '# This file was created by systemd-update-done.\nTIMESTAMP_NSEC=%s\n' "$2" >"$Y$1/.updated"; }
ns=$(stat -c %.9Y "$Y/usr" | tr -d .)
check "/usr's time is read to the nanosecond" test "${ns: -9}" = 123456789
stamp /etc "$ns"; stamp /var "$ns"
check "both stamps at /usr's time: no finding" test -z "$(updated_check)"
touch -d '2026-11-16 12:00:01' "$Y/usr"
check "/usr touched after (a pacman hook): both named" test "$(updated_check | grep -c 'not /usr.s')" = 2
stamp /etc "$(stat -c %.9Y "$Y/usr" | tr -d .)"; stamp /var "$(stat -c %.9Y "$Y/usr" | tr -d .)"; rm -f -- "$Y/etc/.updated"
check "an absent stamp: named" test "$(updated_check)" = "/etc/.updated names no time, not /usr's $(stat -c %.9Y "$Y/usr" | tr -d .)"
SF=$(fn "$STEPS" seal_final)
at() { grep -n -F -- "$1" <<<"$SF" | head -1 | cut -d: -f1; }
check "seal_final marks the update done after removing the harness and before the final probe" \
    test -n "$(at '/usr/lib/systemd/systemd-update-done')" -a "$(at 'rm -rf -- "$BUILD" /usr/local/sbin/template-steps')" -lt "$(at 'ldconfig -X')" \
         -a "$(at 'ldconfig -X')" -lt "$(at '/usr/lib/systemd/systemd-update-done')" -a "$(at '/usr/lib/systemd/systemd-update-done')" -lt "$(at 'if ! probes final')"
check "and the final probe reads the stamps" \
    eval 'grep -q "check \"the first boot.s update armed: a stamp not at /usr.s time\" \"\$(updated_check 2>&1)\"" <<<"$(fn "$STEPS" probes)"'

# ---- 12. What the appliance does not use, masked (the Planner's rule of 2026-11-16), and
# userdb's SSH key command gone; the lists name each unit the Planner named.
eval "$(sed -n '/^UNUSED_UNITS=/,/"$/p' "$STEPS")"; const USERDB_SSH_LINK; const USERDB_TMPFILES
for u in systemd-sysext.service systemd-confext.service systemd-homed.service systemd-tpm2-clear.service \
         systemd-sysupdate-notify-sysext.socket systemd-report.socket systemd-report-basic.socket systemd-userdbd.socket \
         systemd-journalctl-metrics.socket systemd-mountfsd.socket systemd-nsresourced.socket machines.target \
         systemd-boot-update.service systemd-boot-clear-sysfail.service; do
    check "UNUSED_UNITS names $u" grep -qw -- "$u" <<<"$UNUSED_UNITS"
done
for u in systemd-pstore.service systemd-coredump.socket systemd-coredumpd.service; do
    check "UNUSED_UNITS leaves $u, which the Planner keeps" eval '! grep -qw -- "$u" <<<"$UNUSED_UNITS"'
done
check "and the sysupdate daemon and its socket are among the update units" \
    eval 'grep -qw systemd-sysupdated.service <<<"$UPDATE_UNITS" && grep -qw systemd-sysupdate.socket <<<"$UPDATE_UNITS"'
eval "$(steps masks_check userdb_ssh_check)" || exit 1
systemctl() { [ "$1" = is-enabled ] && case $2 in *homed*) echo enabled ;; *gone*) return 1 ;; *) echo masked ;; esac; }
check "masks_check: every unit masked, no line" test -z "$(masks_check systemd-sysext.service systemd-userdbd.socket)"
check "one enabled, one unknown: each named" \
    test "$(masks_check systemd-homed.service x-gone.service | tr '\n' '|')" = "systemd-homed.service is enabled, not masked|x-gone.service is unknown, not masked|"
unset -f systemctl
eval "$(steps userdb_ssh_check | sed -e "s|\"\$USERDB_SSH_LINK\"|\"\$UD\$USERDB_SSH_LINK\"|g" -e "s|\"\$USERDB_TMPFILES\"|\"\$UD\$USERDB_TMPFILES\"|g")" || exit 1
UD=$T/userdb; mkdir -p "$UD/etc/ssh/sshd_config.d" "$UD/etc/tmpfiles.d"
ln -s /dev/null "$UD$USERDB_TMPFILES"
check "the link gone and the tmpfiles line masked: no line" test -z "$(userdb_ssh_check)"
ln -s ../../../usr/lib/systemd/sshd_config.d/20-systemd-userdb.conf "$UD$USERDB_SSH_LINK"
check "the link left, though it leads nowhere here: named" eval 'grep -q "20-systemd-userdb.conf stands" <<<"$(userdb_ssh_check)"'
rm -f -- "$UD$USERDB_SSH_LINK" "$UD$USERDB_TMPFILES"
check "the tmpfiles line unmasked: named, since a boot lays the link again" eval 'grep -q "is not a mask, so a boot lays" <<<"$(userdb_ssh_check)"'

# ---- 13. sudo as WeaverAgent's create-agent.sh asks it (#581): the include read with its
# own pattern, visudo, nobody, and a command as nobody, that last in the pre pass alone.
SU=$T/sudo; mkdir -p "$SU"
# Its root prefixes dropped by name (the common one would take visudo's own letters too).
eval "$(fn "$STEPS" sudoers_check | sed -e "s|/usr/bin/sudo |sudo_stub |g" -e "s|${P}grep |grep |; s|${P}visudo |visudo |" -e "s| /etc/sudoers | $SU/sudoers |")" || exit 1
check "the include pattern is create-agent.sh's own" \
    grep -qF "grep -qE '^[#@]includedir[[:space:]]+/etc/sudoers\.d([[:space:]]|\$)' /etc/sudoers" <<<"$(fn "$STEPS" sudoers_check)"
visudo() { [ ! -e "$SU/visudo-refuses" ]; }
sudo_stub() { echo "$*" >>"$SU/calls"; [ ! -e "$SU/sudo-refuses" ]; }
printf 'root ALL=(ALL:ALL) ALL\n@includedir /etc/sudoers.d\n' >"$SU/sudoers"
check "Arch's sudoers, with its @includedir: no line in the final pass, and no sudo run" \
    eval '[ -z "$(sudoers_check final)" ] && [ ! -e "$SU/calls" ]'
check "and in the pre pass, sudo run as nobody and root's rule read, no line" \
    eval '[ -z "$(sudoers_check pre)" ] && grep -qx -- "-n -u nobody true" "$SU/calls" && grep -qx -- "-n -l -U root -u nobody /usr/bin/true" "$SU/calls"'
printf 'root ALL=(ALL:ALL) ALL\n#includedir /etc/sudoers.d\n' >"$SU/sudoers"
check "the older #includedir form counts, as create-agent.sh reads it" test -z "$(sudoers_check final)"
printf 'root ALL=(ALL:ALL) ALL\n## @includedir /etc/sudoers.d\n' >"$SU/sudoers"
check "a commented include: named" test "$(sudoers_check final)" = "/etc/sudoers does not include /etc/sudoers.d"
printf '@includedir /etc/sudoers.d\n' >"$SU/sudoers"; : >"$SU/visudo-refuses"; : >"$SU/sudo-refuses"
check "visudo refusing, and sudo refusing both asks in the pre pass: each named" \
    test "$(sudoers_check pre | wc -l)" = 3

# ---- 14. The facts step refuses where the seed's host guard is not loaded.
GL=$(fn "$STEPS" facts | sed -n "/^    local g; g=\$(sudo nft list table inet hostguard/,/^    grep -q 'ip daddr/p" | sed -e "s|$P||g" -e 's|^    local g; |    |' -e 's|return 1|exit 1|')
guard() { bash -c "nft() { $1; }; $GL" >/dev/null 2>&1; }
check "the gate is in the facts step" test "$(wc -l <<<"$GL")" = 2
check "a guard table as the seed loads it: passes" guard 'printf "table inet hostguard {\n chain out {\n  ip daddr 10.0.2.2 counter packets 0 bytes 0 drop\n }\n}\n"'
check "no table: refused" eval '! guard "echo Error: No such file or directory >&2; return 1"'

# ---- 15. Crashes: a journal record, no dump (the Planner's ruling of 2026-11-17).
# coredump_check against a stand-in drop-in and a stand-in systemd-analyze that prints the
# files as cat-config does, the main file's commented defaults first.
eval "$(sed -n "/^COREDUMP_DROPIN=/,/^ProcessSizeMax=0'\$/p" "$STEPS")"
check "the settings: Storage=none and ProcessSizeMax=0 under [Coredump]" \
    test "$(grep -cxE '\[Coredump\]|Storage=none|ProcessSizeMax=0' <<<"$COREDUMP_SETTINGS")" = 3
CD=$T/coredump; mkdir -p "$CD"
eval "$(steps coredump_check | sed "s|\"\$COREDUMP_DROPIN\"|\"\$CD/dropin\"|")" || exit 1
printf '%s\n' "$COREDUMP_SETTINGS" >"$CD/dropin"
systemd-analyze() { [ "$1 $2" = "cat-config systemd/coredump.conf" ] || return 9
    printf '# /etc/systemd/coredump.conf\n[Coredump]\n#Storage=external\n#ProcessSizeMax=32G\n\n'
    [ -e "$CD/later" ] && cat "$CD/later"
    printf '# %s\n%s\n' "$COREDUMP_DROPIN" "$(cat "$CD/dropin")"
    [ -e "$CD/after" ] && cat "$CD/after"; return 0; }
check "the drop-in as the image writes it, read last: no line" test -z "$(coredump_check)"
printf '[Coredump]\nStorage = external\n' >"$CD/after"
check "a later file that stores dumps wins, and is named by what it sets" \
    test "$(coredump_check)" = "coredump reads Storage=external, not Storage=none"
rm -f -- "$CD/after"; printf '[Coredump]\nStorage=external\nProcessSizeMax=2G\n' >"$CD/later"
check "an earlier file that stores dumps: the drop-in decides, no line" test -z "$(coredump_check)"
printf '[Coredump]\nStorage=none\n' >"$CD/dropin"
check "a drop-in without ProcessSizeMax=0: not the image's, and the core is written while processed" \
    test "$(coredump_check | tr '\n' '|')" = "$COREDUMP_DROPIN is not the image's settings|coredump reads ProcessSizeMax=2G, not ProcessSizeMax=0|"
systemd-analyze() { echo "Failed to connect"; return 1; }
check "a cat-config that does not run is a finding, never a pass" \
    eval 'grep -q "systemd-analyze cat-config did not run (Failed to connect)" <<<"$(coredump_check)"'
unset -f systemd-analyze
finish
