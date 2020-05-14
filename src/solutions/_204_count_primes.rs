struct Solution;

impl Solution {
    fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let n: usize = n as usize;
        let mut a: Vec<bool> = vec![true; n];
        a[0] = false;
        a[1] = false;
        let mut i: usize = 2;
        while i * i < n {
            if a[i] {
                let mut j = 2;
                while j * i < n {
                    a[i * j] = false;
                    j += 1;
                }
            }
            i += 1;
        }
        let sum: i32 = a.iter().fold(0, |sum, &b| if b { sum + 1 } else { sum });
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_primes(10), 4);
}
