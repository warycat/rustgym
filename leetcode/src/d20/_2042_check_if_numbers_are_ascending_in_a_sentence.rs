struct Solution;

impl Solution {
    fn are_numbers_ascending(s: String) -> bool {
        let mut last = 0;
        for word in s.split_whitespace() {
            if let Ok(x) = word.parse::<i32>() {
                if x <= last {
                    return false;
                } else {
                    last = x;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string();
    let res = true;
    assert_eq!(Solution::are_numbers_ascending(s), res);
    let s = "hello world 5 x 5".to_string();
    let res = false;
    assert_eq!(Solution::are_numbers_ascending(s), res);
    let s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string();
    let res = false;
    assert_eq!(Solution::are_numbers_ascending(s), res);
}
