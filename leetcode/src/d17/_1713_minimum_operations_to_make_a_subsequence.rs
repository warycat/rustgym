struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let n = target.len();
        let mut idx: HashMap<i32, usize> = HashMap::new();
        for x in target {
            idx.insert(x, idx.len());
        }
        let mut dp = vec![];
        for x in arr {
            if let Some(&i) = idx.get(&x) {
                let j = match dp.binary_search(&i) {
                    Ok(j) => j,
                    Err(j) => j,
                };
                if j < dp.len() {
                    dp[j] = i;
                } else {
                    dp.push(i);
                }
            }
        }
        (n - dp.len()) as i32
    }
}

#[test]
fn test() {
    let target = vec![5, 1, 3];
    let arr = vec![9, 4, 2, 3, 4];
    let res = 2;
    assert_eq!(Solution::min_operations(target, arr), res);
    let target = vec![6, 4, 8, 1, 3, 2];
    let arr = vec![4, 7, 6, 2, 3, 8, 6, 1];
    let res = 3;
    assert_eq!(Solution::min_operations(target, arr), res);
}
