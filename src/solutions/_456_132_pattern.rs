struct Solution;

use std::i32;

impl Solution {
    fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut a3 = i32::MIN;
        let mut stack: Vec<i32> = vec![];
        for i in (0..n).rev() {
            if nums[i] < a3 {
                return true;
            } else {
                while let Some(top) = stack.pop() {
                    if nums[i] > top {
                        a3 = top;
                    } else {
                        stack.push(top);
                        break;
                    }
                }
            }
            stack.push(nums[i]);
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = false;
    assert_eq!(Solution::find132pattern(nums), res);
    let nums = vec![3, 1, 4, 2];
    let res = true;
    assert_eq!(Solution::find132pattern(nums), res);
    let nums = vec![-1, 3, 2, 0];
    let res = true;
    assert_eq!(Solution::find132pattern(nums), res);
}
