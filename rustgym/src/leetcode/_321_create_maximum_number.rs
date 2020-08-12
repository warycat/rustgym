struct Solution;

impl Solution {
    fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k = k as usize;
        let mut max_merged: Option<Vec<i32>> = None;
        for size1 in 0..=k.min(n1) {
            let size2 = k - size1;
            if size2 > n2 {
                continue;
            }
            let max1 = Self::max_one(&nums1, size1);
            let max2 = Self::max_one(&nums2, size2);
            let max3 = Self::max_merge(max1, max2);
            if let Some(max) = max_merged {
                if max < max3 {
                    max_merged = Some(max3);
                } else {
                    max_merged = Some(max);
                }
            } else {
                max_merged = Some(max3);
            }
        }
        max_merged.unwrap()
    }

    fn max_one(nums: &[i32], k: usize) -> Vec<i32> {
        let mut stack = vec![];
        let n = nums.len();
        for i in 0..n {
            let right = n - i;
            while let Some(&top) = stack.last() {
                if top < nums[i] && stack.len() + right > k {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(nums[i]);
        }
        while stack.len() > k {
            stack.pop();
        }
        stack
    }

    fn max_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut i = 0;
        let mut j = 0;
        loop {
            if i < nums1.len() && j < nums2.len() {
                if Self::greater(&nums1, &nums2, i, j) {
                    res.push(nums1[i]);
                    i += 1;
                } else {
                    res.push(nums2[j]);
                    j += 1;
                }
                continue;
            }
            if i < nums1.len() {
                res.push(nums1[i]);
                i += 1;
                continue;
            }
            if j < nums2.len() {
                res.push(nums2[j]);
                j += 1;
                continue;
            }
            break;
        }
        res
    }

    fn greater(nums1: &[i32], nums2: &[i32], mut i: usize, mut j: usize) -> bool {
        while i < nums1.len() && j < nums2.len() && nums1[i] == nums2[j] {
            i += 1;
            j += 1;
        }
        j == nums2.len() || (i < nums1.len() && nums1[i] > nums2[j])
    }
}

#[test]
fn test() {
    let nums1 = vec![3, 4, 6, 5];
    let nums2 = vec![9, 1, 2, 5, 8, 3];
    let k = 5;
    let res = vec![9, 8, 6, 5, 3];
    assert_eq!(Solution::max_number(nums1, nums2, k), res);
    let nums1 = vec![6, 7];
    let nums2 = vec![6, 0, 4];
    let k = 5;
    let res = vec![6, 7, 6, 0, 4];
    assert_eq!(Solution::max_number(nums1, nums2, k), res);
    let nums1 = vec![3, 9];
    let nums2 = vec![8, 9];
    let k = 3;
    let res = vec![9, 8, 9];
    assert_eq!(Solution::max_number(nums1, nums2, k), res);
    let nums1 = vec![2, 5, 6, 4, 4, 0];
    let nums2 = vec![7, 3, 8, 0, 6, 5, 7, 6, 2];
    let k = 15;
    let res = vec![7, 3, 8, 2, 5, 6, 4, 4, 0, 6, 5, 7, 6, 2, 0];
    assert_eq!(Solution::max_number(nums1, nums2, k), res);
}
