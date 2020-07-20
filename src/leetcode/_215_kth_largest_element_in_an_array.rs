struct Solution;
use std::cmp::Ordering::*;

trait Partition {
    fn partition(&mut self, l: usize, r: usize) -> usize;
}

impl Partition for Vec<i32> {
    fn partition(&mut self, l: usize, r: usize) -> usize {
        self.swap((l + r) / 2, r);
        let mut j = l;
        let pivot = self[r];
        for i in l..r {
            if self[i] <= pivot {
                self.swap(i, j);
                j += 1;
            }
        }
        self.swap(j, r);
        j
    }
}

impl Solution {
    fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;
        let k = n - k as usize;
        while l < r {
            let m = nums.partition(l, r);
            match m.cmp(&k) {
                Less => {
                    l = m + 1;
                }
                Greater => {
                    r = m - 1;
                }
                Equal => {
                    break;
                }
            }
        }
        nums[k]
    }
}

#[test]
fn test() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::find_kth_largest(nums, k), res);
}
