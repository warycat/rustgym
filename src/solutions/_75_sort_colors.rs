struct Solution;

impl Solution {
    fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut i = 0;
        while i <= r {
            while nums[i] == 2 && i < r {
                nums.swap(i, r);
                r -= 1;
            }
            while nums[i] == 0 && i > l {
                nums.swap(i, l);
                l += 1;
            }
            i += 1;
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    let res = vec![0, 0, 1, 1, 2, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, res);
}
