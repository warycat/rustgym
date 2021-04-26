struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut users: HashMap<i32, HashSet<i32>> = HashMap::new();
        let k = k as usize;
        let mut res = vec![0; k];
        for log in logs {
            let id = log[0];
            let time = log[1];
            users.entry(id).or_default().insert(time);
        }
        for times in users.values() {
            let j = times.len() - 1;
            if j < k {
                res[j] += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let logs = vec_vec_i32![[0, 5], [1, 2], [0, 2], [0, 5], [1, 3]];
    let k = 5;
    let res = vec![0, 2, 0, 0, 0];
    assert_eq!(Solution::finding_users_active_minutes(logs, k), res);
    let logs = vec_vec_i32![[1, 1], [2, 2], [2, 3]];
    let k = 4;
    let res = vec![1, 1, 0, 0];
    assert_eq!(Solution::finding_users_active_minutes(logs, k), res);
}
