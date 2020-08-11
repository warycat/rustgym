struct Solution;
use rustgym_util::*;

trait Postorder {
    fn from_vec(start: usize, end: usize, nums: &[i32]) -> Self;
}

impl Postorder for TreeLink {
    fn from_vec(start: usize, end: usize, nums: &[i32]) -> Self {
        if start == end {
            None
        } else {
            if start + 1 == end {
                tree!(nums[start])
            } else {
                let mut max_i = start;
                let mut max = nums[start];
                for i in start + 1..end {
                    if nums[i] > max {
                        max = nums[i];
                        max_i = i;
                    }
                }
                tree!(
                    max,
                    Self::from_vec(start, max_i, nums),
                    Self::from_vec(max_i + 1, end, nums)
                )
            }
        }
    }
}

impl Solution {
    fn construct_maximum_binary_tree(nums: Vec<i32>) -> TreeLink {
        TreeLink::from_vec(0, nums.len(), &nums)
    }
}

#[test]
fn test() {
    let nums = vec![3, 2, 1, 6, 0, 5];
    let res = tree!(
        6,
        tree!(3, None, tree!(2, None, tree!(1))),
        tree!(5, tree!(0), None)
    );
    assert_eq!(Solution::construct_maximum_binary_tree(nums), res);
}
