struct Solution;

use std::collections::HashSet;

impl Solution {
    fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut list = vec![HashSet::new(); n];
        for pair in pairs {
            let x = pair[0] as usize;
            let y = pair[1] as usize;
            for &u in &preferences[x] {
                let u = u as usize;
                if u != y {
                    list[x].insert(u);
                } else {
                    break;
                }
            }
            for &v in &preferences[y] {
                let v = v as usize;
                if v != x {
                    list[y].insert(v);
                } else {
                    break;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                if i != j && list[i].contains(&j) && list[j].contains(&i) {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 4;
    let preferences = vec_vec_i32![[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]];
    let pairs = vec_vec_i32![[0, 1], [2, 3]];
    let res = 2;
    assert_eq!(Solution::unhappy_friends(n, preferences, pairs), res);
    let n = 2;
    let preferences = vec_vec_i32![[1], [0]];
    let pairs = vec_vec_i32![[1, 0]];
    let res = 0;
    assert_eq!(Solution::unhappy_friends(n, preferences, pairs), res);
    let n = 4;
    let preferences = vec_vec_i32![[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]];
    let pairs = vec_vec_i32![[1, 3], [0, 2]];
    let res = 4;
    assert_eq!(Solution::unhappy_friends(n, preferences, pairs), res);
}
