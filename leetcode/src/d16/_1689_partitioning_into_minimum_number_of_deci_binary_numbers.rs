struct Solution;

impl Solution {
    fn min_partitions(n: String) -> i32 {
        let mut res = 0;
        for b in n.bytes() {
            res = res.max(b - b'0');
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = "32".to_string();
    let res = 3;
    assert_eq!(Solution::min_partitions(n), res);
}
