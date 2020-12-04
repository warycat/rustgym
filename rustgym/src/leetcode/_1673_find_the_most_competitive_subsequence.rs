struct Solution;

impl Solution {
    fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut arr = vec![];
        let mut m = 0;
        for i in 0..n {
            while let Some(&top) = arr.last() {
                if top > nums[i] && k < n - m {
                    m += 1;
                    arr.pop();
                } else {
                    break;
                }
            }
            arr.push(nums[i]);
        }
        arr[0..k].to_vec()
    }
}

#[test]
fn test() {
    let nums = vec![3, 5, 2, 6];
    let k = 2;
    let res = vec![2, 6];
    assert_eq!(Solution::most_competitive(nums, k), res);
    let nums = vec![2, 4, 3, 3, 5, 4, 9, 6];
    let k = 4;
    let res = vec![2, 3, 3, 4];
    assert_eq!(Solution::most_competitive(nums, k), res);
}
