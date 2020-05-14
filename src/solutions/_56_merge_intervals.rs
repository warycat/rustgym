struct Solution;

impl Solution {
    fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut res: Vec<Vec<i32>> = vec![];
        let mut temp: Option<Vec<i32>> = None;
        for v in intervals {
            if let Some(t) = temp.clone() {
                if v[0] <= t[1] {
                    temp = Some(vec![t[0], i32::max(t[1], v[1])]);
                } else {
                    temp = Some(v);
                    res.push(t);
                }
            } else {
                temp = Some(v);
            }
        }
        if let Some(t) = temp {
            res.push(t);
        }
        res
    }
}

#[test]
fn test() {
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[1, 3], [2, 6], [8, 10], [15, 18]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 6], [8, 10], [15, 18]];
    assert_eq!(Solution::merge(intervals), res);
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[1, 4], [4, 5]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 5]];
    assert_eq!(Solution::merge(intervals), res);
}
