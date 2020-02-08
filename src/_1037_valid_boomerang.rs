struct Solution;

impl Solution {
    fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        points[0][0] * (points[1][1] - points[2][1])
            + points[1][0] * (points[2][1] - points[0][1])
            + points[2][0] * (points[0][1] - points[1][1])
            != 0
    }
}

#[test]
fn test() {
    let points: Vec<Vec<i32>> = vec_vec_i32![[1, 1], [2, 3], [3, 2]];
    assert_eq!(Solution::is_boomerang(points), true);
    let points: Vec<Vec<i32>> = vec_vec_i32![[1, 1], [2, 2], [3, 3]];
    assert_eq!(Solution::is_boomerang(points), false);
}
