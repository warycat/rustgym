struct Solution;
use std::collections::HashSet;

impl Solution {
    fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let n = s.len();
        let mut hash = 0;
        let mut once: HashSet<u32> = HashSet::new();
        let mut twice: HashSet<u32> = HashSet::new();
        let s: Vec<char> = s.chars().collect();
        for i in (0..n).rev() {
            hash <<= 2;
            hash |= Self::val(s[i]);
            if i + 10 < n {
                hash -= Self::val(s[i + 10]) << 20;
            }
            if i + 10 <= n {
                if !once.insert(hash) {
                    twice.insert(hash);
                }
            }
        }
        twice.into_iter().map(Self::decode).collect()
    }
    fn val(c: char) -> u32 {
        match c {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            _ => 3,
        }
    }
    fn decode(mut hash: u32) -> String {
        let mut res = "".to_string();
        for _ in 0..10 {
            res.push(match hash & 3 {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                _ => 'T',
            });
            hash >>= 2;
        }
        res
    }
}

#[test]
fn test() {
    let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
    let mut res = vec_string!["AAAAACCCCC", "CCCCCAAAAA"];
    let mut ans = Solution::find_repeated_dna_sequences(s);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
