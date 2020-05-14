struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_isomorphic(s: String, t: String) -> bool {
        let mut hmst: HashMap<char, char> = HashMap::new();
        let mut hmts: HashMap<char, char> = HashMap::new();
        let mut is = s.chars();
        let mut it = t.chars();
        while let (Some(cs), Some(ct)) = (is.next(), it.next()) {
            if let Some(&vt) = hmst.get(&cs) {
                if vt != ct {
                    return false;
                }
            } else {
                hmst.insert(cs, ct);
            }
            if let Some(&vs) = hmts.get(&ct) {
                if vs != cs {
                    return false;
                }
            } else {
                hmts.insert(ct, cs);
            }
        }
        true
    }
}

#[test]
fn test() {
    let s = "egg".to_string();
    let t = "add".to_string();
    assert_eq!(Solution::is_isomorphic(s, t), true);
    let s = "foo".to_string();
    let t = "bar".to_string();
    assert_eq!(Solution::is_isomorphic(s, t), false);
    let s = "paper".to_string();
    let t = "title".to_string();
    assert_eq!(Solution::is_isomorphic(s, t), true);
}
