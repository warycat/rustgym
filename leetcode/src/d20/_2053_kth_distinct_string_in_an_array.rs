struct Solution;

use std::collections::HashMap;

impl Solution {
    fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut hm: HashMap<String, usize> = HashMap::new();
        let n = arr.len();
        for i in 0..n {
            *hm.entry(arr[i].clone()).or_default() += 1;
        }
        let mut j = 0;
        for i in 0..n {
            if hm[&arr[i]] == 1 {
                j += 1;
            }
            if j == k {
                return arr[i].clone();
            }
        }

        "".to_string()
    }
}

#[test]
fn test() {
    let arr = vec_string!["d", "b", "c", "b", "c", "a"];
    let k = 2;
    let res = "a".to_string();
    assert_eq!(Solution::kth_distinct(arr, k), res);
    let arr = vec_string!["aaa", "aa", "a"];
    let k = 1;
    let res = "aaa".to_string();
    assert_eq!(Solution::kth_distinct(arr, k), res);
    let arr = vec_string!["a", "b", "a"];
    let k = 3;
    let res = "".to_string();
    assert_eq!(Solution::kth_distinct(arr, k), res);
}
