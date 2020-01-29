struct Solution;
use std::collections::BTreeSet;
use std::collections::HashMap;

impl Solution {
    fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut bts: BTreeSet<i32> = BTreeSet::new();
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for &x in &arr {
            bts.insert(x);
        }
        for (i, x) in bts.into_iter().enumerate() {
            hm.insert(x, i as i32 + 1);
        }
        arr.into_iter().map(|x| hm[&x]).collect()
    }
}

#[test]
fn test() {
    let arr = vec![40, 10, 20, 30];
    let res = vec![4, 1, 2, 3];
    assert_eq!(Solution::array_rank_transform(arr), res);
    let arr = vec![100, 100, 100];
    let res = vec![1, 1, 1];
    assert_eq!(Solution::array_rank_transform(arr), res);
    let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
    let res = vec![5, 3, 4, 2, 8, 6, 7, 1, 3];
    assert_eq!(Solution::array_rank_transform(arr), res);
}
