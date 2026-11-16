//! Deform > Edit Membership > Prune Membership: points a deformer doesn't move leave its set.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(s: &mut Core, src: &str) -> Value {
    s.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

#[test]
fn prune_cluster_and_lattice() {
    let mut s = Core::new();
    mel(&mut s, "polyPlane -sx 2 -sy 2 -n p;");
    mel(&mut s, "select p.vtx[0:8]; cluster -n cl;");
    mel(&mut s, "percent -v 0 cl p.vtx[0:5];");
    mel(&mut s, "move -r 0 1 0 clHandle;");
    assert_eq!(mel(&mut s, "sets -q clSet;"), json!(["p.vtx[0:8]"]));
    // As Deform > Membership > Prune Membership > Cluster runs it.
    mel(&mut s, "select p;");
    mel(&mut s, "{ string $d[] = `ls -type cluster (listHistory(`ls -sl -o`))`; if (!size($d)) error \"Select an object with a cluster\"; for ($x in $d) cluster -e -prune true $x; }");
    assert_eq!(mel(&mut s, "sets -q clSet;"), json!(["p.vtx[6:8]"]));
    mel(&mut s, "undo;");
    assert_eq!(mel(&mut s, "sets -q clSet;"), json!(["p.vtx[0:8]"]));

    // A wire that reaches only part of the plane keeps only those.
    mel(&mut s, "polyPlane -w 10 -h 10 -sx 10 -sy 10 -n q; curve -d 1 -p -5 0 -5 -p -5 0 5 -n edge;");
    let w = mel(&mut s, "wire -w edge -dds 0 1.5 q;")[0].as_str().unwrap().to_string();
    mel(&mut s, "move -r 0 1 0 edge;");
    mel(&mut s, &format!("wire -e -prune true {w};"));
    let left = mel(&mut s, &format!("sets -q {w}Set;"));
    let n: usize = mel(&mut s, &format!("polyEvaluate -v `sets -q {w}Set`;")).as_u64().unwrap_or(0) as usize;
    assert!(n > 0 && n < 121, "{left}");
}
