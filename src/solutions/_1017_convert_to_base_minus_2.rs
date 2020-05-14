struct Solution;

impl Solution {
    fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut res = vec![];
        while n != 0 {
            res.push((b'0' + (n & 1) as u8) as char);
            n = -(n >> 1);
        }
        res.reverse();
        res.into_iter().collect()
    }
}

#[test]
fn test() {
    let n = 2;
    let res = "110".to_string();
    assert_eq!(Solution::base_neg2(n), res);
    let n = 3;
    let res = "111".to_string();
    assert_eq!(Solution::base_neg2(n), res);
    let n = 4;
    let res = "100".to_string();
    assert_eq!(Solution::base_neg2(n), res);
}
