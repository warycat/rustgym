struct Solution;

impl Solution {
    fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let n = encoded.len();
        let mut res = vec![first];
        for i in 0..n {
            res.push(res[i] ^ encoded[i]);
        }
        res
    }
}

#[test]
fn test() {
    let encoded = vec![1, 2, 3];
    let first = 1;
    let res = vec![1, 0, 2, 1];
    assert_eq!(Solution::decode(encoded, first), res);
    let encoded = vec![6, 2, 7, 3];
    let first = 4;
    let res = vec![4, 2, 0, 7, 4];
    assert_eq!(Solution::decode(encoded, first), res);
}
