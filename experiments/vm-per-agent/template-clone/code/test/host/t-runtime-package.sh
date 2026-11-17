#!/usr/bin/env bash
# The CUDA runtime package (the Planner's ruling of 2026-11-16, form (b)): guest/cuda-runtime/
# PKGBUILD built by this host's makepkg, as this account and under fakeroot, from a stand-in
# toolkit tree, with runtime_build's own makepkg line and its own check of the package's
# file list. The package must hold the listed files and links and nothing else, each file
# byte for byte (an ELF with its symbols and debug sections is not stripped), named and
# licensed as the PKGBUILD says. Nothing is installed. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need makepkg fakeroot bsdtar pacman zstd cc readelf
STEPS=$CODE/guest/template-steps PKGBUILD_SRC=$CODE/guest/cuda-runtime/PKGBUILD
eval "$(sed -n '/^RUNTIME_PKG=/p' "$STEPS")"
# The stand-in toolkit: a shared library built here with symbols and debug sections, the
# others plain bytes, the links as the toolkit lays them, and the toolkit's loader file,
# which the package does not take: it writes its own, naming the library directory alone.
R=$T/root; L=$R/opt/cuda/targets/x86_64-linux/lib; mkdir -p "$L" "$R/etc/ld.so.conf.d"
printf 'int weaver_runtime_probe(int x) { return x + 1; }\n' >"$T/probe.c"
cc -shared -fPIC -g -o "$L/libcudart.so.13.4.39" "$T/probe.c" || exit 1
for n in cublas.so.13.4.1.2 cublasLt.so.13.4.1.2 nvrtc-builtins.so.13.4.42; do head -c 4096 /dev/urandom >"$L/lib$n"; done
chmod 0755 "$L/libcublas.so.13.4.1.2"
ln -s libcudart.so.13.4.39 "$L/libcudart.so.13"; ln -s libcudart.so.13 "$L/libcudart.so"
ln -s targets/x86_64-linux/lib "$R/opt/cuda/lib64"; printf '/opt/cuda/lib64\n/opt/cuda/nvvm/lib64\n' >"$R/etc/ld.so.conf.d/cuda.conf"
RUNTIME_LDCONF=$R/etc/ld.so.conf.d/cuda.conf CUDA_LIBS=$L
check "the stand-in library carries debug sections, so a strip would change it" \
    eval 'readelf -S "$L/libcudart.so.13.4.39" | grep -q "\.debug_info"'
RUNTIME_DIR=$T/runtime; mkdir -p "$RUNTIME_DIR"
printf '%s\n' "$R/opt/cuda/lib64" "$L/libcublas.so.13.4.1.2" "$L/libcublasLt.so.13.4.1.2" \
    "$L/libcudart.so" "$L/libcudart.so.13" "$L/libcudart.so.13.4.39" "$L/libnvrtc-builtins.so.13.4.42" >"$RUNTIME_DIR/files"
# runtime_build's makepkg line and its check, from the file. HOME and XDG_CONFIG_HOME point
# into the test's directory, so no makepkg.conf of this account's is read.
MKLINE=$(sed -n '/^    (cd "\$w" && WEAVER_RUNTIME_LIST=/,/makepkg --noconfirm --nocheck 2>&1) || return 1$/p' "$STEPS")
CHECK=$(sed -n '/^    out=\$(diff <({ cat "\$RUNTIME_DIR\/files"; echo "\$RUNTIME_LDCONF"; }/,+1p' "$STEPS")
check "runtime_build's makepkg line and its list check are in the file" test "$(wc -l <<<"$MKLINE")" = 3 -a "$(wc -l <<<"$CHECK")" = 2
eval "mk() { local w=\$1 v=\$2; cp \"\$PKGBUILD_SRC\" \"\$w/PKGBUILD\" || return 1
$MKLINE
}"
eval "listcheck() { local f=\$1 out; $CHECK
    printf '%s' \"\$out\"; }"
build() { local w; w=$(mktemp -d "$T/w.XXXXXX"); (HOME=$T/home XDG_CONFIG_HOME=$T/home/.config; mkdir -p "$HOME"; mk "$w" "$1") >"$w.log" 2>&1 || return 1; echo "$w"; }
w=$(build 13.4.3); rc=$?
check "makepkg builds the package as this account" test "$rc" = 0 -a -n "$w"
f=$w/out/$RUNTIME_PKG-13.4.3-1-x86_64.pkg.tar.zst
check "named for the toolkit's version, release 1: $RUNTIME_PKG-13.4.3-1" test -s "$f"
check "runtime_build's check: the package's files and links are the list and its loader file, nothing more" test -z "$(listcheck "$f")"
X=$T/x; mkdir -p "$X"; bsdtar -xf "$f" -C "$X" || exit 1
same=1; while IFS= read -r p; do
    if [ -L "$p" ]; then [ -L "$X$p" ] && [ "$(readlink "$X$p")" = "$(readlink "$p")" ] || { echo "     link differs: $p"; same=0; }
    else cmp -s "$p" "$X$p" && [ "$(stat -c %a "$p")" = "$(stat -c %a "$X$p")" ] || { echo "     file differs: $p"; same=0; }; fi
done <"$RUNTIME_DIR/files"
check "each file byte for byte with its mode, each link with its target as written" test "$same" = 1
check "the package's loader file names the library directory alone, mode 0644, not the toolkit's nvvm" \
    test "$(cat "$X$RUNTIME_LDCONF")" = "$L" -a "$(stat -c %a "$X$RUNTIME_LDCONF")" = 644
check "the library keeps its debug sections: nothing stripped" eval 'readelf -S "$X$L/libcudart.so.13.4.39" | grep -q "\.debug_info"'
check "the package's owner is root, as fakeroot records it" test "$(bsdtar -tvf "$f" | grep -F "${L#/}/libcudart.so.13.4.39" | awk '{print $3}')" = root
PI=$X/.PKGINFO
check "its metadata: the name, the version, the license" \
    test "$(grep -cxE "pkgname = $RUNTIME_PKG|pkgver = 13.4.3-1|license = LicenseRef-NVIDIA-CUDA|arch = x86_64" "$PI")" = 4
MDLINE=$(sed -n '/^    echo "\$RUNTIME_PKG metadata, as makepkg wrote it: /p' "$STEPS")
check "runtime_build records the package's metadata line" eval '[ -n "$MDLINE" ] && grep -q "pkgver\|license" <<<"$(eval "$MDLINE")"'
check "and no install script or backup entry" test ! -e "$X/.INSTALL" -a "$(grep -c '^backup = ' "$PI")" = 0
# A list with one more line than the package would hold, and one with a relative path.
cp "$RUNTIME_DIR/files" "$T/files.good"; echo "$L/libcurand.so.10.4.1.9" >>"$RUNTIME_DIR/files"
check "a listed file the package lacks: the check names it" eval 'grep -q "libcurand.so.10.4.1.9" <<<"$(listcheck "$f")"'
cp "$T/files.good" "$RUNTIME_DIR/files"; echo "opt/cuda/lib64" >>"$RUNTIME_DIR/files"
build 13.4.3 >/dev/null; rc=$?
check "a relative path in the list: makepkg fails, naming it" \
    test "$rc" != 0 -a "$(cat "$T"/w.*.log | grep -c 'not an absolute path: opt/cuda/lib64')" -ge 1
cp "$T/files.good" "$RUNTIME_DIR/files"
build "" >/dev/null; rc=$?
check "no version: makepkg fails" test "$rc" != 0
finish
