struct Solution;

impl Solution {
    fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mask: i32 = (1 << maximum_bit) - 1;
        let mut nums: Vec<i32> = nums.into_iter().map(|x| x & mask).collect();
        let mut xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut res = vec![];
        while let Some(last) = nums.pop() {
            res.push(mask ^ xor);
            xor ^= last;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 1, 3];
    let maximum_bit = 2;
    let res = vec![0, 3, 2, 3];
    assert_eq!(Solution::get_maximum_xor(nums, maximum_bit), res);
    let nums = vec![2, 3, 4, 7];
    let maximum_bit = 3;
    let res = vec![5, 2, 6, 5];
    assert_eq!(Solution::get_maximum_xor(nums, maximum_bit), res);
    let nums = vec![0, 1, 2, 2, 5, 7];
    let maximum_bit = 3;
    let res = vec![4, 3, 6, 4, 6, 7];
    assert_eq!(Solution::get_maximum_xor(nums, maximum_bit), res);
}
