struct Solution;

impl Solution {
    fn count_binary_substrings(s: String) -> i32 {
        let mut prev: usize = 0;
        let mut curr: usize = 0;
        let mut curr_c: Option<char> = None;
        let mut res = 0;
        for c in s.chars() {
            if let Some(cc) = curr_c {
                if cc == c {
                    curr += 1;
                } else {
                    res += usize::min(prev, curr);
                    curr_c = Some(c);
                    prev = curr;
                    curr = 1;
                }
            } else {
                curr_c = Some(c);
                curr = 1;
            }
        }
        res += usize::min(prev, curr);
        res as i32
    }
}

#[test]
fn test() {
    let s = "00110011".to_string();
    assert_eq!(Solution::count_binary_substrings(s), 6);
    let s = "10101".to_string();
    assert_eq!(Solution::count_binary_substrings(s), 4);
}
