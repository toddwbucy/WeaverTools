#!/usr/bin/env bash
# The harness (lib-test.sh) clears the build's inputs and knobs, BUILD_ENV_NAMES, before
# any test reads them, so a test passes or fails the same in a clean shell and in one that
# has exported a build's inputs, as COMMANDS.md's recipe block does. Checked here:
# - every name, exported to a wrong value of its own form, is unset once lib-test.sh is
#   sourced, and the names a test may read (KEPT_IMAGE, NVCC, CUOBJDUMP, TMPDIR) are kept;
# - the list covers every name the three scripts read from the environment;
# - every test file sources lib-test.sh before any other command, so a file run alone
#   gets the clearing too;
# - the names the scripts set themselves (SET_ENV_NAMES) and every CUDA_ name the host
#   exports are unset once lib-test.sh is sourced, and the list covers every name the
#   three scripts export, bar PATH;
# - the two tests that read those names (t-version, t-tag; measured 2026-11-16) pass with
#   every name exported wrong, with the same count as in a clean shell. Every test this
#   file runs inherits the harness's clearing, so none of them sees the host's CUDA_
#   names;
# - and, as controls, with the clearing taken out of a copy of the harness, the same
#   exports fail t-tag; and a stand-in test that reads CUDA_PATH without setting it, as
#   t-tag did until 2026-11-17, fails under the harness with CUDA_PATH exported and passes
#   under a copy without the scrub.
# Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
H=$CODE/test/host
# wrong NAME: a value of the name's own form that no test expects.
wrong() {
    case $1 in
        OS) echo 24.04 ;; VERSION) echo 9.9 ;; DRIVER) echo 1.2.3 ;; COMPUTE_CAP) echo 12 ;;
        DRIVER_ROUTE) echo cuda-runfile ;;
        WA_COMMIT) printf '1%.0s' {1..40} ;; WW_COMMIT) printf '2%.0s' {1..40} ;;
        PORT|PROOF_PORT|VCPUS|MEMORY|SEAL_WAIT|CI_WAIT|PROOF_BOOT|PROOF_FIRST_BOOT|PROOF_ACPI) echo 1 ;;
        *) echo "/wrong/$1" ;;
    esac
}
wrongs=()
for n in $BUILD_ENV_NAMES; do wrongs+=("$n=$(wrong "$n")"); done
check "the list names the seven inputs, VM_ROOT, NAME and PORT" \
    bash -c 'for n in OS VERSION DRIVER DRIVER_ROUTE COMPUTE_CAP WA_COMMIT WW_COMMIT VM_ROOT NAME PORT; do grep -qw "$n" <<<"$1" || exit 1; done' _ "$BUILD_ENV_NAMES"

left=$(env "${wrongs[@]}" KEPT_IMAGE=/k NVCC=/n CUOBJDUMP=/c TMPDIR="$T" bash -c '
    source "$1/lib-test.sh"
    for n in $BUILD_ENV_NAMES; do [ -z "${!n+set}" ] || echo "$n"; done
    [ "$KEPT_IMAGE $NVCC $CUOBJDUMP $TMPDIR" = "/k /n /c $2" ] || echo "a kept name was cleared"
    finish >/dev/null' _ "$H" "$T" 2>&1)
check "each name exported wrong is unset once the harness is sourced; the kept names stay" test -z "$left"
[ -z "$left" ] || echo "     left: $left"

# The names the scripts set themselves, and two CUDA_ names a host exports, exported to a
# value: each is unset once the harness is sourced.
setwrongs=(CUDA_DEVICE_ORDER=PCI_BUS_ID CUDA_VISIBLE_DEVICES=0)
for n in $SET_ENV_NAMES; do setwrongs+=("$n=/wrong/$n"); done
set_left=$(env "${setwrongs[@]}" TMPDIR="$T" bash -c '
    source "$1/lib-test.sh"
    for n in $SET_ENV_NAMES CUDA_DEVICE_ORDER CUDA_VISIBLE_DEVICES; do [ -z "${!n+set}" ] || echo "$n"; done
    compgen -e CUDA_
    finish >/dev/null' _ "$H" 2>&1)
check "each name the scripts set, and each CUDA_ name exported, is unset once the harness is sourced" test -z "$set_left"
[ -z "$set_left" ] || echo "     left: $set_left"

# Every name the three scripts export, bar PATH, which the scripts prepend to.
exported=$(grep -ohE '^[[:space:]]*export [A-Za-z_][A-Za-z0-9_]*=' "$CODE/build-template.sh" "$CODE/lib.sh" \
    "$CODE/guest/template-steps" | sed -E 's/^[[:space:]]*export ([A-Za-z0-9_]+)=.*/\1/' | sort -u)
missing_set=$(for n in $exported; do
    [ "$n" = PATH ] && continue
    grep -qw "$n" <<<"$SET_ENV_NAMES" || echo "$n"
done)
check "the scripts' exports are found, and the list covers each of them, bar PATH" \
    test -n "$exported" -a -z "$missing_set"
[ -z "$missing_set" ] || echo "     not cleared: $missing_set"

# Every name the scripts read with a default, bar the kept and the internal: DIR (set in
# build-template.sh), ID and PRETTY_NAME (os-release's, read in subshells).
read_names=$(grep -ohE '\$\{[A-Z][A-Z0-9_]*:[-=?+]' "$CODE/build-template.sh" "$CODE/lib.sh" "$CODE/guest/template-steps" \
    | sed -E 's/^\$\{([A-Z0-9_]+):.*/\1/' | sort -u)
missing=$(for n in $read_names; do
    case " CUOBJDUMP DIR ID PRETTY_NAME " in *" $n "*) continue ;; esac
    grep -qw "$n" <<<"$BUILD_ENV_NAMES" || echo "$n"
done)
check "the list covers every name the three scripts read from the environment" test -z "$missing"
[ -z "$missing" ] || echo "     not cleared: $missing"

late=$(for t in "$H"/t-*.sh; do
    first=$(grep -vE '^[[:space:]]*(#|$)' "$t" | head -1)
    [ "$first" = 'source "$(dirname "$0")/lib-test.sh"' ] || echo "${t##*/}"
done)
check "every test file sources the harness before any other command" test -z "$late"
[ -z "$late" ] || echo "     late: $late"

for t in t-version.sh t-tag.sh; do
    clean=$(TMPDIR=$T bash "$H/$t" 2>&1 | tail -1 | sed 's/ (its folder kept: .*//'); rc1=$?
    under=$(env "${wrongs[@]}" "${setwrongs[@]}" TMPDIR="$T" bash "$H/$t" 2>&1 | tail -1 | sed 's/ (its folder kept: .*//')
    check "$t passes with every name exported wrong, with the clean count ($clean)" \
        bash -c '[[ "$1" =~ ^==\ [0-9]+\ passed,\ 0\ failed$ ]] && [ "$1" = "$2" ]' _ "$clean" "$under"
done

# The control: a copy of the code whose harness lacks the clearing fails t-tag under the
# same exports, so the check above would see the clearing go.
C=$T/code; mkdir -p "$C/test/host"
for e in "$CODE"/*; do [ "${e##*/}" = test ] || ln -s "$e" "$C/${e##*/}"; done
grep -vxF 'unset $BUILD_ENV_NAMES' "$H/lib-test.sh" >"$C/test/host/lib-test.sh"
cp "$H/t-tag.sh" "$C/test/host/t-tag.sh"
check "the control copy differs from the harness by the clearing alone" \
    test "$(diff "$H/lib-test.sh" "$C/test/host/lib-test.sh" | grep -c '^<')" = 1
ctl=$(TMPDIR=$T bash "$C/test/host/t-tag.sh" 2>&1 | tail -1 | sed 's/ (in .*//')
check "the control copy passes in a clean shell, with the clean count" test "$ctl" = "$clean"
ctl=$(env WA_COMMIT="$(wrong WA_COMMIT)" TMPDIR="$T" bash "$C/test/host/t-tag.sh" 2>&1); rc=$?
check "without the clearing, WA_COMMIT exported fails checks of t-tag (as measured)" \
    test "$rc" = 1 -a "$(grep -c '^FAIL ' <<<"$ctl")" -gt 0
# With every name exported, lib.sh refuses OS=24.04 before t-tag prints a FAIL line.
ctl=$(env "${wrongs[@]}" TMPDIR="$T" bash "$C/test/host/t-tag.sh" 2>&1); rc=$?
check "without the clearing, every name exported keeps t-tag from passing" \
    test "$rc" != 0 -a "$(grep -cE '^== [0-9]+ passed, 0 failed' <<<"$ctl")" = 0

# The scrub's control: a stand-in test that reads CUDA_PATH and never sets it, as t-tag
# did, fails under the harness with CUDA_PATH exported, so such a test fails on every
# host, and passes under a copy of the harness without the scrub.
SCRUB='unset $SET_ENV_NAMES $(compgen -e CUDA_)'
for d in with without; do
    mkdir -p "$T/lean-$d"
    printf '%s\n' 'source "$(dirname "$0")/lib-test.sh"' \
        'check "CUDA_PATH is read, never set" test -n "${CUDA_PATH:-}"' finish >"$T/lean-$d/t-lean.sh"
done
cp "$H/lib-test.sh" "$T/lean-with/lib-test.sh"
grep -vxF "$SCRUB" "$H/lib-test.sh" >"$T/lean-without/lib-test.sh"
check "the scrub's control copy differs from the harness by the scrub alone" \
    test "$(diff "$H/lib-test.sh" "$T/lean-without/lib-test.sh" | grep -c '^<')" = 1
check "a test reading CUDA_PATH it never sets fails under the harness, with CUDA_PATH exported" \
    eval '! env CUDA_PATH=/opt/cuda TMPDIR="$T" bash "$T/lean-with/t-lean.sh" >/dev/null 2>&1'
check "and passes under the copy without the scrub" \
    eval 'env CUDA_PATH=/opt/cuda TMPDIR="$T" bash "$T/lean-without/t-lean.sh" >/dev/null 2>&1'
finish
