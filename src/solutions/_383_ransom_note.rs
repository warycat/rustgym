struct Solution;

use std::collections::HashMap;

impl Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut hm: HashMap<char, i32> = HashMap::new();
        if ransom_note.len() > magazine.len() {
            return false;
        }
        if ransom_note == magazine {
            return true;
        }
        for c in magazine.chars() {
            let e = hm.entry(c).or_default();
            *e += 1;
        }
        for c in ransom_note.chars() {
            if let Some(v) = hm.get_mut(&c) {
                *v -= 1;
                if *v < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let r = "a".to_string();
    let m = "b".to_string();
    assert_eq!(Solution::can_construct(r, m), false);
    let r = "aa".to_string();
    let m = "ab".to_string();
    assert_eq!(Solution::can_construct(r, m), false);
    let r = "aa".to_string();
    let m = "aab".to_string();
    assert_eq!(Solution::can_construct(r, m), true);
}
