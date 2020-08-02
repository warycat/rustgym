struct Solution;

impl Solution {
    fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for interval in intervals {
            if interval[0] < new_interval[0] {
                if interval[1] < new_interval[0] {
                    res.push(interval);
                } else {
                    new_interval[0] = new_interval[0].min(interval[0]);
                    new_interval[1] = new_interval[1].max(interval[1]);
                }
            } else {
                if interval[0] > new_interval[1] {
                    res.push(interval);
                } else {
                    new_interval[0] = new_interval[0].min(interval[0]);
                    new_interval[1] = new_interval[1].max(interval[1]);
                }
            }
        }
        if let Err(i) = res.binary_search_by_key(&new_interval[0], |v| v[0]) {
            res.insert(i, new_interval);
        }
        res
    }
}

#[test]
fn test() {
    let intervals = vec_vec_i32![[1, 3], [6, 9]];
    let new_interval = vec![2, 5];
    let res = vec_vec_i32![[1, 5], [6, 9]];
    assert_eq!(Solution::insert(intervals, new_interval), res);
}
