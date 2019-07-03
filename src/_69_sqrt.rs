struct Solution {}

impl Solution {
    fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt().floor() as i32
    }
}
