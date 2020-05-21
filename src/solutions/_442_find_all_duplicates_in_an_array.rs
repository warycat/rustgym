struct Solution;

impl Solution {
    fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let n = nums.len();
        for i in 0..n {
            let x = nums[i];
            let index = (x.abs() - 1) as usize;
            if nums[index] < 0 {
                res.push(x.abs());
            } else {
                nums[index] *= -1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let res = vec![2, 3];
    assert_eq!(Solution::find_duplicates(nums), res);
}
