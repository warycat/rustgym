struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = vec![];
        let n = start_time.len();
        for i in 0..n {
            jobs.push((start_time[i], end_time[i], profit[i]));
        }
        jobs.sort_unstable();
        let mut memo: BTreeMap<i32, i32> = BTreeMap::new();
        let mut res = 0;
        for i in (0..n).rev() {
            let mut cur = jobs[i].2;
            if let Some((_, val)) = memo.range(jobs[i].1..).next() {
                cur += val;
            }
            res = res.max(cur);
            memo.insert(jobs[i].0, res);
        }
        res
    }
}

#[test]
fn test() {
    let start_time = vec![1, 2, 3, 3];
    let end_time = vec![3, 4, 5, 6];
    let profit = vec![50, 10, 40, 70];
    let res = 120;
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), res);
    let start_time = vec![1, 2, 3, 4, 6];
    let end_time = vec![3, 5, 10, 6, 9];
    let profit = vec![20, 20, 100, 70, 60];
    let res = 150;
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), res);
    let start_time = vec![1, 1, 1];
    let end_time = vec![2, 3, 4];
    let profit = vec![5, 6, 4];
    let res = 6;
    assert_eq!(Solution::job_scheduling(start_time, end_time, profit), res);
}
