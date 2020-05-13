struct Solution;

impl Solution {
    fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        for i in 0..n {
            if Self::next(&nums, i) == i {
                nums[i] = 0;
            }
        }
        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }
            let mut slow = i;
            let mut fast = i;
            while nums[slow] * nums[Self::next(&nums, fast)] > 0
                && nums[slow] * nums[Self::next(&nums, Self::next(&nums, fast))] > 0
            {
                slow = Self::next(&nums, slow);
                fast = Self::next(&nums, Self::next(&nums, fast));
                if slow == fast {
                    return true;
                }
            }
            let mut j = i;
            let val = nums[i];
            while nums[j] * val > 0 {
                let next = Self::next(&nums, j);
                nums[j] = 0;
                j = next;
            }
        }
        false
    }

    fn next(nums: &[i32], index: usize) -> usize {
        let n = nums.len();
        let index = index as i32 + nums[index];
        let index = if index < 0 {
            n as i32 + (index % n as i32)
        } else {
            index % n as i32
        };
        (index as usize) % n
    }
}

#[test]
fn test() {
    let nums = vec![2, -1, 1, 2, 2];
    let res = true;
    assert_eq!(Solution::circular_array_loop(nums), res);
    let nums = vec![-1, 2];
    let res = false;
    assert_eq!(Solution::circular_array_loop(nums), res);
    let nums = vec![-2, 1, -1, -2, -2];
    let res = false;
    assert_eq!(Solution::circular_array_loop(nums), res);
    let nums = vec![-1];
    let res = false;
    assert_eq!(Solution::circular_array_loop(nums), res);
}
