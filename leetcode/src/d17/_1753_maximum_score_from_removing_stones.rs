struct Solution;

impl Solution {
    fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let sum = a + b + c;
        let min = a.max(b).max(c);
        (sum / 2).min(sum - min)
    }
}

#[test]
fn test() {
    let a = 2;
    let b = 4;
    let c = 6;
    let res = 6;
    assert_eq!(Solution::maximum_score(a, b, c), res);
    let a = 4;
    let b = 4;
    let c = 6;
    let res = 7;
    assert_eq!(Solution::maximum_score(a, b, c), res);
    let a = 1;
    let b = 8;
    let c = 8;
    let res = 8;
    assert_eq!(Solution::maximum_score(a, b, c), res);
}
