struct Solution;

impl Solution {
    fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut bitmask = 0;
        for &num in &nums {
            bitmask ^= num;
        }
        let diff = bitmask & (-bitmask);
        let mut x = 0;
        for &num in &nums {
            if diff & num != 0 {
                x ^= num;
            }
        }
        vec![x, bitmask ^ x]
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 3, 2, 5];
    let res = vec![3, 5];
    assert_eq!(Solution::single_number(nums), res);
}
