//! Deform > Edit > Blend Shape > Add and Remove, as the runtime commands run them: targets first, the base last.

use aztek_host::Core;
use serde_json::{json, Value};

fn mel(c: &mut Core, src: &str) -> Value {
    c.mel_checked(src).unwrap_or_else(|e| panic!("{src}: {e}"))
}

const ADD: &str = "{ string $s[] = `ls -sl -o`; if (size($s) < 2) error \"Select the targets, then the base\"; string $base = $s[size($s) - 1]; string $b[] = `ls -type blendShape (listHistory($base))`; if (!size($b)) error (\"No blend shape on \" + $base); int $n = `blendShape -q -wc $b[0]`; for ($i = 0; $i < size($s) - 1; $i++) blendShape -e -t $base ($n + $i) $s[$i] 1.0 $b[0]; }";
const REMOVE: &str = "{ string $s[] = `ls -sl -o`; if (size($s) < 2) error \"Select the targets, then the base\"; string $base = $s[size($s) - 1]; string $b[] = `ls -type blendShape (listHistory($base))`; if (!size($b)) error (\"No blend shape on \" + $base); for ($i = 0; $i < size($s) - 1; $i++) blendShape -e -rmt $s[$i] $b[0]; }";

#[test]
fn add_and_remove_targets() {
    let mut c = Core::new();
    mel(&mut c, "polyPlane -sx 2 -sy 2 -n base; polyPlane -sx 2 -sy 2 -n t1; polyPlane -sx 2 -sy 2 -n t2; polyPlane -sx 2 -sy 2 -n t3;");
    mel(&mut c, "move -r 0 1 0 t1.vtx[4]; move -r 0 2 0 t2.vtx[4]; move -r 0 3 0 t3.vtx[4];");
    mel(&mut c, "blendShape -n bs t1 base;");
    assert_eq!(mel(&mut c, "blendShape -q -wc bs;"), json!(1));
    mel(&mut c, "select t2 t3 base;");
    mel(&mut c, ADD);
    assert_eq!(mel(&mut c, "blendShape -q -wc bs;"), json!(3));
    mel(&mut c, "setAttr bs.t3 1;");
    let y = mel(&mut c, "pointPosition -l base.vtx[4];")[1].as_f64().unwrap();
    assert!((y - 3.0).abs() < 1e-6, "{y}");
    mel(&mut c, "select t2 base;");
    mel(&mut c, REMOVE);
    assert_eq!(mel(&mut c, "blendShape -q -wc bs;"), json!(2));
    let names = mel(&mut c, "aliasAttr -q bs;");
    assert!(!names.to_string().contains("t2"), "{names}");
}
