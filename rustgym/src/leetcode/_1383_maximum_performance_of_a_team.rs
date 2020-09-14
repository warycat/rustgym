struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut max_efficiency: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for i in 0..n {
            max_efficiency.push((efficiency[i], speed[i]));
        }
        let mut min_speed: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut sum_speed = 0;
        let mut res = 0;
        while let Some((e, s)) = max_efficiency.pop() {
            sum_speed += s as i64;
            min_speed.push(Reverse(s));
            if min_speed.len() > k {
                sum_speed -= min_speed.pop().unwrap().0 as i64;
            }
            res = res.max(sum_speed as i64 * e as i64);
        }
        (res % MOD) as i32
    }
}

#[test]
fn test() {
    let n = 6;
    let speed = vec![2, 10, 3, 1, 5, 8];
    let efficiency = vec![5, 4, 3, 9, 7, 2];
    let k = 2;
    let res = 60;
    assert_eq!(Solution::max_performance(n, speed, efficiency, k), res);
    let n = 6;
    let speed = vec![2, 10, 3, 1, 5, 8];
    let efficiency = vec![5, 4, 3, 9, 7, 2];
    let k = 3;
    let res = 68;
    assert_eq!(Solution::max_performance(n, speed, efficiency, k), res);
    let n = 6;
    let speed = vec![2, 10, 3, 1, 5, 8];
    let efficiency = vec![5, 4, 3, 9, 7, 2];
    let k = 4;
    let res = 72;
    assert_eq!(Solution::max_performance(n, speed, efficiency, k), res);
}
