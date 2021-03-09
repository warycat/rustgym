struct Solution;

impl Solution {
    fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut i = n - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        let mut j = i;
        let mut k = n - 1;
        while j < k {
            nums.swap(j, k);
            j += 1;
            k -= 1;
        }
        if i > 0 {
            k = i;
            i -= 1;
            while nums[k] <= nums[i] {
                k += 1;
            }
            nums.swap(i, k)
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 2, 3];
    let res = vec![1, 3, 2];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, res);
    let mut nums = vec![3, 2, 1];
    let res = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, res);
    let mut nums = vec![1, 1, 5];
    let res = vec![1, 5, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, res);
}
