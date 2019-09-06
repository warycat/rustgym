struct Solution;

impl Solution {
    fn number_of_days(y: i32, m: i32) -> i32 {
        let days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if m == 2 {
            if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
                1 + days[1]
            } else {
                days[1]
            }
        } else {
            days[m as usize - 1]
        }
    }
}

#[test]
fn test() {
    let y = 1992;
    let m = 7;
    let res = 31;
    assert_eq!(Solution::number_of_days(y, m), res);
    let y = 2000;
    let m = 2;
    let res = 29;
    assert_eq!(Solution::number_of_days(y, m), res);
    let y = 1900;
    let m = 2;
    let res = 28;
    assert_eq!(Solution::number_of_days(y, m), res);
}
