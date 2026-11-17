#!/usr/bin/env bash
# The seal's hostname probe, template-steps' image_hostname_check, on stand-in /etc trees:
# the function's text with its root prefix dropped and /etc pointed at a fresh directory
# for each case. The build name is matched whole (the Planner's review of 2026-11-15, B2),
# against the image's real nftables.conf text among others. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
STEPS=$CODE/guest/template-steps
P='s''udo '
# The function's text, its root prefix dropped. Each case points its /etc at that case's
# own tree, literally, since its patterns sit in single quotes.
body=$(fn "$STEPS" image_hostname_check | sed -e "s|$P||g")
check "the function is in the file, its root prefix dropped" eval '[ -n "$body" ] && ! grep -qF -- "$P" <<<"$body"'
eval "$(sed -n '/^IMAGE_HOSTNAME=/p' "$STEPS")"
eval "$(awk '/^FIREWALL_RULES=/ { f = 1 } f { print } f && /^}'"'"'$/ { exit }' "$STEPS")"
check "the image's firewall text is in the file, and names the build Spec" grep -q 'weaver-appliance-build-Spec' <<<"$FIREWALL_RULES"
BUILD_NAME=weaver-appliance-build
hostname() { echo "$BUILD_NAME"; }
fresh() {
    ETC=$(mktemp -d "$T/etc.XXXXXX"); RECORD=$ETC/weaver-appliance
    eval "$(sed "s|/etc|$ETC|g" <<<"$body")"
    mkdir -p "$ETC/ssh" "$ETC/sudoers.d" "$ETC/pacman.d/gnupg"
    echo "$IMAGE_HOSTNAME" >"$ETC/hostname"; echo "127.0.0.1 localhost" >"$ETC/hosts"
    printf '%s\n' "$FIREWALL_RULES" >"$ETC/nftables.conf"
}
fresh
check "the image's hostname, and the default build name only inside weaver-appliance-build-Spec: no finding" \
    test -z "$(image_hostname_check final)"
echo "$BUILD_NAME" >"$ETC/hostname"
check "the build's hostname left in /etc/hostname: a finding" test -n "$(image_hostname_check final)"
fresh; echo "127.0.1.1 $BUILD_NAME" >>"$ETC/hosts"
check "the build's name in /etc/hosts: a finding naming the file" test "$(image_hostname_check final)" = "$ETC/hosts"
fresh; echo "127.0.1.1 $BUILD_NAME.lan $BUILD_NAME" >"$ETC/hosts"; echo "x root@$BUILD_NAME" >"$ETC/motd"
check "the name before a dot, or after an @: findings" test "$(image_hostname_check final | wc -l)" = 2
fresh; echo "ssh-ed25519 AAAA root@$BUILD_NAME" >"$ETC/ssh/ssh_host_ed25519_key.pub"
echo "# made for $BUILD_NAME" >"$ETC/sudoers.d/90-cloud-init-users"; echo "uid $BUILD_NAME" >"$ETC/pacman.d/gnupg/pubring.kbx"
check "in the pre pass, the files the final step removes are left out" test -z "$(image_hostname_check pre)"
check "and in the final pass they are findings" test "$(image_hostname_check final | wc -l)" = 3
fresh; BUILD_NAME=$IMAGE_HOSTNAME
check "a build guest named as the image is a finding, not a pass" test -n "$(image_hostname_check final)"
fresh; BUILD_NAME=weaver-appliance-build5; echo "weaver-appliance-build" >"$ETC/mailname"
check "a shorter name the build's begins with is not the build's name" test -z "$(image_hostname_check final)"
fresh; BUILD_NAME=a.b; echo "axb" >"$ETC/mailname"
check "a dot in the build name matches only a dot" test -z "$(image_hostname_check final)"
echo "a.b" >"$ETC/mailname"
check "and the name itself is found" test "$(image_hostname_check final)" = "$ETC/mailname"
fresh; BUILD_NAME=weaver-appliance-build; mkdir -p "$RECORD"; echo "HIT  /etc/hostname is '$BUILD_NAME'" >"$RECORD/seal-probes"
check "the final pass's own record, written while it runs, is left out" test -z "$(image_hostname_check final)"
echo "x $BUILD_NAME" >"$RECORD/stamp"
check "but the stamp beside it is not" test "$(image_hostname_check final)" = "$RECORD/stamp"

# Where the hostname is written, against where it is judged: every probes pre the recipe
# reaches comes after the write. The proving build of 2026-11-16 stopped at the recipe's
# probes step, which ran before the seal that then wrote the hostname.
cleanup=$(fn "$STEPS" cleanup); seal=$(fn "$STEPS" seal)
code=$(grep -vE '^[[:space:]]*#' "$STEPS")
check "the cleanup writes the image's hostname to /etc/hostname" \
    grep -qF "printf '%s\n' \"\$IMAGE_HOSTNAME\" | sudo tee /etc/hostname >/dev/null || return 1" <<<"$cleanup"
# A write is a tee, a redirection, a copy, a move or an install onto the file, or
# hostnamectl; every other line that names the file reads it.
writes=$(grep -E '(tee|>|cp|mv|install)[^|;]* /etc/hostname|hostnamectl' <<<"$code")
check "and nothing else in the guest steps writes /etc/hostname" \
    test "$(grep -c . <<<"$writes")" = 1 -a "$(grep -cxF -- "$writes" <<<"$cleanup")" = 1
check "probes pre is reached only from the seal and the probes step" \
    test "$(grep -cE '(^|[^a-z_])probes pre' <<<"$code")" = 2 -a "$(grep -cxF '    probes pre || return 1' <<<"$seal")" = 1 \
         -a "$(grep -cxF 'probes) probes pre ;;' <<<"$code")" = 1
# The recipe (COMMANDS.md, the appliance image's block): the step loop that ends in the
# cleanup and the probes step, then the seal.
loop=$(grep -n -E '^ *for s in .* cleanup .*; do$' "$CODE/../COMMANDS.md" | tail -1)
seal_at=$(grep -n -E '^ *code/build-template\.sh seal \|\| exit$' "$CODE/../COMMANDS.md" | tail -1 | cut -d: -f1)
order=$(sed -E 's/.*for s in (.*); do$/\1/' <<<"${loop#*:}" | tr ' ' '\n' | grep -nxE 'cleanup|probes' | cut -d: -f2 | tr '\n' ' ')
check "the recipe runs the cleanup, then its probes step, then the seal" \
    test "$order" = "cleanup probes " -a -n "$seal_at" -a "${seal_at:-0}" -gt "${loop%%:*}"
# And the probe on the tree the cleanup leaves, the build guest still running under its
# own name: no finding in the pre pass.
fresh
check "the tree as the cleanup leaves it passes the pre pass" test -z "$(image_hostname_check pre)"
finish
