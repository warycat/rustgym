struct Solution;

impl Solution {
    fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut a: Vec<i32> = vec![0; 38];
        a[0] = 0;
        a[1] = 1;
        a[2] = 1;
        for i in 3..=n {
            a[i] = a[i - 1] + a[i - 2] + a[i - 3];
        }
        a[n]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::tribonacci(25), 1_389_537);
}
