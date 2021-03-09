struct Solution;

use std::collections::HashMap;
impl Solution {
    fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        if croak_of_frogs.len() % 5 != 0 {
            return -1;
        }
        let mut id: HashMap<char, usize> = HashMap::new();
        for &c in &['c', 'r', 'o', 'a', 'k'] {
            id.insert(c, id.len());
        }
        let mut count = vec![0; 5];
        let mut frogs = 0;
        let mut res = 0;
        for c in croak_of_frogs.chars() {
            let i = id[&c];
            count[i] += 1;
            if i == 0 {
                frogs += 1;
                res = res.max(frogs);
            }
            if i > 0 {
                if count[i - 1] < count[i] {
                    return -1;
                }
            }
            if i == 4 {
                frogs -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let croak_of_frogs = "croakcroak".to_string();
    let res = 1;
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), res);
    let croak_of_frogs = "crcoakroak".to_string();
    let res = 2;
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), res);
    let croak_of_frogs = "croakcrook".to_string();
    let res = -1;
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), res);
    let croak_of_frogs = "croakcroa".to_string();
    let res = -1;
    assert_eq!(Solution::min_number_of_frogs(croak_of_frogs), res);
}
