#!/usr/bin/env bash
# The sealed image's first login (the operator's rulings of 2026-11-16: root, set at first
# login; systemd's first-boot paths left and named), from guest/template-steps' own text:
# accounts_judge on stand-in account files, sshd_check against this host's sshd with a
# stand-in configuration, firstboot_paths_check against a stand-in tree and a stand-in
# systemctl, and where each is wired: the cleanup writes the SSH settings and checks them,
# the seal's final step sets root's two fields before its final probe, and the probes run
# each check in both passes. Each function's root prefix is dropped and its paths pointed
# at the case's own tree. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need sshd ssh-keygen
TS=$CODE/guest/template-steps
P='s''udo '
eval "$(fn "$TS" accounts_judge)" || exit 1

# 1. accounts_judge. A stand-in set of account files, sealed as the final step leaves them:
# root's password field empty and its last change 0, every other account locked, no login
# shell but root's, no hash anywhere.
A=$T/acc; mkdir -p "$A"
printf '# /etc/shells: valid login shells\n/bin/sh\n/bin/bash\n/usr/bin/bash\n' >"$A/shells"
sealed() {
    printf 'root:x:0:0:root:/root:/bin/bash\ndaemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin\nsync:x:4:65534:sync:/bin:/bin/sync\n' >"$A/passwd"
    printf 'root::0:0:99999:7:::\ndaemon:*:20742:0:99999:7:::\nsync:!*:20742:0:99999:7:::\n' >"$A/shadow"
    printf 'root:*::\nadm:!::syslog\n' >"$A/gshadow"
    printf 'root:*:20742:0:99999:7:::\ndaemon:*:20742:0:99999:7:::\n' >"$A/shadow-"
    rm -f -- "$A/gshadow-"
}
judge() { accounts_judge "$1" "$A/passwd" "$A/shadow" "$A/gshadow" "$A/shells" "$A/shadow-" "$A/gshadow-" 2>&1; }
sealed
check "the sealed state: no finding, and a backup the tools never wrote is none" test -z "$(judge final)"
sed -i 's/^root::0:/root:$y$j9T$salt$hash:20773:/' "$A/shadow"
out=$(judge final)
check "root with a hash and a last change: the hash and both fields named" \
    test "$(grep -c "^root: a password hash in $A/shadow\$" <<<"$out")" = 1 \
         -a "$(grep -cx 'root: the password field is not empty and the last change 20773, not empty and 0' <<<"$out")" = 1
sealed; sed -i 's/^root::0:/root::20773:/' "$A/shadow"
check "root's field emptied but the change not required: a finding" \
    test "$(judge final)" = "root: the password field is empty and the last change 20773, not empty and 0"
sealed; sed -i 's/^daemon:\*:/daemon::/' "$A/shadow"
check "another account with an empty password field: not locked" test "$(judge final)" = "daemon: not locked"
sealed; sed -i 's/^daemon:\*:/daemon:!$y$j9T$salt$hash:/' "$A/shadow"
check "a locked hash is still a hash the image would ship" test "$(judge final)" = "daemon: a password hash in $A/shadow"
sealed; printf 'svc:x:999:999::/var/lib/svc:/usr/bin/bash\n' >>"$A/passwd"; printf 'svc:!:20742::::::\n' >>"$A/shadow"
check "another account with a login shell: named, with its shell" test "$(judge final)" = "svc: the login shell /usr/bin/bash"
check "/bin/sync is no login shell: sync is no finding" bash -c '! grep -q "^sync" <<<"$1"' _ "$(judge final)"
sealed; sed -i 's/^daemon:x:/daemon::/' "$A/passwd"
check "a password field in passwd other than x: a finding" test "$(judge final)" = "daemon: the password field of passwd is not x"
sealed; sed -i 's|^root:x:0:0:root:/root:/bin/bash|root:x:0:0:root:/root:/usr/sbin/nologin|' "$A/passwd"
check "root without a login shell: the console login would fail, a finding" test "$(judge final)" = "root: the shell /usr/sbin/nologin is not a login shell"
sealed; printf 'sudo:$y$j9T$salt$hash::\n' >>"$A/gshadow"; printf 'root:$y$j9T$salt$hash:20773:0:99999:7:::\n' >"$A/shadow-"; printf 'adm:x::\n' >"$A/gshadow-"
out=$(judge final)
check "a hash in gshadow or in a backup is named with its file; a backup without one is not" \
    test "$(grep -cx "sudo: a password hash in $A/gshadow" <<<"$out")" = 1 -a "$(grep -cx "root: a password hash in $A/shadow-" <<<"$out")" = 1 \
         -a "$(grep -c . <<<"$out")" = 2
# Before the seal: the build account is there with its shell, and root is locked as
# shipped. The pre pass leaves out both; it still finds any other account that can log in.
sealed; printf 'op:x:1000:1000::/home/op:/bin/bash\n' >>"$A/passwd"; printf 'op:!:20773:0:99999:7:::\n' >>"$A/shadow"
sed -i 's/^root::0:/root:*:20742:/' "$A/shadow"
check "the pre pass: the build account and root as shipped, no finding" test -z "$(judge pre)"
out=$(judge final)
check "the final pass on the same files: the build account's shell and root's fields are findings" \
    test "$(grep -cx 'op: the login shell /bin/bash' <<<"$out")" = 1 -a "$(grep -c '^root: the password field is not empty' <<<"$out")" = 1
sed -i 's/^daemon:\*:/daemon::/' "$A/shadow"
check "the pre pass still finds an account that is not locked" test "$(judge pre)" = "daemon: not locked"

# 2. sshd_check against this host's sshd. A stand-in sshd_config that includes the
# drop-in directory first and then names the opposite of each setting, as Ubuntu's order
# is; the drop-in as the image writes it; and a later file naming the opposite again.
S=$T/ssh; mkdir -p "$S/conf.d" "$T/run"
eval "$(sed -n "/^SSHD_DROPIN=/,/^PermitRootLogin prohibit-password'\$/p" "$TS")"
check "the settings name the four, no password and root by key alone" \
    test "$(grep -cxE 'PasswordAuthentication no|KbdInteractiveAuthentication no|PermitEmptyPasswords no|PermitRootLogin prohibit-password' <<<"$SSHD_SETTINGS")" = 4
SSHD_DROPIN=$S/conf.d/10-weaver-appliance.conf
printf 'Include %s/conf.d/*.conf\nPasswordAuthentication yes\nKbdInteractiveAuthentication yes\nPermitEmptyPasswords yes\nPermitRootLogin yes\nUsePAM yes\n' "$S" >"$S/sshd_config"
printf '%s\n' "$SSHD_SETTINGS" >"$SSHD_DROPIN"
printf 'PasswordAuthentication yes\nPermitRootLogin yes\n' >"$S/conf.d/90-later.conf"
sc=$(fn "$TS" sshd_check | sed -e "s|$P||g" -e "s|/run/weaver-sshd-check|$T/run/weaver-sshd-check|g" -e "s|t=\$(sshd -T |t=\$(sshd -f $S/sshd_config -T |")
eval "$sc"
check "the stand-in configuration is the one sshd reads here, not this host's" grep -qF "t=\$(sshd -f $S/sshd_config -T " <<<"$sc"
out=$(sshd_check 2>&1); rc=$?
check "the drop-in first by name decides all four, over the main file and a later file" test "$rc" = 0 -a -z "$out"
check "and its throwaway host key is gone after" test -z "$(ls -A "$T/run")"
printf 'PasswordAuthentication yes\n' >"$S/conf.d/05-earlier.conf"
out=$(sshd_check 2>&1); rc=$?
check "an earlier file that names a password wins, and sshd's own reading finds it, though the drop-in is exact" \
    test "$rc" = 1 -a "$(grep -ci "sshd -T reads 'PasswordAuthentication yes', not 'passwordauthentication no'" <<<"$out")" = 1
rm -f -- "$S/conf.d/05-earlier.conf"; echo 'X11Forwarding no' >>"$SSHD_DROPIN"
out=$(sshd_check 2>&1); rc=$?
check "a drop-in with other text than the image's: a finding" test "$rc" = 1 -a "$(grep -c "$SSHD_DROPIN is not the image's settings" <<<"$out")" = 1
printf '%s\n' "$SSHD_SETTINGS" >"$SSHD_DROPIN"
eval "$(sed 's|t=\$(sshd -f [^ ]* -T |t=$(sshd_stub -T |' <<<"$sc")"
sshd_stub() { echo "sshd: no hostkeys available -- exiting."; return 1; }
out=$(sshd_check 2>&1); rc=$?
check "an sshd -T that does not run is a finding, never a pass" \
    test "$rc" = 1 -a "$(grep -cF 'sshd -T did not run (sshd: no hostkeys available -- exiting.)' <<<"$out")" = 1 -a -z "$(ls -A "$T/run")"

# 3. firstboot_paths_check against a stand-in tree, with a stand-in systemctl that reads
# each unit's state from the tree.
F=$T/fb
eval "$(fn "$TS" firstboot_paths_check | sed -e "s|/usr/lib/|$F/usr/lib/|g" -e "s|/etc/|$F/etc/|g")"
systemctl() { [ "$1" = is-enabled ] && cat "$F/state-$2" 2>/dev/null; }
shipped() {
    rm -rf -- "${F:?}"; mkdir -p "$F/usr/lib/tmpfiles.d" "$F/usr/lib/systemd/system-generators" "$F/etc/tmpfiles.d" "$F/etc/systemd/system-generators"
    printf 'f^ /root/.ssh/authorized_keys 0600 root root - ssh.authorized_keys.root\n' >"$F/usr/lib/tmpfiles.d/provision.conf"
    printf '#!/bin/sh\n' >"$F/usr/lib/systemd/system-generators/systemd-ssh-generator"; chmod 0755 "$F/usr/lib/systemd/system-generators/systemd-ssh-generator"
    ln -s /dev/null "$F/etc/systemd/system-generators/systemd-gpt-auto-generator"
    echo static >"$F/state-systemd-firstboot.service"; echo static >"$F/state-systemd-creds.socket"
}
shipped
check "as Ubuntu 26.04 ships them, with another generator masked: no finding" test -z "$(firstboot_paths_check)"
echo masked >"$F/state-systemd-firstboot.service"
check "systemd-firstboot masked: named" test "$(firstboot_paths_check)" = "systemd-firstboot.service is masked"
shipped; rm -f -- "$F/state-systemd-creds.socket"
check "systemd-creds.socket not found: named" test "$(firstboot_paths_check)" = "systemd-creds.socket is not found"
shipped; ln -s /dev/null "$F/etc/systemd/system-generators/systemd-ssh-generator"
check "the SSH generator masked by a link in /etc: named" test "$(firstboot_paths_check)" = "systemd-ssh-generator is masked"
shipped; rm -f -- "$F/usr/lib/systemd/system-generators/systemd-ssh-generator"
check "the SSH generator gone: named" test "$(firstboot_paths_check)" = "systemd-ssh-generator is not among the system generators"
shipped; ln -s /dev/null "$F/etc/tmpfiles.d/provision.conf"
check "provision.conf masked by a link in /etc: named" test "$(firstboot_paths_check)" = "$F/etc/tmpfiles.d/provision.conf stands over the shipped file"
shipped; : >"$F/usr/lib/tmpfiles.d/provision.conf"
check "provision.conf without root's key credential: named" \
    test "$(firstboot_paths_check)" = "$F/usr/lib/tmpfiles.d/provision.conf does not name ssh.authorized_keys.root"

# 4. Where each is wired.
probes=$(fn "$TS" probes); seal_final=$(fn "$TS" seal_final); cleanup=$(fn "$TS" cleanup)
final_at=$(grep -n -F 'if [ "$mode" = final ]; then' <<<"$probes" | cut -d: -f1)
at() { grep -n -F "$1" <<<"$probes" | head -1 | cut -d: -f1; }
check "the probes run the SSH settings, the first-boot paths and the accounts in both passes" \
    test -n "$(at 'check "SSH settings not as built" "$(sshd_check 2>&1)"')" -a "$(at 'check "SSH settings not as built"')" -lt "$final_at" \
         -a -n "$(at "check \"systemd's first-boot paths masked or missing\" \"\$(firstboot_paths_check 2>&1)\"")" -a "$(at "check \"systemd's first-boot paths")" -lt "$final_at" \
         -a -n "$(at 'check "an account that can log in, other than as sealed"')" -a "$(at 'check "an account that can log in')" -lt "$final_at"
# The probes' accounts line, run as written, its root prefix dropped and /etc pointed at a
# stand-in tree: the pass's mode reaches the judge, and the backups are read.
line=$(grep -F 'check "an account that can log in, other than as sealed"' <<<"$probes" | sed -e "s|$P||g" -e "s|/etc/|$A/etc/|g")
mkdir -p "$A/etc"; sealed; for f in passwd shadow gshadow shells shadow-; do cp "$A/$f" "$A/etc/$f"; done
# The line runs in a subshell with its own check, which prints what it was given.
probe_line() { ( mode=$1; check() { printf '%s|%s\n' "$1" "$2"; }; eval "$line" ); }
got_final=$(probe_line final)
sed -i 's/^root::0:/root:*:20742:/' "$A/etc/shadow"
got_pre=$(probe_line pre); got_final2=$(probe_line final)
check "the probes' accounts line: the sealed tree passes the final pass" test "$got_final" = "an account that can log in, other than as sealed|"
check "root as shipped passes the pre pass and is a hit in the final" \
    test "$got_pre" = "an account that can log in, other than as sealed|" -a "$(grep -c 'root: the password field is not empty' <<<"$got_final2")" = 1
pu=$(grep -n -F 'userdel -r op' <<<"$seal_final" | cut -d: -f1)
pd=$(grep -n -xF '    passwd -d root 2>&1; chage -d 0 root 2>&1' <<<"$seal_final" | cut -d: -f1)
pf=$(grep -n -F 'probes final' <<<"$seal_final" | head -1 | cut -d: -f1)
check "the final step empties root's password field and requires the change, after the build account goes and before the final probe" \
    test -n "$pd" -a -n "$pu" -a -n "$pf" && [ "$pu" -lt "$pd" ] && [ "$pd" -lt "$pf" ]
check "the cleanup writes the SSH settings to the drop-in and checks them as sshd reads them" \
    bash -c 'grep -qF "printf '"'"'%s\n'"'"' \"\$SSHD_SETTINGS\" | sudo tee \"\$SSHD_DROPIN\"" <<<"$1" && grep -qF "sudo sshd -t" <<<"$1" && grep -qxF "    sshd_check || return 1" <<<"$1"' _ "$cleanup"
finish
