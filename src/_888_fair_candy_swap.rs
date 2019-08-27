struct Solution;

use std::collections::HashSet;

impl Solution {
    fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let sum_a: i32 = a.iter().sum();
        let sum_b: i32 = b.iter().sum();
        let hs: HashSet<i32> = b.into_iter().collect();
        for x in a {
            let y = x + (sum_b - sum_a) / 2;
            if hs.contains(&y) {
                return vec![x, y];
            }
        }
        unreachable!();
    }
}

#[test]
fn test() {
    let a = vec![1, 2];
    let b = vec![2, 3];
    let res = vec![1, 2];
    assert_eq!(Solution::fair_candy_swap(a, b), res);
    let a = vec![2];
    let b = vec![1, 3];
    let res = vec![2, 3];
    assert_eq!(Solution::fair_candy_swap(a, b), res);
    let a = vec![1, 2, 5];
    let b = vec![2, 4];
    let res = vec![5, 4];
    assert_eq!(Solution::fair_candy_swap(a, b), res);
}
