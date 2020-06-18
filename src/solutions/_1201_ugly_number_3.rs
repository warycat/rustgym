struct Solution;

impl Solution {
    fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let mut left = 0;
        let mut right = 2_000_000_000;
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::count(mid, a as u64, b as u64, c as u64) < n as u64 {
                left = mid + 1;
            } else {
                right = mid
            }
        }
        left as i32
    }

    fn count(num: u64, a: u64, b: u64, c: u64) -> u64 {
        num / a + num / b + num / c
            - num / Self::lcm(a, b)
            - num / Self::lcm(b, c)
            - num / Self::lcm(a, c)
            + num / Self::lcm(a, Self::lcm(b, c))
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / Self::gcd(a, b)
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if a == 0 {
            b
        } else {
            Self::gcd(b % a, a)
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let a = 2;
    let b = 3;
    let c = 5;
    let res = 4;
    assert_eq!(Solution::nth_ugly_number(n, a, b, c), res);
    let n = 4;
    let a = 2;
    let b = 3;
    let c = 4;
    let res = 6;
    assert_eq!(Solution::nth_ugly_number(n, a, b, c), res);
    let n = 5;
    let a = 2;
    let b = 11;
    let c = 13;
    let res = 10;
    assert_eq!(Solution::nth_ugly_number(n, a, b, c), res);
    let n = 1000000000;
    let a = 2;
    let b = 217983653;
    let c = 336916467;
    let res = 1999999984;
    assert_eq!(Solution::nth_ugly_number(n, a, b, c), res);
}
