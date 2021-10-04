struct Solution;

impl Solution {
    fn find_middle_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            let left: i32 = (0..i).map(|j| nums[j]).sum();
            let right: i32 = (i + 1..n).map(|j| nums[j]).sum();
            if left == right {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, -1, 8, 4];
    let res = 3;
    assert_eq!(Solution::find_middle_index(nums), res);
    let nums = vec![1, -1, 4];
    let res = 2;
    assert_eq!(Solution::find_middle_index(nums), res);
    let nums = vec![2, 5];
    let res = -1;
    assert_eq!(Solution::find_middle_index(nums), res);
    let nums = vec![1];
    let res = 0;
    assert_eq!(Solution::find_middle_index(nums), res);
}
