struct Solution;

impl Solution {
    fn get_lucky(s: String, k: i32) -> i32 {
        let mut x = "".to_string();
        for c in s.bytes() {
            x.push_str(&format!("{}", c - b'a' + 1));
        }
        for _ in 0..k {
            x = Solution::transform(x);
        }
        x.parse::<i32>().unwrap()
    }

    fn transform(s: String) -> String {
        let mut res = 0;
        for c in s.bytes() {
            res += (c - b'0') as i32;
        }
        format!("{}", res)
    }
}

#[test]
fn test() {
    let s = "iiii".to_string();
    let k = 1;
    let res = 36;
    assert_eq!(Solution::get_lucky(s, k), res);
    let s = "leetcode".to_string();
    let k = 2;
    let res = 6;
    assert_eq!(Solution::get_lucky(s, k), res);
    let s = "zbax".to_string();
    let k = 2;
    let res = 8;
    assert_eq!(Solution::get_lucky(s, k), res);
}
