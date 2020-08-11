use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hasher;

struct ValidWordAbbr {
    abbr: HashMap<u64, HashSet<String>>,
}

impl ValidWordAbbr {
    fn new(dictionary: Vec<String>) -> Self {
        let mut abbr: HashMap<u64, HashSet<String>> = HashMap::new();
        for s in dictionary {
            let hash = Self::abbr_hash(&s);
            abbr.entry(hash).or_default().insert(s);
        }
        ValidWordAbbr { abbr }
    }

    fn abbr_hash(s: &str) -> u64 {
        let s: Vec<u8> = s.bytes().collect();
        let n = s.len();
        let mut hasher = DefaultHasher::new();
        if !s.is_empty() {
            hasher.write_u8(s[0]);
            hasher.write_usize(s.len());
            hasher.write_u8(s[n - 1]);
            hasher.finish()
        } else {
            0
        }
    }

    fn is_unique(&self, word: String) -> bool {
        let hash = Self::abbr_hash(&word);
        if let Some(set) = self.abbr.get(&hash) {
            set.contains(&word) && set.len() == 1
        } else {
            true
        }
    }
}

#[test]
fn test() {
    let obj = ValidWordAbbr::new(vec_string!["deer", "door", "cake", "card"]);
    assert_eq!(obj.is_unique("dear".to_string()), false);
    assert_eq!(obj.is_unique("cart".to_string()), true);
    assert_eq!(obj.is_unique("cane".to_string()), false);
    assert_eq!(obj.is_unique("make".to_string()), true);
    assert_eq!(obj.is_unique("door".to_string()), false);
}
