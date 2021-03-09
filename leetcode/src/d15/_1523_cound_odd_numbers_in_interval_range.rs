struct Solution;

impl Solution {
    fn count_odds(low: i32, high: i32) -> i32 {
        let l = low / 2 * 2;
        let r = (high + 1) / 2 * 2;
        (r - l) / 2
    }
}

#[test]
fn test() {
    let low = 3;
    let high = 7;
    let res = 3;
    assert_eq!(Solution::count_odds(low, high), res);
    let low = 8;
    let high = 10;
    let res = 1;
    assert_eq!(Solution::count_odds(low, high), res);
}
