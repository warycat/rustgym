struct Solution;

impl Solution {
    fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n > 1 {
            0.5
        } else {
            1.0
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 1.0;
    assert_eq!(Solution::nth_person_gets_nth_seat(n), res);
    let n = 2;
    let res = 0.5;
    assert_eq!(Solution::nth_person_gets_nth_seat(n), res);
}
