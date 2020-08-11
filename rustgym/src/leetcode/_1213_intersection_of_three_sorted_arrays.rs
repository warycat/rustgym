struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
        let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
        for x in arr1 {
            *btm.entry(x).or_default() += 1;
        }
        for x in arr2 {
            *btm.entry(x).or_default() += 1;
        }
        for x in arr3 {
            *btm.entry(x).or_default() += 1;
        }
        let mut res = vec![];
        for (k, v) in btm {
            if v == 3 {
                res.push(k);
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![1, 2, 5, 7, 9];
    let arr3 = vec![1, 3, 4, 5, 8];
    let res = vec![1, 5];
    assert_eq!(Solution::arrays_intersection(arr1, arr2, arr3), res);
}
