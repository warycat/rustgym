struct Solution;

impl Solution {
    fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = points.len();
        for i in 1..n {
            let x1 = points[i - 1][0];
            let y1 = points[i - 1][1];
            let x2 = points[i][0];
            let y2 = points[i][1];
            res += i32::max((x2 - x1).abs(), (y2 - y1).abs());
        }
        res
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[1, 1], [3, 4], [-1, 0]];
    let res = 7;
    assert_eq!(Solution::min_time_to_visit_all_points(points), res);
}
