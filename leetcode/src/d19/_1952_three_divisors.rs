struct Solution;

impl Solution {
    fn is_three(n: i32) -> bool {
        let mut count = 2;
        for i in 2..n {
            if n % i == 0 {
                count += 1;
            }
        }
        count == 3
    }
}

#[test]
fn test() {
    let n = 2;
    let res = false;
    assert_eq!(Solution::is_three(n), res);
    let n = 4;
    let res = true;
    assert_eq!(Solution::is_three(n), res);
}
