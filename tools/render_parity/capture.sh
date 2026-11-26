#!/usr/bin/env bash
# Renders every lighting parity scene both ways and keeps the pictures for comparison:
#   examples/rendering/reference/<scene>.png  the Render View's path (Harold)
#   examples/rendering/viewport/<scene>.png   the realtime viewport, cropped to the same aspect
# Usage: tools/render_parity/capture.sh [build/Aztek/AppRun] [scene ...]
# Needs a display (Xvfb is enough) and ImageMagick. Runs from anywhere; paths are taken from the repository root.
set -euo pipefail
cd "$(dirname "$0")/../.."
root=$(pwd)
app=${1:-build/Aztek/AppRun}
case "$app" in /*) ;; *) app="$root/$app" ;; esac
[ -x "$app" ] || { echo "no application at $app (build it first)"; exit 1; }
shift || true
command -v magick >/dev/null && im=magick || im=convert

scenes=("$@")
if [ ${#scenes[@]} -eq 0 ]; then
  scenes=(reflections metal_roughness_sweep dielectric_glass tinted_glass emissive_object area_lights environment_hdri checker_sphere)
fi

mkdir -p examples/rendering/reference examples/rendering/viewport
work=$(mktemp -d "${TMPDIR:-/tmp}/aztek_parity.XXXXXX")
trap 'rm -rf "$work"' EXIT
export XDG_CONFIG_HOME="$work/config" XDG_DATA_HOME="$work/data"

for name in "${scenes[@]}"; do
  scene="examples/rendering/$name.ma"
  echo "== $name"
  "$app" -render -s 1 -cam shotCam -x 640 -y 360 -rd "$root/examples/rendering/reference" -im "$name" -of png "$root/$scene" >/dev/null
  "$app" --script "$root/tools/render_parity/viewport.py" --viewport-screenshot "$work/$name.raw.png" "$root/$scene" >/dev/null
  read -r w h < <("$im" identify -format '%w %h\n' "$work/$name.raw.png")
  ch=$(( w * 9 / 16 ))
  "$im" "$work/$name.raw.png" -gravity center -crop "${w}x${ch}+0+0" +repage -resize 640x360\! "examples/rendering/viewport/$name.png"
  echo "   reference/$name.png  viewport/$name.png"
done
echo "done"
