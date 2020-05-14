struct Solution;

impl Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..(1 << n) {
            res.push(i ^ i >> 1);
        }
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let res = vec![0, 1, 3, 2];
    assert_eq!(Solution::gray_code(n), res);
}
