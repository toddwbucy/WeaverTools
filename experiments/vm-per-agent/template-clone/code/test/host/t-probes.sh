#!/usr/bin/env bash
# Three of the seal's probes, from template-steps' own lines: the machine identity (it must
# read uninitialized, the Planner's review of 2026-11-15, F8), the authorized keys, run in
# both passes with the build account's own left out of the pre pass (F9), and cloud-init's
# state and configuration gone (the Planner's ruling of 2026-11-16: cloud-init removed
# outright at the seal). Each line's root prefix is
# dropped and its paths pointed at a fresh stand-in tree for each case. Pure logic: no
# root, no guest.
source "$(dirname "$0")/lib-test.sh"
STEPS=$CODE/guest/template-steps
P='s''udo '
probes=$(fn "$STEPS" probes)
mid=$(grep -F 'check "machine identity"' <<<"$probes")
ak=$(grep -F 'check "an authorized key"' <<<"$probes")
ci=$(grep -F "check \"cloud-init's state and configuration\"" <<<"$probes")
check "each probe has one line" test "$(grep -c . <<<"$mid")" = 1 -a "$(grep -c . <<<"$ak")" = 1 -a "$(grep -c . <<<"$ci")" = 1
# The authorized-key line stands before the final pass's block, so both passes run it.
check "the authorized-key probe runs in both passes" \
    test "$(grep -n -F 'check "an authorized key"' <<<"$probes" | cut -d: -f1)" -lt "$(grep -n -F 'if [ "$mode" = final ]; then' <<<"$probes" | cut -d: -f1)"
check "and the machine identity in the final pass" \
    test "$(grep -n -F 'check "machine identity"' <<<"$probes" | cut -d: -f1)" -gt "$(grep -n -F 'if [ "$mode" = final ]; then' <<<"$probes" | cut -d: -f1)"
check "and cloud-init's state in the final pass" \
    test "$(grep -n -F "check \"cloud-init's state and configuration\"" <<<"$probes" | cut -d: -f1)" -gt "$(grep -n -F 'if [ "$mode" = final ]; then' <<<"$probes" | cut -d: -f1)"
# Each line's root prefix dropped, and its paths pointed at the case's own tree,
# literally, since some sit in single quotes.
point() { sed -e "s|$P||g" -e "s|/etc/|$R/etc/|g" -e "s|/var/|$R/var/|g" -e "s| /etc/cloud| $R/etc/cloud|g" -e "s|/root |$R/root |g" -e "s|/home |$R/home |g" -e "s|/home/op/|$R/home/op/|g" <<<"$1"; }
define() {
    eval "probe_mid() { local mode=\$1; $(point "$mid")
}"
    eval "probe_ak() { local mode=\$1; $(point "$ak")
}"
    eval "probe_ci() { $(point "$ci")
}"
}
hits=""
check() { hits+="$1|$2"$'\n'; }   # the probe's own check, recorded
got() { hits=""; "$@"; cut -d'|' -f2- <<<"$hits" | grep -v '^$' || true; }
ok_t() { local w=$1; shift; if "$@"; then ok "$w"; else no "$w"; fi; }
fresh() { R=$(mktemp -d "$T/root.XXXXXX"); mkdir -p "$R/etc/ssh" "$R/root/.ssh" "$R/home/op/.ssh" "$R/var/lib/systemd"; define; }
fresh; echo uninitialized >"$R/etc/machine-id"
ok_t "machine-id uninitialized: no hit" test -z "$(got probe_mid final)"
: >"$R/etc/machine-id"
ok_t "an empty machine-id is a hit" eval '[ "$(got probe_mid final)" = "machine-id is '"''"', not uninitialized" ]'
rm -f -- "$R/etc/machine-id"
ok_t "an absent machine-id is a hit" test -n "$(got probe_mid final)"
echo 0123456789abcdef0123456789abcdef >"$R/etc/machine-id"
ok_t "a machine's own id is a hit" eval 'grep -q "machine-id is .0123456789abcdef0123456789abcdef., not uninitialized" <<<"$(got probe_mid final)"'
echo uninitialized >"$R/etc/machine-id"; : >"$R/etc/ssh/ssh_host_ed25519_key"
ok_t "a host key left is a hit" eval 'grep -q "ssh_host_ed25519_key" <<<"$(got probe_mid final)"'
fresh; : >"$R/root/.ssh/authorized_keys"
ok_t "an empty authorized_keys for root: no hit in either pass" test -z "$(got probe_ak pre)$(got probe_ak final)"
echo "ssh-ed25519 AAAA op" >"$R/home/op/.ssh/authorized_keys"
ok_t "the build account's key: no hit in the pre pass" test -z "$(got probe_ak pre)"
ok_t "but a hit in the final pass" eval 'grep -q "/home/op/.ssh/authorized_keys" <<<"$(got probe_ak final)"'
echo "ssh-ed25519 AAAA x" >"$R/root/.ssh/authorized_keys"
ok_t "root's key: a hit in the pre pass, before the removals" eval 'grep -q "/root/.ssh/authorized_keys" <<<"$(got probe_ak pre)"'
fresh; echo "ssh-ed25519 AAAA x" >"$R/etc/ssh/authorized_keys2"
ok_t "an authorized_keys2 under /etc/ssh: a hit in the pre pass" eval 'grep -q "authorized_keys2" <<<"$(got probe_ak pre)"'
fresh
ok_t "no cloud-init state, configuration or log: no hit" test -z "$(got probe_ci)"
mkdir -p "$R/etc/cloud"
ok_t "/etc/cloud left: a hit naming it" eval 'grep -q "$R/etc/cloud" <<<"$(got probe_ci)"'
fresh; mkdir -p "$R/var/lib/cloud/instance"
ok_t "/var/lib/cloud left: a hit naming it" eval 'grep -q "$R/var/lib/cloud" <<<"$(got probe_ci)"'
fresh; mkdir -p "$R/var/log"; : >"$R/var/log/cloud-init.log"
ok_t "cloud-init's log left: a hit naming it" eval 'grep -q "cloud-init.log" <<<"$(got probe_ci)"'
finish
