struct Solution;
use std::collections::HashMap;

impl Solution {
    fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        let n = houses.len();
        let k = k as usize;
        houses.sort_unstable();
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Self::dp(0, k, &mut memo, &houses, n)
    }

    fn dp(
        start: usize,
        k: usize,
        memo: &mut HashMap<(usize, usize), i32>,
        houses: &[i32],
        n: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&(start, k)) {
            return res;
        }
        let res = if k == 1 {
            Self::distance(start, n, houses)
        } else {
            let mut min = std::i32::MAX;
            for i in start..=n - k {
                let end = i + 1;
                let dist = Self::distance(start, end, houses);
                min = min.min(dist + Self::dp(end, k - 1, memo, houses, n));
            }
            min
        };
        memo.insert((start, k), res);
        res
    }

    fn distance(start: usize, end: usize, houses: &[i32]) -> i32 {
        let mut sum = 0;
        let median = (houses[(start + end - 1) / 2] + houses[(start + end) / 2]) / 2;
        for i in start..end {
            sum += (houses[i] - median).abs();
        }
        sum
    }
}

#[test]
fn test() {
    let houses = vec![1, 4, 8, 10, 20];
    let k = 3;
    let res = 5;
    assert_eq!(Solution::min_distance(houses, k), res);
    let houses = vec![2, 3, 5, 12, 18];
    let k = 2;
    let res = 9;
    assert_eq!(Solution::min_distance(houses, k), res);
    let houses = vec![7, 4, 6, 1];
    let k = 1;
    let res = 8;
    assert_eq!(Solution::min_distance(houses, k), res);
    let houses = vec![3, 6, 14, 10];
    let k = 4;
    let res = 0;
    assert_eq!(Solution::min_distance(houses, k), res);
}
