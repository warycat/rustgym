struct Solution;

use std::collections::HashMap;

impl Solution {
    fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = vec![];
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in nums2 {
            while let Some(last) = stack.pop() {
                if last > x {
                    stack.push(last);
                    break;
                } else {
                    hm.insert(last, x);
                }
            }
            stack.push(x);
        }
        nums1.iter().map(|x| *hm.get(x).unwrap_or(&-1)).collect()
    }
}

#[test]
fn test() {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    let res = vec![-1, 3, -1];
    assert_eq!(Solution::next_greater_element(nums1, nums2), res);
    let nums1 = vec![2, 4];
    let nums2 = vec![1, 2, 3, 4];
    let res = vec![3, -1];
    assert_eq!(Solution::next_greater_element(nums1, nums2), res);
}
