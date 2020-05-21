struct Solution;

impl Solution {
    fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len() + 1;
        let nums: Vec<i32> = (0..n).map(|i| i as i32).collect();
        let mut res: Vec<i32> = vec![];
        let mut l = 0;
        let mut r = n - 1;
        for c in s.chars() {
            match c {
                'I' => {
                    res.push(nums[l]);
                    l += 1;
                }
                'D' => {
                    res.push(nums[r]);
                    r -= 1;
                }
                _ => (),
            }
        }
        res.push(nums[l]);
        res
    }
}

#[test]
fn test() {
    let s = "IDID".to_string();
    let res = vec![0, 4, 1, 3, 2];
    assert_eq!(Solution::di_string_match(s), res);
    let s = "III".to_string();
    let res = vec![0, 1, 2, 3];
    assert_eq!(Solution::di_string_match(s), res);
    let s = "DDI".to_string();
    let res = vec![3, 2, 0, 1];
    assert_eq!(Solution::di_string_match(s), res);
}
