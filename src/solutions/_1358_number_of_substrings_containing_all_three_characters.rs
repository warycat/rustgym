struct Solution;

impl Solution {
    fn number_of_substrings(s: String) -> i32 {
        let mut count: [usize; 3] = [0; 3];
        let s: Vec<u8> = s.bytes().collect();
        let n = s.len();
        let mut j = 0;
        let mut res = 0;
        for i in 0..n {
            count[(s[i] - b'a') as usize] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                count[(s[j] - b'a') as usize] -= 1;
                j += 1;
            }
            res += j;
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "abcabc".to_string();
    let res = 10;
    assert_eq!(Solution::number_of_substrings(s), res);
    let s = "aaacb".to_string();
    let res = 3;
    assert_eq!(Solution::number_of_substrings(s), res);
    let s = "abc".to_string();
    let res = 1;
    assert_eq!(Solution::number_of_substrings(s), res);
}
