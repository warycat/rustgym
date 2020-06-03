struct Solution;

use std::collections::HashMap;

impl Solution {
    fn total_fruit(tree: Vec<i32>) -> i32 {
        let n = tree.len();
        let mut last: HashMap<i32, usize> = HashMap::new();
        let mut start = 0;
        let mut res = 0;
        for end in 0..n {
            *last.entry(tree[end]).or_default() = end;
            if last.len() == 3 {
                let (index, key) = last.iter().map(|(&k, &v)| (v, k)).min().unwrap();
                start = index + 1;
                last.remove(&key);
            }
            res = res.max(end - start + 1);
        }
        res as i32
    }
}

#[test]
fn test() {
    let tree = vec![1, 2, 1];
    let res = 3;
    assert_eq!(Solution::total_fruit(tree), res);
    let tree = vec![0, 1, 2, 2];
    let res = 3;
    assert_eq!(Solution::total_fruit(tree), res);
    let tree = vec![1, 2, 3, 2, 2];
    let res = 4;
    assert_eq!(Solution::total_fruit(tree), res);
    let tree = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
    let res = 5;
    assert_eq!(Solution::total_fruit(tree), res);
}
