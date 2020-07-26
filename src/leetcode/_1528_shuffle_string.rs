struct Solution;

impl Solution {
    fn restore_string(s: String, indices: Vec<i32>) -> String {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut v = vec![' '; n];
        for i in 0..n {
            v[indices[i] as usize] = s[i];
        }
        v.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "codeleet".to_string();
    let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];
    let res = "leetcode".to_string();
    assert_eq!(Solution::restore_string(s, indices), res);
    let s = "abc".to_string();
    let indices = vec![0, 1, 2];
    let res = "abc".to_string();
    assert_eq!(Solution::restore_string(s, indices), res);
    let s = "aiohn".to_string();
    let indices = vec![3, 1, 4, 2, 0];
    let res = "nihao".to_string();
    assert_eq!(Solution::restore_string(s, indices), res);
    let s = "aaiougrt".to_string();
    let indices = vec![4, 0, 2, 6, 7, 3, 1, 5];
    let res = "arigatou".to_string();
    assert_eq!(Solution::restore_string(s, indices), res);
    let s = "art".to_string();
    let indices = vec![1, 0, 2];
    let res = "rat".to_string();
    assert_eq!(Solution::restore_string(s, indices), res);
}
