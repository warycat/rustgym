struct Solution;

impl Solution {
    fn fib(n: i32) -> i32 {
        let mut a: Vec<i32> = vec![0; 31];
        a[1] = 1;
        for i in 2..=30 {
            a[i] = a[i - 1] + a[i - 2];
        }
        a[n as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
}
