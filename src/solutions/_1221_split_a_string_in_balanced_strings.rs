struct Solution;

impl Solution {
    fn balanced_string_split(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut res = 0;
        for c in s.chars() {
            match c {
                'R' => r += 1,
                'L' => l += 1,
                _ => {}
            }
            if l == r {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "RLRRLLRLRL".to_string();
    assert_eq!(Solution::balanced_string_split(s), 4);
    let s = "RLLLLRRRLR".to_string();
    assert_eq!(Solution::balanced_string_split(s), 3);
    let s = "LLLLRRRR".to_string();
    assert_eq!(Solution::balanced_string_split(s), 1);
}
