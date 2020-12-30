struct Solution;

use std::collections::HashMap;

impl Solution {
    fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1: Vec<i64> = nums1.into_iter().map(|x| x as i64).collect();
        let nums2: Vec<i64> = nums2.into_iter().map(|x| x as i64).collect();
        let nums1_square: Vec<i64> = nums1.iter().map(|x| x * x).collect();
        let nums2_square: Vec<i64> = nums2.iter().map(|x| x * x).collect();
        let mut res = 0;
        for x in nums1_square {
            res += Self::product(x, &nums2);
        }
        for x in nums2_square {
            res += Self::product(x, &nums1);
        }
        res
    }

    fn product(x: i64, nums: &[i64]) -> i32 {
        let mut count: HashMap<i64, i32> = HashMap::new();
        let mut res = 0;
        for &y in nums {
            if x % y == 0 {
                if let Some(t) = count.get(&(x / y)) {
                    res += t;
                }
            }
            *count.entry(y).or_default() += 1;
        }
        res
    }
}

#[test]
fn test() {
    let nums1 = vec![7, 4];
    let nums2 = vec![5, 2, 8, 9];
    let res = 1;
    assert_eq!(Solution::num_triplets(nums1, nums2), res);
    let nums1 = vec![1, 1];
    let nums2 = vec![1, 1, 1];
    let res = 9;
    assert_eq!(Solution::num_triplets(nums1, nums2), res);
    let nums1 = vec![7, 7, 8, 3];
    let nums2 = vec![1, 2, 9, 7];
    let res = 2;
    assert_eq!(Solution::num_triplets(nums1, nums2), res);
    let nums1 = vec![4, 7, 9, 11, 23];
    let nums2 = vec![3, 5, 1024, 12, 18];
    let res = 0;
    assert_eq!(Solution::num_triplets(nums1, nums2), res);
    let nums1 = vec![43024, 99908];
    let nums2 = vec![1864];
    let res = 0;
    assert_eq!(Solution::num_triplets(nums1, nums2), res);
}
