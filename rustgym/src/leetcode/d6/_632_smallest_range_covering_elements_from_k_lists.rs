struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut left = std::i32::MAX;
        let mut right = std::i32::MIN;
        for i in 0..n {
            let x = nums[i][0];
            left = left.min(x);
            right = right.max(x);
            queue.push((Reverse(x), i, 1));
        }
        let mut min = (right - left, (left, right));
        while let Some((Reverse(_), i, j)) = queue.pop() {
            if j == nums[i].len() {
                break;
            } else {
                let x = nums[i][j];
                right = right.max(x);
                queue.push((Reverse(x), i, j + 1));
                if let Some(&(Reverse(y), _, _)) = queue.peek() {
                    left = left.max(y);
                }
                let r = (right - left, (left, right));
                if r < min {
                    min = r;
                }
            }
        }
        vec![(min.1).0, (min.1).1]
    }
}

#[test]
fn test() {
    let nums = vec_vec_i32![[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]];
    let res = vec![20, 24];
    assert_eq!(Solution::smallest_range(nums), res);
}
