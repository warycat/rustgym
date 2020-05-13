struct Solution;

impl Solution {
    fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        for &w in &weights {
            sum += w;
            max = max.max(w);
        }
        let mut l = max;
        let mut h = sum;
        while l < h {
            let m = l + (h - l) / 2;
            if Self::days(m, &weights) <= d {
                h = m;
            } else {
                l = m + 1;
            }
        }
        l
    }

    fn days(capacity: i32, weights: &[i32]) -> i32 {
        let mut cur = 0;
        let mut res = 1;
        for &w in weights {
            cur += w;
            if cur > capacity {
                res += 1;
                cur = w;
            }
        }
        res
    }
}

#[test]
fn test() {
    let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let d = 5;
    let res = 15;
    assert_eq!(Solution::ship_within_days(weights, d), res);
    let weights = vec![3, 2, 2, 4, 1, 4];
    let d = 3;
    let res = 6;
    assert_eq!(Solution::ship_within_days(weights, d), res);
    let weights = vec![1, 2, 3, 1, 1];
    let d = 4;
    let res = 3;
    assert_eq!(Solution::ship_within_days(weights, d), res);
}
