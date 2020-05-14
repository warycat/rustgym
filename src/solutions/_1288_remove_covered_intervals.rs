struct Solution;
use std::cmp::Reverse;

impl Solution {
    fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|v| (v[0], Reverse(v[1])));
        let n = intervals.len();
        let mut r = -1;
        let mut res = 0;
        for i in 0..n {
            let interval = &intervals[i];
            if interval[1] <= r {
                continue;
            } else {
                r = interval[1];
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let intervals = vec_vec_i32![[1, 4], [3, 6], [2, 8]];
    let res = 2;
    assert_eq!(Solution::remove_covered_intervals(intervals), res);
}
