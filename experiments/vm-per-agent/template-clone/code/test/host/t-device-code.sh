#!/usr/bin/env bash
# device_code_check (decision 6 of 2026-11-15), on stand-in images whose /opt/weaver/lib
# holds libggml-cuda.so.0 -> libggml-cuda.so.0.9.11, opened read-only as a sealed image is.
# - Always: a stub cuobjdump that prints set listings, for the listing forms the check reads
#   (the Planner's review of 2026-11-15, F7).
# - Where nvcc and cuobjdump are on the host (NVCC, CUOBJDUMP): stand-in libraries the
#   host's nvcc makes here, listed by the real cuobjdump.
# - Where KEPT_IMAGE names a sealed image built before the change: that image, read-only,
#   as the measured negative, with its owner, mode, length and time read back.
# Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need qemu-img qemu-storage-daemon fusermount3 sfdisk mkfs.ext4 debugfs
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
REAL_CUOBJDUMP=$CUOBJDUMP
# img NAME LIB [broken]: a qcow2 whose /opt/weaver/lib carries LIB behind ggml's link chain.
img() {
    local d; d=$(mktemp -d "$T/img-$1.XXXXXX"); mkdir -p "$d/tree/opt/weaver/lib"
    if [ "${3:-}" = broken ]; then ln -s libggml-cuda.so.0.9.11 "$d/tree/opt/weaver/lib/libggml-cuda.so.0"
    else cp "$2" "$d/tree/opt/weaver/lib/libggml-cuda.so.0.9.11"; ln -s libggml-cuda.so.0.9.11 "$d/tree/opt/weaver/lib/libggml-cuda.so.0"; fi
    ln -s libggml-cuda.so.0 "$d/tree/opt/weaver/lib/libggml-cuda.so"
    truncate -s 64M "$d/disk.raw"
    printf 'label: gpt\nstart=2048, size=110592, type=4F68BCE3-E8CD-4DB1-96E7-FBCAF984B709\n' | sfdisk -q "$d/disk.raw"
    truncate -s $((110592 * 512)) "$d/part.raw"; mkfs.ext4 -q -F -d "$d/tree" "$d/part.raw"
    dd if="$d/part.raw" of="$d/disk.raw" bs=512 seek=2048 conv=notrunc status=none
    qemu-img convert -f raw -O qcow2 "$d/disk.raw" "$d/img.qcow2"; rm -f -- "$d/disk.raw" "$d/part.raw"
    chmod 0444 "$d/img.qcow2"
    printf '%s\n' "$d/img.qcow2"
}
# run DISK: device_code_check on DISK, opened read-only, with its record in T/dc.out.
run() { local r; r=$(mktemp -d "$T/read.XXXXXX")
    ( image_open "$1" "$r" && device_code_check "$r" "$r"; image_close "$r" ) >"$T/dc.out" 2>&1; }
fails() { grep -c '^PROOF-FAIL: ' "$T/dc.out" || true; }
checked() { grep -qx 'device code: checked' "$T/dc.out"; }

# The listing forms, with a stub cuobjdump.
STUB=$T/stub; mkdir -p "$STUB"
printf '#!/usr/bin/env bash\ncase $1 in\n  --version) echo "Cuda compilation tools, release 13.4, V13.4.92" ;;\n  --list-ptx) cat "%s/ptx" ;;\n  --list-elf) cat "%s/elf" ;;\nesac\n' "$STUB" "$STUB" >"$STUB/cuobjdump"
chmod 0755 "$STUB/cuobjdump"
printf 'stand-in\n' >"$T/stand-in.so"
SI=$(img stub "$T/stand-in.so")
noptx="cuobjdump info    : No PTX file found to extract from 'libggml-cuda.so'. You may try with -all option."
elf4() { local a n=0; for a in $GGML_CUBIN_ARCHS; do n=$((n + 1)); printf 'ELF file    %s: libggml-cuda.%s.%s.cubin\n' "$n" "$n" "$a"; done; }
# elfn A...: one ELF line for each architecture named, in order.
elfn() { local a n=0; for a; do n=$((n + 1)); printf 'ELF file    %s: libggml-cuda.%s.%s.cubin\n' "$n" "$n" "$a"; done; }
stub() { printf '%s\n' "$1" >"$STUB/ptx"; printf '%s\n' "$2" >"$STUB/elf"; CUOBJDUMP=$STUB/cuobjdump run "$SI"; }
stub "$noptx" "$(elf4)"
check "stub: no PTX said, one cubin for each of the four: no finding (one cubin passes)" test "$(fails)" = 0 -a "$(checked && echo y)" = y
stub "" "$(elf4)"
check "stub: a PTX listing that is empty, without cuobjdump saying it found none, is a finding" \
    test "$(fails)" = 1 -a "$(grep -c '^PROOF-FAIL: --list-ptx named no PTX image and did not say it found none' "$T/dc.out")" = 1
stub "PTX image    1: libggml-cuda.1.sm_80.ptx" "$(elf4)"
check "stub: a PTX line worded otherwise is a finding, never no PTX" \
    test "$(grep -c "^PROOF-FAIL: a --list-ptx line out of the forms" "$T/dc.out")" = 1
stub "$noptx" "$(elf4; echo 'ELF section 1: libggml-cuda.9.sm_86.cubin')"
check "stub: an ELF line worded otherwise is a finding" \
    test "$(grep -c "^PROOF-FAIL: a --list-elf line out of the form 'ELF file N: NAME'" "$T/dc.out")" = 1
stub "PTX file    1: libggml-cuda.1.sm_80.ptx" "$(elf4)"
check "stub: a PTX image is a finding, the only one" test "$(fails)" = 1 -a "$(grep -c '^PROOF-FAIL: libggml-cuda carries 1 PTX image(s)' "$T/dc.out")" = 1
stub "$noptx" "$(elfn sm_86 sm_89 sm_120a sm_121a sm_86 sm_89 sm_120a sm_121a)"
check "stub: two cubins for each of the four: no finding, and the count recorded, not fixed" \
    test "$(fails)" = 0 -a "$(grep -cx 'cubins per architecture: sm_86=2 sm_89=2 sm_120a=2 sm_121a=2' "$T/dc.out")" = 1
stub "$noptx" "$(elfn sm_86 sm_89 sm_120a sm_121a sm_86 sm_89 sm_120a)"
check "stub: one architecture with fewer cubins than the rest is a finding, the only one" \
    test "$(fails)" = 1 -a "$(grep -cx 'PROOF-FAIL: libggml-cuda carries a different count of cubins for its architectures: sm_86=2 sm_89=2 sm_120a=2 sm_121a=1' "$T/dc.out")" = 1
stub "$noptx" "$(elfn sm_86 sm_89 sm_120a sm_86 sm_89)"
check "stub: an absent architecture is one finding, and the uneven rest another" \
    test "$(fails)" = 2 -a "$(grep -cx 'PROOF-FAIL: libggml-cuda carries no cubin for sm_121a' "$T/dc.out")" = 1 \
         -a "$(grep -c '^PROOF-FAIL: libggml-cuda carries a different count of cubins' "$T/dc.out")" = 1

# The real cuobjdump, on libraries the host's nvcc makes.
NVCC=${NVCC:-nvcc}
if command -v "$NVCC" >/dev/null 2>&1 && command -v "$REAL_CUOBJDUMP" >/dev/null 2>&1; then
    CUOBJDUMP=$REAL_CUOBJDUMP
    printf '__global__ void k(float *x) { x[threadIdx.x] *= 2.0f; }\nextern "C" void run(float *x) { k<<<1, 32>>>(x); }\n' >"$T/k.cu"
    g() { local a o=""; for a in "$@"; do case $a in ptx80) o+=" -gencode arch=compute_80,code=compute_80" ;; *) o+=" -gencode arch=compute_$a,code=sm_$a" ;; esac; done; echo "$o"; }
    mk() { # shellcheck disable=SC2046
        "$NVCC" -shared -Xcompiler -fPIC $(g "${@:2}") "$T/k.cu" -o "$T/$1.so" >"$T/nvcc-$1.txt" 2>&1 || { no "nvcc built $1"; cat "$T/nvcc-$1.txt"; }; }
    mk four 86 89 120a 121a; mk ptx 86 89 120a 121a ptx80; mk three 86 89 120a; mk extra 86 89 120a 121a 75
    # uneven: two objects in one library, one compiled for the four, one for three of them,
    # linked for the four. nvcc's shared link adds cubins of its own (two for each
    # architecture in a one-object library, measured with nvcc 13.4), so the counts are
    # matched as equal or not, never as a number.
    printf '__global__ void k2(float *x) { x[threadIdx.x] += 1.0f; }\nextern "C" void run2(float *x) { k2<<<1, 32>>>(x); }\n' >"$T/k2.cu"
    # shellcheck disable=SC2046
    { "$NVCC" -c -Xcompiler -fPIC $(g 86 89 120a 121a) "$T/k.cu" -o "$T/k4.o" \
        && "$NVCC" -c -Xcompiler -fPIC $(g 86 89 120a) "$T/k2.cu" -o "$T/k3.o" \
        && "$NVCC" -shared $(g 86 89 120a 121a) "$T/k4.o" "$T/k3.o" -o "$T/uneven.so"; } >"$T/nvcc-uneven.txt" 2>&1 || { no "nvcc built uneven"; cat "$T/nvcc-uneven.txt"; }
    printf 'not a library\n' >"$T/plain.so"
    run "$(img four "$T/four.so")"
    check "nvcc: four cubin sets and no PTX: no finding, and the check ran to its end" test "$(fails)" = 0 -a "$(checked && echo y)" = y
    check "nvcc: the record names the link chain's file and the counts" \
        test "$(grep -c '^/opt/weaver/lib/libggml-cuda.so.0 -> /opt/weaver/lib/libggml-cuda.so.0.9.11, ' "$T/dc.out")" = 1 -a "$(grep -c '^PTX images: 0$' "$T/dc.out")" = 1 \
             -a "$(grep -cxE 'cubins per architecture: sm_86=([1-9][0-9]*) sm_89=\1 sm_120a=\1 sm_121a=\1' "$T/dc.out")" = 1
    run "$(img uneven "$T/uneven.so")"
    check "nvcc: an object compiled for fewer architectures is a finding, the only one" \
        test "$(fails)" = 1 -a "$(grep -cxE 'PROOF-FAIL: libggml-cuda carries a different count of cubins for its architectures: sm_86=([0-9]+) sm_89=\1 sm_120a=\1 sm_121a=[1-9][0-9]*' "$T/dc.out")" = 1 \
             -a "$(grep -cxE 'cubins per architecture: sm_86=([0-9]+) sm_89=\1 sm_120a=\1 sm_121a=\1' "$T/dc.out")" = 0
    run "$(img ptx "$T/ptx.so")"
    check "nvcc: a PTX image is a finding, the only one" test "$(fails)" = 1 -a "$(grep -c '^PROOF-FAIL: libggml-cuda carries 1 PTX image(s)' "$T/dc.out")" = 1
    run "$(img three "$T/three.so")"
    check "nvcc: a missing architecture is a finding, the only one" test "$(fails)" = 1 -a "$(grep -c '^PROOF-FAIL: libggml-cuda carries no cubin for sm_121a$' "$T/dc.out")" = 1
    run "$(img extra "$T/extra.so")"
    check "nvcc: a cubin for an architecture off the list is a finding, the only one" test "$(fails)" = 1 -a "$(grep -c '^PROOF-FAIL: libggml-cuda carries a cubin for sm_75, off the list' "$T/dc.out")" = 1
    run "$(img plain "$T/plain.so")"
    check "nvcc: a file with no device code is a finding, and the check does not claim it ran" \
        test "$(fails)" -ge 1 -a "$(grep -c "^PROOF-FAIL: $CUOBJDUMP --list-ptx exited" "$T/dc.out")" = 1 -a "$(checked && echo n || echo y)" = y
else
    echo "note: no $NVCC or $REAL_CUOBJDUMP on this host, so the stand-in libraries are not built"
fi
run "$(img broken x broken)"
check "a link to nothing is a finding, and nothing is listed" \
    test "$(fails)" = 1 -a "$(grep -c '^PROOF-FAIL: /opt/weaver/lib/libggml-cuda.so.0.9.11 could not be read out of the image' "$T/dc.out")" = 1
( CUOBJDUMP=$T/no-such-cuobjdump; run "$SI" )
check "no cuobjdump on the host is a finding, not a pass" \
    test "$(fails)" = 1 -a "$(grep -c "^PROOF-FAIL: no $T/no-such-cuobjdump on this host" "$T/dc.out")" = 1

# A sealed image built before the change, read-only, where one is named.
if [ -n "${KEPT_IMAGE:-}" ]; then
    CUOBJDUMP=$REAL_CUOBJDUMP
    before=$(stat -c '%U:%G %a %s %Y' "$KEPT_IMAGE")
    run "$KEPT_IMAGE"
    check "the kept image fails on its PTX, and on nothing about its cubins" \
        test "$(fails)" -ge 1 -a "$(grep -c '^PROOF-FAIL: libggml-cuda carries [0-9]* PTX image(s)' "$T/dc.out")" = 1 \
             -a "$(grep -c '^PROOF-FAIL: .*cubin' "$T/dc.out")" = 0
    grep '^cubins per architecture:' "$T/dc.out"
    check "and its owner, mode, length and time are unchanged" test "$(stat -c '%U:%G %a %s %Y' "$KEPT_IMAGE")" = "$before"
fi
finish
