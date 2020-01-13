struct Solution;

impl Solution {
    fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for p in nums.chunks(2) {
            let a = p[0] as usize;
            let b = p[1];
            for _ in 0..a {
                res.push(b);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let res = vec![2, 4, 4, 4];
    assert_eq!(Solution::decompress_rl_elist(nums), res);
}
