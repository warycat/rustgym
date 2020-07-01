struct Solution;

impl Solution {
    fn valid_subarrays(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut res = 0;
        for x in nums {
            while let Some(&top) = stack.last() {
                if top > x {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(x);
            res += stack.len();
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 2, 5, 3];
    let res = 11;
    assert_eq!(Solution::valid_subarrays(nums), res);
    let nums = vec![3, 2, 1];
    let res = 3;
    assert_eq!(Solution::valid_subarrays(nums), res);
    let nums = vec![2, 2, 2];
    let res = 6;
    assert_eq!(Solution::valid_subarrays(nums), res);
}
