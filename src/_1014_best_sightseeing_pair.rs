struct Solution;

impl Solution {
    fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut maxs = vec![];
        let mut prev_max = 0;
        let mut res = 0;
        for i in 0..n {
            prev_max = prev_max.max(i as i32 + a[i]);
            maxs.push(prev_max);
        }
        for i in 1..n {
            res = res.max(maxs[i - 1] + a[i] - i as i32);
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![8, 1, 5, 2, 6];
    let res = 11;
    assert_eq!(Solution::max_score_sightseeing_pair(a), res);
}
