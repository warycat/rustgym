struct Solution;

impl Solution {
    fn min_steps(s: String, t: String) -> i32 {
        let mut count: Vec<i32> = vec![0; 26];
        for c in s.chars() {
            let i = (c as u32 - 'a' as u32) as usize;
            count[i] += 1;
        }
        for c in t.chars() {
            let i = (c as u32 - 'a' as u32) as usize;
            count[i] -= 1;
        }
        count.iter().map(|x| x.abs()).sum::<i32>() / 2
    }
}

#[test]
fn test() {
    let s = "bab".to_string();
    let t = "aba".to_string();
    let res = 1;
    assert_eq!(Solution::min_steps(s, t), res);
    let s = "leetcode".to_string();
    let t = "practice".to_string();
    let res = 5;
    assert_eq!(Solution::min_steps(s, t), res);
    let s = "anagram".to_string();
    let t = "mangaar".to_string();
    let res = 0;
    assert_eq!(Solution::min_steps(s, t), res);
    let s = "xxyyzz".to_string();
    let t = "xxyyzz".to_string();
    let res = 0;
    assert_eq!(Solution::min_steps(s, t), res);
    let s = "friend".to_string();
    let t = "family".to_string();
    let res = 4;
    assert_eq!(Solution::min_steps(s, t), res);
}
