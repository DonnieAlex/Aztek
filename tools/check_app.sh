#!/usr/bin/env bash
# Compiles the shell's C++ (app/src) with clang and GCC as strict C++17, in a few minutes and without building Rust:
# what MSVC, AppleClang or GCC would stop on shows here before the long Windows, macOS and Linux builds reach it.
# tools/check_app.sh. Needs Qt 6 development files and CMake; Ninja when it is there.
set -uo pipefail
cd "$(dirname "$0")/.."
root=$(pwd)
fail=0
gen=()
command -v ninja >/dev/null && gen=(-G Ninja)
for cxx in clang++ g++; do
  command -v "$cxx" >/dev/null || { echo "$cxx: not installed, skipped"; continue; }
  dir=$(mktemp -d)
  # The moc and uic output is all a syntax check needs: Python and the Rust build stay out of it.
  if ! cmake -S "$root" -B "$dir" "${gen[@]}" -DCMAKE_CXX_COMPILER="$cxx" -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DAZTEK_PYTHON=OFF -DAZTEK_SHIPPED_PLUGINS=OFF -DCMAKE_AUTOGEN_ORIGIN_DEPENDS=OFF >"$dir/configure.log" 2>&1; then
    echo "$cxx: configure failed"; tail -20 "$dir/configure.log"; fail=1; continue
  fi
  if ! cmake --build "$dir" --target aztek_autogen >"$dir/autogen.log" 2>&1; then
    echo "$cxx: moc failed"; tail -20 "$dir/autogen.log"; fail=1; continue
  fi
  # Each app/src file's own command, compiling nothing out.
  python3 - "$dir/compile_commands.json" "$root/app/src/" >"$dir/commands.sh" <<'EOF'
import json, shlex, sys
for e in json.load(open(sys.argv[1])):
    if not e["file"].startswith(sys.argv[2]):
        continue
    args = shlex.split(e["command"]) if "command" in e else e["arguments"]
    out = []
    skip = False
    for a in args:
        if skip:
            skip = False
            continue
        if a in ("-o", "-MF", "-MT"):
            skip = True
            continue
        if a == "-MD":
            continue
        out.append(a)
    print("cd " + shlex.quote(e["directory"]) + " && " + shlex.join(out + ["-fsyntax-only"]))
EOF
  while IFS= read -r line; do
    if ! out=$(bash -c "$line" 2>&1); then
      echo "$cxx:"; echo "$out" | grep -E "error" | head -8; fail=1
    fi
  done <"$dir/commands.sh"
  rm -rf "$dir"
done
[ $fail = 0 ] && echo "app: no errors under clang or GCC"
exit $fail
