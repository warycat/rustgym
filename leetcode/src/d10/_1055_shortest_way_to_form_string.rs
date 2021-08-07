struct Solution;
use std::collections::HashMap;

impl Solution {
    fn shortest_way(source: String, target: String) -> i32 {
        let mut pos: HashMap<char, Vec<usize>> = HashMap::new();
        let mut res = 1;
        for (i, c) in source.char_indices() {
            pos.entry(c).or_default().push(i);
        }
        let mut start = 0;
        for c in target.chars() {
            if let Some(indexes) = pos.get(&c) {
                loop {
                    let index = Self::next(start, indexes);
                    if index == 0 {
                        start = 0;
                        res += 1;
                    } else {
                        start = index;
                        break;
                    }
                }
            } else {
                return -1;
            }
        }
        res
    }

    fn next(start: usize, indexes: &[usize]) -> usize {
        match indexes.binary_search(&start) {
            Ok(i) => indexes[i] + 1,
            Err(i) => {
                if i == indexes.len() {
                    0
                } else {
                    indexes[i] + 1
                }
            }
        }
    }
}

#[test]
fn test() {
    let source = "abc".to_string();
    let target = "abcbc".to_string();
    let res = 2;
    assert_eq!(Solution::shortest_way(source, target), res);
    let source = "abc".to_string();
    let target = "acdbc".to_string();
    let res = -1;
    assert_eq!(Solution::shortest_way(source, target), res);
    let source = "xyz".to_string();
    let target = "xzyxz".to_string();
    let res = 3;
    assert_eq!(Solution::shortest_way(source, target), res);
    let source = "aaaaa".to_string();
    let target = "aaaaaaaaaaaaa".to_string();
    let res = 3;
    assert_eq!(Solution::shortest_way(source, target), res);
    let source = "adbsc".to_string();
    let target = "addddddddddddsbc".to_string();
    let res = 13;
    assert_eq!(Solution::shortest_way(source, target), res);
}
