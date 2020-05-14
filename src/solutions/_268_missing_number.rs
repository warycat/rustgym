struct Solution;

impl Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        let mut xor: i32 = 0;
        let n = nums.len();
        for n in 0..=n {
            xor ^= n as i32;
        }
        for n in nums {
            xor ^= n;
        }
        xor
    }
}

#[test]
fn test() {
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(Solution::missing_number(nums), 8);
}
