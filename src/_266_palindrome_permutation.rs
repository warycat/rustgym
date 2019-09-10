struct Solution;

impl Solution {
    fn can_permute_palindrome(s: String) -> bool {
        let mut count: Vec<usize> = vec![0; 256];
        for b in s.bytes() {
            let d = b as usize;
            if count[d] == 1 {
                count[d] = 0;
            } else {
                count[d] = 1;
            }
        }
        count.iter().sum::<usize>() <= 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_permute_palindrome("code".to_string()), false);
    assert_eq!(Solution::can_permute_palindrome("aab".to_string()), true);
    assert_eq!(
        Solution::can_permute_palindrome("carerac".to_string()),
        true
    );
}
