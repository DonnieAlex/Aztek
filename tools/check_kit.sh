#!/usr/bin/env bash
# Compiles the plug-in kit and the plug-ins it is tested with (plugins/) with clang and GCC, to catch in a minute what
# would fail the macOS (clang) and Linux builds: tools/check_kit.sh. Only errors fail it; the plug-ins' own warnings are theirs.
set -uo pipefail
cd "$(dirname "$0")/.."
fail=0
for cxx in clang++ g++; do
  command -v "$cxx" >/dev/null || { echo "$cxx: not installed, skipped"; continue; }
  # C++14, the oldest the kit's plug-ins are built with, and C++20, the VFX Reference Platform CY2027 standard.
  for std in c++14 c++20; do
    for f in devkit/src/*.cpp plugins/*/*.cpp devkit/samples/*/*.cpp; do
      [ -f "$f" ] || continue
      if ! out=$("$cxx" -std=$std -DREQUIRE_IOSTREAM -D_BOOL -DLINUX -fsyntax-only -Idevkit/include -Idevkit/src -I"$(dirname "$f")" "$f" 2>&1); then
        echo "$cxx -std=$std $f:"; echo "$out" | grep -E "error" | head -5; fail=1
      fi
    done
  done
done
[ $fail = 0 ] && echo "kit and plug-ins: no errors"
exit $fail
