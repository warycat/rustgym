struct Solution;

impl Solution {
    fn count_elements(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        if min == max {
            return 0;
        }
        let mut count = 0;
        for i in 0..n {
            if nums[i] == min || nums[i] == max {
                count += 1;
            }
        }
        (n - count) as i32
    }
}

#[test]
fn test() {
    let nums = vec![11, 7, 2, 15];
    let res = 2;
    assert_eq!(Solution::count_elements(nums), res);
    let nums = vec![-3, 3, 3, 90];
    let res = 2;
    assert_eq!(Solution::count_elements(nums), res);
}
