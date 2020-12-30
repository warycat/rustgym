struct Solution;
use std::collections::HashSet;

impl Solution {
    fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut citys: HashSet<&str> = HashSet::new();
        let n = paths.len();
        for i in 0..n {
            citys.insert(&paths[i][0]);
        }
        for i in 0..n {
            if citys.insert(&paths[i][1]) {
                return paths[i][1].to_string();
            };
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let paths = vec_vec_string![
        ["London", "New York"],
        ["New York", "Lima"],
        ["Lima", "Sao Paulo"]
    ];
    let res = "Sao Paulo".to_string();
    assert_eq!(Solution::dest_city(paths), res);
    let paths = vec_vec_string![["B", "C"], ["D", "B"], ["C", "A"]];
    let res = "A".to_string();
    assert_eq!(Solution::dest_city(paths), res);
    let paths = vec_vec_string![["A", "Z"]];
    let res = "Z".to_string();
    assert_eq!(Solution::dest_city(paths), res);
}
