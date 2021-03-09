struct Solution;
use rustgym_util::*;

impl Solution {
    fn sorted_array_to_bst(nums: Vec<i32>) -> TreeLink {
        let n = nums.len();
        match n {
            0 => None,
            1 => tree!(nums[0]),
            _ => {
                let mid = n / 2;
                let left = nums[..mid].to_vec();
                let right = nums[mid + 1..].to_vec();
                tree!(
                    nums[mid],
                    Self::sorted_array_to_bst(left),
                    Self::sorted_array_to_bst(right)
                )
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![-10, -3, 0, 5, 9];
    let bst = tree!(0, tree!(-3, tree!(-10), None), tree!(9, tree!(5), None));
    assert_eq!(Solution::sorted_array_to_bst(nums), bst);
}
