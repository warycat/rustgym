struct Solution;

impl Solution {
    fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut i: usize = 0;
        let mut j: usize = i;
        let mut res: usize = 1;
        while i < n {
            while j + 1 < n && nums[j + 1] > nums[j] {
                j += 1;
            }
            res = usize::max(j - i + 1, res);
            i = j + 1;
            j = i;
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 5, 4, 7];
    assert_eq!(Solution::find_length_of_lcis(nums), 3);
    let nums = vec![2, 2, 2, 2, 2];
    assert_eq!(Solution::find_length_of_lcis(nums), 1);
}
