struct Solution;

impl Solution {
    fn special_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..=n {
            if i == nums.iter().filter(|&&x| x >= i as i32).count() {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![3, 5];
    let res = 2;
    assert_eq!(Solution::special_array(nums), res);
    let nums = vec![0, 0];
    let res = -1;
    assert_eq!(Solution::special_array(nums), res);
    let nums = vec![0, 4, 3, 0, 4];
    let res = 3;
    assert_eq!(Solution::special_array(nums), res);
}
