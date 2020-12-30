struct Solution;

impl Solution {
    fn is_armstrong(n: i32) -> bool {
        let mut x = n;
        let mut k = 0;
        let mut digits: Vec<i32> = vec![];
        while x > 0 {
            let d = x % 10;
            digits.push(d);
            x /= 10;
            k += 1;
        }
        let sum: i32 = digits.iter().map(|x| x.pow(k)).sum();
        sum == n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_armstrong(153), true);
    assert_eq!(Solution::is_armstrong(123), false);
}
