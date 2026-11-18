#!/usr/bin/env bash
# Times every poly tool on a dense mesh: tools/poly_bench.sh [faces] [seconds each] [name filter]
# One process a command, stopped after the limit (shown as "> N s"). Build first:
#   cargo build --release -p aztek_host --example poly_bench
set -u
faces=${1:-250000}
limit=${2:-120}
only=${3:-}
bin=${POLY_BENCH:-target/release/examples/poly_bench}
half="select -r m.f[0:$((faces / 2))];"
loop="select -r m.e[0];"
all_e="select -r m.e[*];"
while IFS='|' read -r name setup cmd; do
  [ -z "$name" ] && continue
  case "$name" in \#*) continue ;; esac
  [ -n "$only" ] && [[ "$name" != *$only* ]] && continue
  start=$(date +%s.%N)
  out=$(timeout "$limit" "$bin" "$faces" "$setup" "$cmd" 2>/dev/null)
  rc=$?
  if [ $rc = 124 ]; then out="> $limit s"; fi
  printf '%-26s %s\n' "$name" "$out"
done <<EOF
mirror|select -r m;|polyMirrorFace -ws 1 -direction 1 -mergeMode 1 m;
mirror_cut|select -r m;|polyMirrorFace -ws 1 -direction 1 -mergeMode 1 -cutMesh 1 m;
retopo|select -r m;|polyRetopo -targetFaceCount 5000 m;
remesh|select -r m;|polyRemesh m;
reduce|select -r m;|polyReduce -p 50 m;
smooth|select -r m;|polySmooth -dv 1 m;
triangulate|select -r m;|polyTriangulate m;
quadrangulate|polyTriangulate m;|polyQuad m;
merge_vertex|select -r m.vtx[*];|polyMergeVertex -d 0.0001 m.vtx[*];
soft_edge|select -r m;|polySoftEdge -a 30 m;
normal_reverse|select -r m;|polyNormal -nm 0 m;
extrude_faces|$half|polyExtrudeFacet -ltz 0.1;
extrude_edges|select -r m.e[0:20000];|polyExtrudeEdge -ltz 0.1;
bevel_edges|select -r m.e[0:20000];|polyBevel3 -f 0.3 -sg 2;
chamfer_vertex|select -r m.vtx[0:5000];|polyExtrudeVertex -w 0.1 -l 0.1;
delete_faces|$half|delete;
delete_edges|select -r m.e[0:20000];|polyDelEdge -cv 1;
collapse_edges|select -r m.e[0:20000];|polyCollapseEdge;
poke|$half|polyPoke;
subdivide_faces|$half|polySubdivideFacet -dv 1;
chip_off|$half|polyChipOff -dup 0;
duplicate_faces|$half|polyDuplicateEdge 0.5 m.e[0:2000];
insert_loop|$loop|polySplitRing -wt 0.5;
fill_hole|delete m.f[0:2000];|polyCloseBorder m;
average_vertex|select -r m;|polyAverageVertex -i 10 m;
move_vertices|select -r m.vtx[*];|move -r 0 1 0;
move_object|select -r m;|move -r 0 1 0;
separate|polyChipOff -dup 0 m.f[0:1000];|polySeparate m;
combine|polySphere -n m2 -sx 400 -sy 200;|polyUnite m m2;
boolean|polyCube -n b -w 12 -h 12 -d 12; move 8 0 0 b;|polyCBoolOp -op 2 m b;
planar_uv|select -r m;|polyPlanarProjection -md x m.f[*];
auto_uv|select -r m;|polyAutoProjection m.f[*];
unfold_uv|polyAutoProjection m.f[*];|u3dUnfold m;
layout_uv|polyAutoProjection m.f[*];|u3dLayout m;
symmetry_map|select -r m;|symmetricModelling -s 1 -about world -axis x;
convert_selection|select -r m.f[*];|polyListComponentConversion -tv m.f[*];
select_all_faces||select -r m.f[*];
grow_selection|select -r m.f[0:1000];|polySelectConstraint -pp 1;
evaluate||polyEvaluate -v -e -f -uv m;
delete_history|polySmooth -dv 1 m;|delete -ch m;
undo_extrude|polyExtrudeFacet -ltz 0.1 m.f[0:1000];|undo;
EOF
