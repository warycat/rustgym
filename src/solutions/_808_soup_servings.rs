struct Solution;
use std::collections::HashMap;

impl Solution {
    fn soup_servings(n: i32) -> f64 {
        if n > 5000 {
            return 1.0;
        }
        let mut memo: HashMap<(i32, i32), f64> = HashMap::new();
        Self::dp(n, n, &mut memo)
    }
    fn dp(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
        if a == 0 && b == 0 {
            return 0.5;
        }
        if a == 0 && b != 0 {
            return 1.0;
        }
        if a != 0 && b == 0 {
            return 0.0;
        }

        if let Some(&res) = memo.get(&(a, b)) {
            return res;
        }
        let mut res = 0.0;
        res += 0.25 * Self::dp(a - a.min(100), b, memo);
        res += 0.25 * Self::dp(a - a.min(75), b - b.min(25), memo);
        res += 0.25 * Self::dp(a - a.min(50), b - b.min(50), memo);
        res += 0.25 * Self::dp(a - a.min(25), b - b.min(75), memo);
        memo.insert((a, b), res);
        res
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let n = 50;
    let res = 0.625;
    assert_approx_eq!(Solution::soup_servings(n), res);
}
