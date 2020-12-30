struct Solution;

impl Solution {
    fn check_inclusion(s1: String, s2: String) -> bool {
        let mut c1: [usize; 26] = [0; 26];
        let mut c2: [usize; 26] = [0; 26];
        let n1 = s1.len();
        let n2 = s2.len();
        let s1: Vec<u8> = s1.bytes().collect();
        let s2: Vec<u8> = s2.bytes().collect();
        if n1 > n2 {
            return false;
        }
        for i in 0..n1 {
            c1[(s1[i] - b'a') as usize] += 1;
            c2[(s2[i] - b'a') as usize] += 1;
        }
        if c1 == c2 {
            return true;
        }
        for i in n1..n2 {
            c2[(s2[i] - b'a') as usize] += 1;
            c2[(s2[i - n1] - b'a') as usize] -= 1;
            if c1 == c2 {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let s1 = "ab".to_string();
    let s2 = "eidbaooo".to_string();
    let res = true;
    assert_eq!(Solution::check_inclusion(s1, s2), res);
    let s1 = "ab".to_string();
    let s2 = "eidboaoo".to_string();
    let res = false;
    assert_eq!(Solution::check_inclusion(s1, s2), res);
}
