struct Solution;

impl Solution {
    fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    for l in k + 1..n {
                        if nums[i] + nums[j] + nums[k] == nums[l] {
                            res += 1;
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 6];
    let res = 1;
    assert_eq!(Solution::count_quadruplets(nums), res);
    let nums = vec![3, 3, 6, 4, 5];
    let res = 0;
    assert_eq!(Solution::count_quadruplets(nums), res);
    let nums = vec![1, 1, 1, 3, 5];
    let res = 4;
    assert_eq!(Solution::count_quadruplets(nums), res);
}
