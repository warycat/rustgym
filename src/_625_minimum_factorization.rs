struct Solution;

impl Solution {
    fn smallest_factorization(mut a: i32) -> i32 {
        let mut res: Vec<i32> = vec![];
        if a == 1 {
            return 1;
        }
        for i in (2..=9).rev() {
            while a % i == 0 {
                res.push(i);
                a /= i;
            }
        }
        if a != 1 {
            return 0;
        }
        let n = res.len();
        let mut x: i64 = 0;
        for i in (0..n).rev() {
            x = x * 10 + res[i] as i64;
        }
        if x > std::i32::MAX as i64 {
            0
        } else {
            x as i32
        }
    }
}

#[test]
fn test() {
    let a = 48;
    let res = 68;
    assert_eq!(Solution::smallest_factorization(a), res);
    let a = 15;
    let res = 35;
    assert_eq!(Solution::smallest_factorization(a), res);
    let a = 1;
    let res = 1;
    assert_eq!(Solution::smallest_factorization(a), res);
}
