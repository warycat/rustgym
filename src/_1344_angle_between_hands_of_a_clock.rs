struct Solution;

impl Solution {
    fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let h = ((hour % 12) as f64 + (minutes as f64 / 60.0)) * 30.0;
        let m = minutes as f64 * 6.0;
        let a = (h - m).abs();
        if a > 180.0 {
            360.0 - a
        } else {
            a
        }
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let hour = 12;
    let minutes = 30;
    let res = 165.0;
    assert_approx_eq!(Solution::angle_clock(hour, minutes), res);
    let hour = 3;
    let minutes = 30;
    let res = 75.0;
    assert_approx_eq!(Solution::angle_clock(hour, minutes), res);
    let hour = 3;
    let minutes = 15;
    let res = 7.5;
    assert_approx_eq!(Solution::angle_clock(hour, minutes), res);
    let hour = 4;
    let minutes = 50;
    let res = 155.0;
    assert_approx_eq!(Solution::angle_clock(hour, minutes), res);
    let hour = 12;
    let minutes = 0;
    let res = 0.0;
    assert_approx_eq!(Solution::angle_clock(hour, minutes), res);
    let hour = 1;
    let minutes = 57;
    let res = 76.5;
    assert_approx_eq!(Solution::angle_clock(hour, minutes), res);
}
