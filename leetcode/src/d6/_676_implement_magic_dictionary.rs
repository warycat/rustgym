use std::collections::HashMap;

#[derive(Default)]
struct MagicDictionary {
    data: HashMap<Vec<char>, Vec<(char, usize)>>,
}

impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary::default()
    }

    fn build_dict(&mut self, dict: Vec<String>) {
        for s in dict {
            let s: Vec<char> = s.chars().collect();
            let n = s.len();
            for i in 0..n {
                let mut t = vec![];
                for &c in &s[..i] {
                    t.push(c);
                }
                for &c in &s[i + 1..] {
                    t.push(c);
                }
                self.data.entry(t).or_default().push((s[i], i));
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let s: Vec<char> = word.chars().collect();
        let n = s.len();
        for i in 0..n {
            let mut t = vec![];
            for &c in &s[..i] {
                t.push(c);
            }
            for &c in &s[i + 1..] {
                t.push(c);
            }
            if let Some(v) = self.data.get(&t) {
                for &(c, j) in v {
                    if c != s[i] && i == j {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec_string!["hello", "leetcode"]);
    assert_eq!(obj.search("hello".to_string()), false);
    assert_eq!(obj.search("hhllo".to_string()), true);
    assert_eq!(obj.search("hell".to_string()), false);
    assert_eq!(obj.search("leetcoded".to_string()), false);
}
