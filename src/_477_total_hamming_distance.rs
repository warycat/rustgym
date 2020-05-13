struct Solution;

impl Solution {
    fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        for i in 0..32 {
            let mut count = 0;
            for &x in &nums {
                if x & 1 << i != 0 {
                    count += 1;
                }
            }
            res += count * (n - count);
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![4, 14, 2];
    let res = 6;
    assert_eq!(Solution::total_hamming_distance(nums), res);
}
