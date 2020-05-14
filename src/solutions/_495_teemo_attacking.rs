struct Solution;

impl Solution {
    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let n = time_series.len();
        if n == 0 {
            return 0;
        }
        let mut start = time_series[0];
        let mut res = 0;
        for i in 1..n {
            let end = time_series[i];
            if start + duration > end {
                res += end - start;
            } else {
                res += duration;
            }
            start = end;
        }
        res += duration;
        res
    }
}

#[test]
fn test() {
    let time_series = vec![1, 4];
    let duration = 2;
    let res = 4;
    assert_eq!(Solution::find_poisoned_duration(time_series, duration), res);
    let time_series = vec![1, 2];
    let duration = 2;
    let res = 3;
    assert_eq!(Solution::find_poisoned_duration(time_series, duration), res);
}
