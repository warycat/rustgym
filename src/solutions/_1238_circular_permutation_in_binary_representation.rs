struct Solution;

impl Solution {
    fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..1 << n {
            res.push(start ^ (i ^ i >> 1));
        }
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let start = 3;
    let res = vec![3, 2, 0, 1];
    assert_eq!(Solution::circular_permutation(n, start), res);
    let n = 3;
    let start = 2;
    let res = vec![2, 3, 1, 0, 4, 5, 7, 6];
    assert_eq!(Solution::circular_permutation(n, start), res);
}
