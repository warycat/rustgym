struct Solution;

impl Solution {
    fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        let n = intervals.len();
        intervals.sort_by_key(|v| v[1]);
        let mut end = intervals[0][1];
        let mut res = 0;
        for i in 1..n {
            if intervals[i][0] < end {
                res += 1;
            } else {
                end = intervals[i][1];
            }
        }
        res
    }
}

#[test]
fn test() {
    let intervals = vec_vec_i32![[1, 2], [2, 3], [3, 4], [1, 3]];
    let res = 1;
    assert_eq!(Solution::erase_overlap_intervals(intervals), res);
    let intervals = vec_vec_i32![[1, 2], [1, 2], [1, 2]];
    let res = 2;
    assert_eq!(Solution::erase_overlap_intervals(intervals), res);
    let intervals = vec_vec_i32![[1, 2], [2, 3]];
    let res = 0;
    assert_eq!(Solution::erase_overlap_intervals(intervals), res);
}
