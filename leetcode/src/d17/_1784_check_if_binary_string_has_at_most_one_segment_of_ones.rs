struct Solution;

impl Solution {
    fn check_ones_segment(s: String) -> bool {
        let mut one = false;
        let mut count = 0;
        for c in s.chars() {
            if c == '1' {
                one = true;
            } else {
                if one {
                    count += 1;
                }
                one = false;
            }
        }
        if one {
            count += 1;
        }
        count <= 1
    }
}

#[test]
fn test() {
    let s = "1001".to_string();
    let res = false;
    assert_eq!(Solution::check_ones_segment(s), res);
    let s = "110".to_string();
    let res = true;
    assert_eq!(Solution::check_ones_segment(s), res);
}
