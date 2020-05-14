struct Solution;

impl Solution {
    fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let n = nums.len();
        let mut res = vec![-1; n];
        for i in 0..2 * n {
            let j = i % n;
            let x = nums[j];
            while let Some(top) = stack.pop() {
                if nums[top] < x {
                    res[top] = x;
                } else {
                    stack.push(top);
                    break;
                }
            }
            stack.push(j);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1];
    let res = vec![2, -1, 2];
    assert_eq!(Solution::next_greater_elements(nums), res);
}
