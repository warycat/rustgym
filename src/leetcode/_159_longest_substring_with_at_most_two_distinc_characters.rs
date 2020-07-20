struct Solution;

impl Solution {
    fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut count: Vec<usize> = vec![0; 256];
        let mut start = 0;
        let mut end = 0;
        let mut res = 0;
        let n = s.len();
        let mut sum = 0;
        while end < n {
            if sum <= 2 {
                count[s[end] as usize] += 1;
                if count[s[end] as usize] == 1 {
                    sum += 1;
                }
                end += 1;
            } else {
                count[s[start] as usize] -= 1;
                if count[s[start] as usize] == 0 {
                    sum -= 1;
                }
                start += 1;
            }
            if sum <= 2 {
                res = res.max(end - start);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "eceba".to_string();
    let res = 3;
    assert_eq!(Solution::length_of_longest_substring_two_distinct(s), res);
    let s = "ccaabbb".to_string();
    let res = 5;
    assert_eq!(Solution::length_of_longest_substring_two_distinct(s), res);
}
