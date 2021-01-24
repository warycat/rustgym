struct Solution;

impl Solution {
    fn prime_palindrome(n: i32) -> i32 {
        if (8..=11).contains(&n) {
            return 11;
        }
        for i in 1..100_000 {
            let mut x: Vec<char> = format!("{}", i).chars().collect();
            let mut y = x.clone();
            y.reverse();
            x.extend(y.iter().skip(1));
            let s: String = x.iter().collect();
            let v = s.parse::<i32>().unwrap();
            if v < n {
                continue;
            }
            if Self::is_prime(v) {
                return v;
            }
        }
        0
    }
    fn is_prime(x: i32) -> bool {
        if x < 2 || x % 2 == 0 {
            return x == 2;
        }
        let mut i = 3;
        while i * i <= x {
            if x % i == 0 {
                return false;
            } else {
                i += 2;
            }
        }
        true
    }
}

#[test]
fn test() {
    let n = 6;
    let res = 7;
    assert_eq!(Solution::prime_palindrome(n), res);
    let n = 8;
    let res = 11;
    assert_eq!(Solution::prime_palindrome(n), res);
    let n = 13;
    let res = 101;
    assert_eq!(Solution::prime_palindrome(n), res);
    let n = 9_989_900;
    let res = 100_030_001;
    assert_eq!(Solution::prime_palindrome(n), res);
}
