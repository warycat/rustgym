struct Solution;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hasher;

struct Dict {
    same: HashSet<String>,
    capitlization: HashMap<u64, String>,
    vowel_error: HashMap<u64, String>,
}

impl Dict {
    fn new(wordlist: Vec<String>) -> Self {
        let mut same: HashSet<String> = HashSet::new();
        let mut capitlization: HashMap<u64, String> = HashMap::new();
        let mut vowel_error: HashMap<u64, String> = HashMap::new();

        for s in wordlist.iter().rev() {
            same.insert(s.to_string());
            capitlization.insert(Self::hash1(s), s.to_string());
            vowel_error.insert(Self::hash2(s), s.to_string());
        }
        Dict {
            same,
            capitlization,
            vowel_error,
        }
    }

    fn query(&self, s: String) -> String {
        if self.same.contains(&s) {
            return s;
        }
        if let Some(res) = self.capitlization.get(&Self::hash1(&s)) {
            return res.to_string();
        }
        if let Some(res) = self.vowel_error.get(&Self::hash2(&s)) {
            return res.to_string();
        }
        "".to_string()
    }

    fn hash1(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        for c in s.to_lowercase().chars() {
            hasher.write_u8(c as u8);
        }
        hasher.finish()
    }

    fn hash2(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        for c in s.to_lowercase().chars() {
            let c = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '_',
                _ => c,
            };
            hasher.write_u8(c as u8);
        }
        hasher.finish()
    }
}

impl Solution {
    fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let dict = Dict::new(wordlist);
        queries.into_iter().map(|s| dict.query(s)).collect()
    }
}

#[test]
fn test() {
    let wordlist = vec_string!["KiTe", "kite", "hare", "Hare"];
    let queries =
        vec_string!["kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"];
    let res = vec_string!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"];
    assert_eq!(Solution::spellchecker(wordlist, queries), res);
}
