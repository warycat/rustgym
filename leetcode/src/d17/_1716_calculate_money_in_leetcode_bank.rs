struct Solution;

impl Solution {
    fn total_money(n: i32) -> i32 {
        let mut res = 0;
        for i in 0..n {
            res += (i % 7) + (i / 7) + 1;
        }
        res
    }
}

#[test]
fn test() {
    let n = 4;
    let res = 10;
    assert_eq!(Solution::total_money(n), res);
    let n = 10;
    let res = 37;
    assert_eq!(Solution::total_money(n), res);
    let n = 20;
    let res = 96;
    assert_eq!(Solution::total_money(n), res);
}
