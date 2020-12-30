struct Solution;

impl Solution {
    fn number_of_primes(n: usize) -> i32 {
        let mut a: Vec<bool> = vec![true; n + 1];
        a[0] = false;
        a[1] = false;
        let mut i: usize = 2;
        while i * i <= n {
            if a[i] {
                let mut j: usize = 2;
                while i * j <= n {
                    a[i * j] = false;
                    j += 1;
                }
            }
            i += 1;
        }
        let mut res = 0;
        for k in 0..=n {
            if a[k] {
                res += 1;
            }
        }
        res
    }

    fn num_prime_arrangements(n: i32) -> i32 {
        let primes = Self::number_of_primes(n as usize);
        let mut product = 1i64;
        for i in 1..=primes {
            product *= i as i64;
            product %= 1_000_000_007;
        }
        for i in 1..=(n - primes) {
            product *= i as i64;
            product %= 1_000_000_007;
        }
        product as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_prime_arrangements(5), 12);
    assert_eq!(Solution::num_prime_arrangements(100), 682_289_015);
}
