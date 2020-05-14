struct Solution;

impl Solution {
    fn find_permutation(s: String) -> Vec<i32> {
        let n = s.len();
        let mut res: Vec<i32> = (1..=(n + 1) as i32).collect();
        let s: Vec<char> = s.chars().collect();
        let mut l = 0;
        while l < n {
            if s[l] == 'D' {
                let mut r = l;
                while r < n && s[r] == 'D' {
                    r += 1;
                }
                res[l..=r].reverse();
                l = r;
            } else {
                l += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "I".to_string();
    let res = vec![1, 2];
    assert_eq!(Solution::find_permutation(s), res);
    let s = "DI".to_string();
    let res = vec![2, 1, 3];
    assert_eq!(Solution::find_permutation(s), res);
}
