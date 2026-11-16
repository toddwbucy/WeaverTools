#!/usr/bin/env bash
# refusal_judge against ssh -v outputs as a refused, an accepted and a cut login print
# them, each judged under errexit and pipefail as proof runs it. A server that names no
# methods (a connection cut before authentication, as sshd's PerSourcePenalties cuts one)
# is a finding the judge prints, not an exit that ends the proof without a verdict
# (CodeRabbit's review of #33, 2026-11-16). Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
check "refusal_judge is in lib.sh" eval 'declare -F refusal_judge >/dev/null'
# judge NAME RC TEXT: the judge's line for TEXT as OUT, under errexit and pipefail; the
# shell's own status is appended, so an exit before the verdict shows.
judge() { printf '%b' "$3" >"$T/$1"; ( set -euo pipefail; refusal_judge "$T/$1" "$2" "$1" ); echo "status $?"; }
REFUSED='debug1: Authentications that can continue: publickey\ndebug1: Next authentication method: publickey\ndebug1: Authentications that can continue: publickey\nroot@127.0.0.1: Permission denied (publickey).\n'
got=$(judge refused 255 "$REFUSED")
check "publickey alone, refused: refused, status 0" test "$got" = "refused: refused (the server offers publickey alone)"$'\n'"status 0"
got=$(judge cut 255 'Connection closed by 127.0.0.1 port 22\n')
check "no methods named (a cut connection): a PROOF-FAIL naming it, status 0" \
    test "$got" = "PROOF-FAIL: SSH as root with cut: the server named no methods; no 'Permission denied (publickey)'"$'\n'"status 0"
got=$(judge password 255 "${REFUSED//: publickey/: publickey,password}")
check "password offered too: a PROOF-FAIL naming the methods" test "$got" = "PROOF-FAIL: SSH as root with password: the server offered publickey,password"$'\n'"status 0"
got=$(judge in 0 'debug1: Authentications that can continue: publickey\nAuthenticated to 127.0.0.1 ([127.0.0.1]:22) using "publickey".\n')
check "a login that went through: a PROOF-FAIL naming it and the status" \
    test "$got" = "PROOF-FAIL: SSH as root with in: the login went through; no 'Permission denied (publickey)'; ssh exited 0, not 255"$'\n'"status 0"
got=$( (set -euo pipefail; refusal_judge "$T/absent" 255 absent) 2>/dev/null; echo "status $?")
check "an absent output file: a PROOF-FAIL, status 0" test "$(grep -c '^PROOF-FAIL: SSH as root with absent: the server named no methods' <<<"$got")-$(tail -1 <<<"$got")" = "1-status 0"
finish
