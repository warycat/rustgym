struct Solution;

impl Solution {
    fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            loop {
                let j = nums[i] - 1;
                if j >= 0 && j < n as i32 && nums[j as usize] != nums[i] {
                    nums.swap(i, j as usize);
                } else {
                    break;
                }
            }
        }
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return (i + 1) as i32;
            }
        }
        (n + 1) as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 0];
    let res = 3;
    assert_eq!(Solution::first_missing_positive(nums), res);
    let nums = vec![3, 4, -1, 1];
    let res = 2;
    assert_eq!(Solution::first_missing_positive(nums), res);
    let nums = vec![7, 8, 9, 11, 12];
    let res = 1;
    assert_eq!(Solution::first_missing_positive(nums), res);
}
