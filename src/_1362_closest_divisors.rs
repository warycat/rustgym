struct Solution;

impl Solution {
    fn closest_divisors(num: i32) -> Vec<i32> {
        for i in (0..=((num + 2) as f64).sqrt() as i32).rev() {
            if (num + 1) % i == 0 {
                return vec![(num + 1) / i, i];
            }
            if (num + 2) % i == 0 {
                return vec![(num + 2) / i, i];
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    let num = 8;
    let res = vec![3, 3];
    assert_eq!(Solution::closest_divisors(num), res);
    let num = 123;
    let res = vec![25, 5];
    assert_eq!(Solution::closest_divisors(num), res);
    let num = 999;
    let res = vec![40, 25];
    assert_eq!(Solution::closest_divisors(num), res);
}
