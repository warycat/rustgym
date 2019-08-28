struct Solution;

impl Solution {
    fn check_record(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        if s.iter().filter(|&c| c == &'A').count() > 1 {
            return false;
        }
        if s.windows(3).filter(|&w| w == ['L', 'L', 'L']).count() > 0 {
            return false;
        }
        true
    }
}

#[test]
fn test() {
    let s = "PPALLP".to_string();
    assert_eq!(Solution::check_record(s), true);
    let s = "PPALLLP".to_string();
    assert_eq!(Solution::check_record(s), false);
}
