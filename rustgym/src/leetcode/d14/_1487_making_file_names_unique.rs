struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut hs: HashSet<String> = HashSet::new();
        let mut hm: HashMap<String, usize> = HashMap::new();
        let mut res = vec![];
        'outer: for name in names {
            if !hs.insert(name.to_string()) {
                let mut i = *hm.get(&name).unwrap_or(&1);
                loop {
                    let new_name = format!("{}({})", name, i);
                    if hs.insert(new_name.to_string()) {
                        res.push(new_name);
                        hm.insert(name, i + 1);
                        continue 'outer;
                    }
                    i += 1;
                }
            } else {
                res.push(name);
            }
        }
        res
    }
}

#[test]
fn test() {
    let names = vec_string!["pes", "fifa", "gta", "pes(2019)"];
    let res = vec_string!["pes", "fifa", "gta", "pes(2019)"];
    assert_eq!(Solution::get_folder_names(names), res);
    let names = vec_string!["gta", "gta(1)", "gta", "avalon"];
    let res = vec_string!["gta", "gta(1)", "gta(2)", "avalon"];
    assert_eq!(Solution::get_folder_names(names), res);
    let names = vec_string![
        "onepiece",
        "onepiece(1)",
        "onepiece(2)",
        "onepiece(3)",
        "onepiece"
    ];
    let res = vec_string![
        "onepiece",
        "onepiece(1)",
        "onepiece(2)",
        "onepiece(3)",
        "onepiece(4)"
    ];
    assert_eq!(Solution::get_folder_names(names), res);
    let names = vec_string!["wano", "wano", "wano", "wano"];
    let res = vec_string!["wano", "wano(1)", "wano(2)", "wano(3)"];
    assert_eq!(Solution::get_folder_names(names), res);
    let names = vec_string!["kaido", "kaido(1)", "kaido", "kaido(1)"];
    let res = vec_string!["kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"];
    assert_eq!(Solution::get_folder_names(names), res);
}
