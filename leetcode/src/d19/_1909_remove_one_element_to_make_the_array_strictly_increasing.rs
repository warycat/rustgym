struct Solution;

impl Solution {
    fn can_be_increasing(nums: Vec<i32>) -> bool {
        let n = nums.len();
        for i in 0..n {
            let mut v = vec![];
            for j in 0..n {
                if i == j {
                    continue;
                }
                v.push(nums[j]);
            }
            if Self::is_increasing(&v) {
                return true;
            }
        }
        false
    }

    fn is_increasing(nums: &[i32]) -> bool {
        let n = nums.len();
        for i in 1..n {
            if nums[i] <= nums[i - 1] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 10, 5, 7];
    let res = true;
    assert_eq!(Solution::can_be_increasing(nums), res);
    let nums = vec![2, 3, 1, 2];
    let res = false;
    assert_eq!(Solution::can_be_increasing(nums), res);
    let nums = vec![1, 1, 1];
    let res = false;
    assert_eq!(Solution::can_be_increasing(nums), res);
    let nums = vec![1, 2, 3];
    let res = true;
    assert_eq!(Solution::can_be_increasing(nums), res);
}
