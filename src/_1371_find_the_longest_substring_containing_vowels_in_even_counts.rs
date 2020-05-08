struct Solution;
use std::collections::HashMap;

impl Solution {
    fn find_the_longest_substring(s: String) -> i32 {
        let mut mask = 0;
        let mut hm: HashMap<u8, usize> = HashMap::new();
        hm.insert(0, 0);
        let mut res = 0;
        for (i, c) in s.char_indices() {
            if Self::vowel(c) != 0 {
                mask ^= 1 << Self::vowel(c);
            }
            hm.entry(mask).or_insert(i + 1);
            res = res.max(i + 1 - hm[&mask]);
        }
        res as i32
    }

    fn vowel(c: char) -> usize {
        match c {
            'a' => 1,
            'e' => 2,
            'i' => 3,
            'o' => 4,
            'u' => 5,
            _ => 0,
        }
    }
}

#[test]
fn test() {
    let s = "eleetminicoworoep".to_string();
    let res = 13;
    assert_eq!(Solution::find_the_longest_substring(s), res);
    let s = "leetcodeisgreat".to_string();
    let res = 5;
    assert_eq!(Solution::find_the_longest_substring(s), res);
    let s = "bcbcbc".to_string();
    let res = 6;
    assert_eq!(Solution::find_the_longest_substring(s), res);
}
