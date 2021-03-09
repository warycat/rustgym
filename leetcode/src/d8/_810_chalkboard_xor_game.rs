struct Solution;

impl Solution {
    fn xor_game(nums: Vec<i32>) -> bool {
        let xor = nums.iter().fold(0, |xor, x| xor ^ x);
        xor == 0 || nums.len() % 2 == 0
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2];
    let res = false;
    assert_eq!(Solution::xor_game(nums), res);
}
