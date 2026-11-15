//! Issue 53: what the Outliner's Display menu asks of the list: Assigned Materials, Set Members, Ignore 'Hidden In
//! Outliner'.

use aztek_host::Core;
use serde_json::Value;

fn find<'a>(list: &'a Value, name: &str) -> Option<&'a Value> {
    list.as_array()?.iter().find_map(|n| if n["name"] == name { Some(n) } else { find(&n["children"], name) })
}

#[test]
fn display_menu_words_change_the_list() {
    let mut c = Core::new();
    c.mel_checked("polyCube -n a; polyCube -n b; sets -n s1 a; setAttr b.hiddenInOutliner 1;").unwrap();
    let plain = aztek_host::query::outliner(&c, "shapes");
    assert!(find(&plain, "lambert1").is_none());
    assert!(find(&plain, "s1").unwrap()["children"].as_array().unwrap().is_empty());
    assert!(find(&plain, "b").is_none());
    let asked = aztek_host::query::outliner(&c, "shapes materials setMembers showHidden");
    let shape = find(&asked, "aShape").unwrap();
    assert!(shape["children"].as_array().unwrap().iter().any(|k| k["name"] == "lambert1"), "{shape}");
    assert_eq!(find(&asked, "s1").unwrap()["children"][0]["name"], "a");
    assert!(find(&asked, "b").is_some());
}
