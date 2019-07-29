struct Solution;

impl Solution {
    fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        for i in 1..intervals.len() {
            if intervals[i][0] < intervals[i - 1][1] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    assert_eq!(Solution::can_attend_meetings(intervals), false);
    let intervals = vec![vec![7, 10], vec![2, 4]];
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}
