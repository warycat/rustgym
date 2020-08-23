struct Solution;
use rand::prelude::*;

impl Solution {
    fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let mut rng = thread_rng();
        let mut x = 50.0;
        let mut y = 50.0;
        let mut r = 50.0;
        let mut prev = Self::dist(&positions, x, y);
        for _ in 0..25 {
            for _ in 0..25 {
                let xx = x + rng.gen_range(-1.0, 1.0) * r;
                let yy = y + rng.gen_range(-1.0, 1.0) * r;
                let next = Self::dist(&positions, xx, yy);
                if next < prev {
                    prev = next;
                    x = xx;
                    y = yy;
                }
            }
            r *= 0.5;
        }
        prev
    }

    fn dist(positions: &[Vec<i32>], x: f64, y: f64) -> f64 {
        positions
            .iter()
            .map(|v| (((v[0] as f64 - x).powi(2) + (v[1] as f64 - y).powi(2)).sqrt()))
            .sum::<f64>()
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let positions = vec_vec_i32![[0, 1], [1, 0], [1, 2], [2, 1]];
    let res = 4.0;
    assert_approx_eq!(Solution::get_min_dist_sum(positions), res, 1.0e-5);
    let positions = vec_vec_i32![[1, 1], [3, 3]];
    let res = 2.82843;
    assert_approx_eq!(Solution::get_min_dist_sum(positions), res, 1.0e-5);
    let positions = vec_vec_i32![[1, 1]];
    let res = 0.0;
    assert_approx_eq!(Solution::get_min_dist_sum(positions), res, 1.0e-5);
    let positions = vec_vec_i32![[1, 1], [0, 0], [2, 0]];
    let res = 2.73205;
    assert_approx_eq!(Solution::get_min_dist_sum(positions), res, 1.0e-5);
    let positions = vec_vec_i32![[0, 1], [3, 2], [4, 5], [7, 6], [8, 9], [11, 1], [2, 12]];
    let res = 32.94036;
    assert_approx_eq!(Solution::get_min_dist_sum(positions), res, 1.0e-5);
}
