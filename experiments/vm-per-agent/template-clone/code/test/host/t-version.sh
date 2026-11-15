#!/usr/bin/env bash
# VERSION, <release>.<respin> (decision 4 of 2026-11-15): the host's inputs_ok, proof's
# version block, the guest's facts regex and seal_final's pattern, each from its own text.
# And the guest's prelude, run with a stand-in os-release: VERSION must survive it (the
# Planner's review of 2026-11-15, B1). Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
BT=$CODE/build-template.sh STEPS=$CODE/guest/template-steps
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$BT" inputs_ok)"
vin() { ( set -euo pipefail; VERSION=$1 inputs_ok set ) >"$T/vin.out" 2>&1; }
vin_refused() { ! vin "$1" && grep -qF -- "$2" "$T/vin.out"; }
check "1.0 passes" vin 1.0
check "12.0 passes" vin 12.0
check "1.1 is a respin" vin_refused 1.1 'REFUSED: VERSION 1.1 is a respin'
check "10.20 is a respin" vin_refused 10.20 'REFUSED: VERSION 10.20 is a respin'
for v in 1 1.0.0 0.0 01.0 1.00 v1.0 ' 1.0' '1.0 '; do
    check "'$v' is out of form" vin_refused "$v" 'REFUSED: VERSION is <release>.<respin>'
done
check "create refuses without VERSION, and says so" eval '! o=$( ( set -euo pipefail; OS_GIVEN=26.04 DRIVER=615.71.09 DRIVER_ROUTE=ubuntu-prebuilt COMPUTE_CAP=86 WA_COMMIT=$(printf %040d 1) WW_COMMIT=$(printf %040d 2) inputs_ok all ) 2>&1) && grep -q "name VERSION" <<<"$o"'

# proof's version block, from 'local ver' to its tee.
BLOCK=$(sed -n '/^    local ver$/,/^    } 2>&1 | tee "\$out\/version.txt"$/p' "$BT")
check "proof's version block is in the file" test "$(wc -l <<<"$BLOCK")" -gt 3
eval "vblk() { $BLOCK
}"
out=$T/out; mkdir -p "$out"
vt() { printf '%b' "$1" >"$out/stamp"; VERSION=${2:-} vblk >/dev/null; }
vt 'sealed = x\nappliance.version = 1.0\n'
check "a stamp naming 1.0 passes" test "$(grep -c PROOF-FAIL "$out/version.txt")" = 0
vt 'appliance.version = 1.0\n' 1.0
check "and equals VERSION 1.0" test "$(grep -c PROOF-FAIL "$out/version.txt")" = 0
vt 'appliance.version = 1.0\n' 2.0
check "but not VERSION 2.0" grep -q 'PROOF-FAIL: the stamp names version 1.0, not VERSION 2.0' "$out/version.txt"
vt 'sealed = x\n'
check "no version fails" grep -q 'PROOF-FAIL: the stamp names no one version' "$out/version.txt"
vt 'appliance.version = 1.0\nappliance.version = 1.0\n'
check "two version lines fail" grep -q 'PROOF-FAIL: the stamp names no one version' "$out/version.txt"
vt 'appliance.version = one\n'
check "a version out of form fails" grep -q 'PROOF-FAIL: the stamp names no one version' "$out/version.txt"
rm -f -- "$out/stamp"; VERSION='' vblk >/dev/null
check "no stamp fails" grep -q 'PROOF-FAIL: the stamp names no one version' "$out/version.txt"

# The guest's two checks, as their lines hold them.
re=$(sed -n 's/^    \[\[ "\${VERSION:-}" =~ \(.*\) \]\] || { echo "VERSION .*/\1/p' "$STEPS")
check "the facts step's regex is in the file" test -n "$re"
for v in 1.0 3.0; do check "facts takes $v" eval "[[ '$v' =~ $re ]]"; done
for v in 1.1 0.0 1 ''; do check "facts refuses '$v'" eval "! [[ '$v' =~ $re ]]"; done
pat=$(sed -n "s/^    grep -qE '\(\^appliance.*\)' \"\$RECORD\/stamp.part\" || commits=0$/\1/p" "$STEPS")
check "seal_final's pattern is in the file" test -n "$pat"
check "seal_final takes a stamp naming 1.0" grep -qE "$pat" <(printf 'appliance.version = 1.0\n')
check "seal_final refuses one naming 1.1, or none" eval "! grep -qE '$pat' <(printf 'appliance.version = 1.1\nx = y\n')"

# The prelude: everything before the file's first function, with os-release pointed at a
# stand-in that assigns VERSION as Ubuntu's does.
printf '%s\n' 'NAME="Ubuntu"' 'VERSION="26.04 LTS (Resolute Raccoon)"' 'VERSION_ID="26.04"' \
    'PRETTY_NAME="Ubuntu 26.04 LTS"' 'ID=ubuntu' >"$T/os-release"
pre=$(awk '/^[a-z_]+\(\) \{/ { exit } { print }' "$STEPS" | sed "s|/etc/os-release|$T/os-release|g")
check "the prelude reads os-release" grep -q "$T/os-release" <<<"$pre"
got=$(VERSION=1.0 bash -c "$pre"'
printf "%s|%s|%s|%s\n" "$VERSION" "$VERSION_ID" "$PRETTY_NAME" "$CXX_MAJOR"' 2>&1)
check "VERSION survives the prelude, which reads 26.04 and its name ($got)" \
    test "$got" = "1.0|26.04|Ubuntu 26.04 LTS|15"
sed -i 's/^VERSION_ID=.*/VERSION_ID="25.10"/' "$T/os-release"
pre=$(awk '/^[a-z_]+\(\) \{/ { exit } { print }' "$STEPS" | sed "s|/etc/os-release|$T/os-release|g")
VERSION=1.0 bash -c "$pre" >"$T/other.out" 2>&1; rc=$?
check "another Ubuntu is refused by the prelude, status 2" test "$rc" = 2 -a "$(grep -c 'no build for Ubuntu 25.10' "$T/other.out")" = 1
finish
