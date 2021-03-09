struct Solution;

impl Solution {
    fn clumsy(n: i32) -> i32 {
        let magic = vec![1, 2, 2, -1, 0, 0, 3, 3];
        n + if n > 4 {
            magic[(n % 4) as usize]
        } else {
            magic[(n + 3) as usize]
        }
    }
}

#[test]
fn test() {
    let n = 4;
    let res = 7;
    assert_eq!(Solution::clumsy(n), res);
    let n = 10;
    let res = 12;
    assert_eq!(Solution::clumsy(n), res);
}
