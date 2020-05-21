struct Solution;

impl Solution {
    fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut res: f64 = 0f64;
        for i in &points {
            for j in &points {
                for k in &points {
                    let area = (i[0] * j[1] + j[0] * k[1] + k[0] * i[1]
                        - j[0] * i[1]
                        - k[0] * j[1]
                        - i[0] * k[1])
                        .abs();
                    let area = area as f64 / 2f64;
                    res = f64::max(area, res);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let points: Vec<Vec<i32>> = vec_vec_i32![[0, 0], [0, 1], [1, 0], [0, 2], [2, 0]];
    assert_approx_eq!(Solution::largest_triangle_area(points), 2f64);
}
