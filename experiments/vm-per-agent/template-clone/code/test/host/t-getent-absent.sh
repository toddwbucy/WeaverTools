#!/usr/bin/env bash
# getent_absent, as guest/template-steps defines it, against a stand-in getent: only
# getent's status 2 (not found) is an absence. Status 0 is an entry found, and any other
# status is a read that did not happen; both are a sentence and status 1. The two sites
# that judge an account gone, os_probes and the seal's final probe, read it. Pure logic:
# no root, no guest.
source "$(dirname "$0")/lib-test.sh"
TS=$CODE/guest/template-steps
eval "$(fn "$TS" getent_absent)" || exit 1
# A stand-in getent: its status is the one GS names.
getent() { return "$GS"; }

GS=2; out=$(getent_absent passwd x "the probe account"); rc=$?
check "status 2: absent, status 0 and nothing printed" eval '[ "$rc" = 0 ] && [ -z "$out" ]'
GS=0; out=$(getent_absent passwd x "the probe account"); rc=$?
check "status 0: found, status 1, named" eval '[ "$rc" = 1 ] && [ "$out" = "the probe account remains" ]'
for GS in 1 3; do
    out=$(getent_absent group x "the probe group"); rc=$?
    check "status $GS: a read that did not happen, status 1, not an absence" \
        eval '[ "$rc" = 1 ] && [ "$out" = "getent exited $GS, so whether the probe group remains is not known" ]'
done
unset -f getent

# The real getent, where the host has one: the account running the test is found, and an
# account no system has is not.
if command -v getent >/dev/null 2>&1; then
    out=$(getent_absent passwd "$(id -un)" "this account"); rc=$?
    check "the real getent: this account remains" eval '[ "$rc" = 1 ] && [ "$out" = "this account remains" ]'
    out=$(getent_absent passwd weaver-no-such-account-x9 "no such account"); rc=$?
    check "the real getent: an account no system has is absent" eval '[ "$rc" = 0 ] && [ -z "$out" ]'
fi

# The sites: os_probes judges its probe account and group gone, and the final probe the
# build account and its group, through getent_absent and no other getent read.
OSP=$(fn "$TS" os_probes) || exit 1
PROBES=$(fn "$TS" probes) || exit 1
check "os_probes reads the probe account and group through getent_absent" \
    bash -c 'grep -qF "getent_absent passwd \"\$u\"" <<<"$1" && grep -qF "getent_absent group \"\$g\"" <<<"$1"' _ "$OSP"
check "os_probes has no other getent read" bash -c '! grep -E "(^|[^_])getent " <<<"$1" | grep -v getent_absent | grep -q .' _ "$OSP"
check "the final probe reads the build account and its group through getent_absent" \
    bash -c 'grep -qF "getent_absent passwd op" <<<"$1" && grep -qF "getent_absent group op" <<<"$1"' _ "$PROBES"
check "the probes have no other getent read" bash -c '! grep -E "(^|[^_])getent " <<<"$1" | grep -v getent_absent | grep -q .' _ "$PROBES"
finish
