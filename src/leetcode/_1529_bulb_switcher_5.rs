struct Solution;

impl Solution {
    fn min_flips(target: String) -> i32 {
        let n = target.len();
        let s: Vec<char> = target.chars().collect();
        let mut res = 0;
        let mut i = 0;
        let mut prev = '0';
        loop {
            while i < n && s[i] == prev {
                i += 1;
            }
            if i == n {
                break;
            }
            prev = s[i];
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let target = "10111".to_string();
    let res = 3;
    assert_eq!(Solution::min_flips(target), res);
    let target = "101".to_string();
    let res = 3;
    assert_eq!(Solution::min_flips(target), res);
    let target = "00000".to_string();
    let res = 0;
    assert_eq!(Solution::min_flips(target), res);
    let target = "001011101".to_string();
    let res = 5;
    assert_eq!(Solution::min_flips(target), res);
}
