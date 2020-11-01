struct Solution;

use std::collections::HashSet;

impl Solution {
    fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut x_set: HashSet<i32> = HashSet::new();
        for point in points {
            x_set.insert(point[0]);
        }
        let mut x_arr: Vec<i32> = x_set.into_iter().collect();
        x_arr.sort_unstable();
        let mut res = 0;
        for w in x_arr.windows(2) {
            res = res.max(w[1] - w[0]);
        }
        res
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[8, 7], [9, 9], [7, 4], [9, 7]];
    let res = 1;
    assert_eq!(Solution::max_width_of_vertical_area(points), res);
    let points = vec_vec_i32![[3, 1], [9, 0], [1, 0], [1, 4], [5, 3], [8, 8]];
    let res = 3;
    assert_eq!(Solution::max_width_of_vertical_area(points), res);
}
