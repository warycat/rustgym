struct Solution;

impl Solution {
    fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
        let l = to_be_removed[0];
        let r = to_be_removed[1];
        let mut res = vec![];
        for interval in intervals {
            if interval[1] < l || interval[0] > r {
                res.push(interval);
            } else {
                if interval[0] < l {
                    res.push(vec![interval[0], l.min(interval[1])]);
                }
                if interval[1] > r {
                    res.push(vec![r.max(interval[0]), interval[1]]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let intervals = vec_vec_i32![[0, 2], [3, 4], [5, 7]];
    let to_be_removed = vec![1, 6];
    let res = vec_vec_i32![[0, 1], [6, 7]];
    assert_eq!(Solution::remove_interval(intervals, to_be_removed), res);
    let intervals = vec_vec_i32![[0, 5]];
    let to_be_removed = vec![2, 3];
    let res = vec_vec_i32![[0, 2], [3, 5]];
    assert_eq!(Solution::remove_interval(intervals, to_be_removed), res);
}
