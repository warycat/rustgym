struct Solution;

impl Solution {
    fn minimum_moves(s: String) -> i32 {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut res = 0;
        for i in 0..n {
            if s[i] == 'X' {
                res += 1;
                s[i] = 'O';
                if i + 1 < n {
                    s[i + 1] = 'O';
                }
                if i + 2 < n {
                    s[i + 2] = 'O';
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "XXX".to_string();
    let res = 1;
    assert_eq!(Solution::minimum_moves(s), res);
    let s = "XXOX".to_string();
    let res = 2;
    assert_eq!(Solution::minimum_moves(s), res);
    let s = "OOOO".to_string();
    let res = 0;
    assert_eq!(Solution::minimum_moves(s), res);
}
