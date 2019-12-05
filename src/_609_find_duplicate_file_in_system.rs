struct Solution;
use std::collections::HashMap;

impl Solution {
    fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();
        for s in paths {
            let mut s_iter = s.split_whitespace();
            let dir: &str = s_iter.next().unwrap();
            for f in s_iter {
                let mut f_iter = f.chars();
                let name: String = f_iter.by_ref().take_while(|&c| c != '(').collect();
                let content: String = f_iter.by_ref().take_while(|&c| c != ')').collect();
                hm.entry(content)
                    .or_default()
                    .push(format!("{}/{}", dir, name))
            }
        }
        hm.into_iter()
            .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
            .collect()
    }
}

#[test]
fn test() {
    let paths: Vec<String> = [
        "root/a 1.txt(abcd) 2.txt(efgh)",
        "root/c 3.txt(abcd)",
        "root/c/d 4.txt(efgh)",
        "root 4.txt(efgh)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let mut res: Vec<Vec<String>> = vec![
        vec![
            "root/a/2.txt".to_string(),
            "root/c/d/4.txt".to_string(),
            "root/4.txt".to_string(),
        ],
        vec!["root/a/1.txt".to_string(), "root/c/3.txt".to_string()],
    ];
    assert_eq!(Solution::find_duplicate(paths).sort(), res.sort());
}
