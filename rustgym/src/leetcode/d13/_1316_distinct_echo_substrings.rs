struct Solution;

use std::collections::HashSet;

const MOD: i64 = 1_000_000_007;
const PRIME: i64 = 29;

impl Solution {
    fn distinct_echo_substrings(text: String) -> i32 {
        let text: Vec<u8> = text.bytes().collect();
        let n = text.len();
        let mut hs: HashSet<i64> = HashSet::new();
        for i in 1..=n / 2 {
            let mut j = 0;
            let mut k = i;
            let mut hash1 = 0;
            let mut hash2 = 0;
            let mut m = 1;
            for _ in 0..i {
                m *= PRIME;
                m %= MOD;
            }
            while k < n {
                hash1 = (hash1 * PRIME + (text[j] - b'a' + 1) as i64) % MOD;
                hash2 = (hash2 * PRIME + (text[k] - b'a' + 1) as i64) % MOD;
                if j >= i {
                    hash1 = (MOD + hash1 - ((text[j - i] - b'a' + 1) as i64 * m) % MOD) % MOD;
                    hash2 = (MOD + hash2 - ((text[k - i] - b'a' + 1) as i64 * m) % MOD) % MOD;
                }
                j += 1;
                k += 1;
                if j >= i && hash1 == hash2 {
                    hs.insert(hash1);
                }
            }
        }
        hs.len() as i32
    }
}

#[test]
fn test() {
    let text = "abcabcabc".to_string();
    let res = 3;
    assert_eq!(Solution::distinct_echo_substrings(text), res);
    let text = "leetcodeleetcode".to_string();
    let res = 2;
    assert_eq!(Solution::distinct_echo_substrings(text), res);
    let text = "abcabcabcabcabcabcabcabcabc".to_string();
    let res = 12;
    assert_eq!(Solution::distinct_echo_substrings(text), res);
}
