struct Solution;

impl Solution {
    fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut first = 0;
        for i in 0..n {
            first ^= (i + 1) as i32;
        }
        for i in 0..n - 1 {
            if i % 2 == 1 {
                first ^= encoded[i];
            }
        }
        let mut res = vec![first];
        for i in 1..n {
            res.push(res[i - 1] ^ encoded[i - 1]);
        }
        res
    }
}

#[test]
fn test() {
    let encoded = vec![3, 1];
    let res = vec![1, 2, 3];
    assert_eq!(Solution::decode(encoded), res);
    let encoded = vec![6, 5, 4, 6];
    let res = vec![2, 4, 1, 5, 3];
    assert_eq!(Solution::decode(encoded), res);
}
