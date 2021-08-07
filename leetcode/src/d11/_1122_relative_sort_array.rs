struct Solution;

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for (i, &v) in arr2.iter().enumerate() {
            hm.insert(v, i);
        }
        arr1.sort_by(|a, b| match (hm.get(a), hm.get(b)) {
            (Some(i), Some(j)) => i.cmp(j),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.cmp(b),
        });
        arr1
    }
}

#[test]
fn test() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    let res = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];
    assert_eq!(Solution::relative_sort_array(arr1, arr2), res);
}
