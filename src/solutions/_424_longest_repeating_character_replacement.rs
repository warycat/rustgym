struct Solution;

impl Solution {
    fn character_replacement(s: String, k: i32) -> i32 {
        let mut count = vec![0; 26];
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        let mut res = 0;
        while end < n {
            if Self::sum(&count) <= k {
                count[(s[end] as u8 - b'A') as usize] += 1;
                end += 1;
            } else {
                count[(s[start] as u8 - b'A') as usize] -= 1;
                start += 1;
            }
            if Self::sum(&count) <= k {
                res = res.max(end - start);
            }
        }
        res as i32
    }

    fn sum(count: &[i32]) -> i32 {
        let max = count.iter().copied().max().unwrap();
        count.iter().sum::<i32>() - max
    }
}

#[test]
fn test() {
    let s = "ABAB".to_string();
    let k = 2;
    let res = 4;
    assert_eq!(Solution::character_replacement(s, k), res);
    let s = "AABABBA".to_string();
    let k = 1;
    let res = 4;
    assert_eq!(Solution::character_replacement(s, k), res);
}
