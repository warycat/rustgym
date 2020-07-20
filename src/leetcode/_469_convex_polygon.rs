struct Solution;

impl Solution {
    fn is_convex(points: Vec<Vec<i32>>) -> bool {
        let n = points.len();
        let mut positive = 0;
        let mut negative = 0;
        for i in 0..n {
            let sign = Self::cross(&points[i], &points[(i + 1) % n], &points[(i + 2) % n]);
            if sign >= 0 {
                positive += 1;
            }
            if sign <= 0 {
                negative += 1;
            }
        }
        positive == n || negative == n
    }

    fn cross(a: &[i32], b: &[i32], c: &[i32]) -> i32 {
        (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0])
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[0, 0], [0, 1], [1, 1], [1, 0]];
    let res = true;
    assert_eq!(Solution::is_convex(points), res);
    let points = vec_vec_i32![[0, 0], [0, 10], [10, 10], [10, 0], [5, 5]];
    let res = false;
    assert_eq!(Solution::is_convex(points), res);
}
