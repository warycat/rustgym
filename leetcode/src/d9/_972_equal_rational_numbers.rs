struct Solution;

impl Solution {
    fn is_rational_equal(s: String, t: String) -> bool {
        let a = Self::parse(s);
        let b = Self::parse(t);
        (a - b).abs() < std::f64::EPSILON
    }

    fn parse(s: String) -> f64 {
        let n = s.len();
        if let Some(dot) = s.find('.') {
            let integer_part = &s[..dot];
            if let Some(lparen) = s.find('(') {
                let non_repeating_part = &s[dot + 1..lparen];
                let repeating_part = &s[lparen + 1..n - 1];
                Self::rational(integer_part, non_repeating_part, repeating_part)
            } else {
                let non_repeating_part = &s[dot + 1..];
                Self::rational(integer_part, non_repeating_part, "")
            }
        } else {
            s.parse::<f64>().unwrap()
        }
    }

    fn rational(integer_part: &str, non_repeating_part: &str, repeating_part: &str) -> f64 {
        let a = integer_part.parse::<f64>().unwrap_or(0.0);
        let n = non_repeating_part.len();
        let b = non_repeating_part.parse::<f64>().unwrap_or(0.0);
        let m = repeating_part.len();
        let c = repeating_part.parse::<f64>().unwrap_or(0.0);
        let mut d = 0.0;
        for _ in 0..m {
            d *= 10.0;
            d += 9.0;
        }
        let mut e = 1.0;
        for _ in 0..n {
            e *= 10.0;
        }
        if n == 0 && m == 0 {
            return a;
        }
        if n == 0 {
            return a + c / d;
        }
        if m == 0 {
            return a + b / e;
        }
        a + (b + c / d) / e
    }
}

#[test]
fn test() {
    let s = "0.(52)".to_string();
    let t = "0.5(25)".to_string();
    let res = true;
    assert_eq!(Solution::is_rational_equal(s, t), res);
    let s = "0.1666(6)".to_string();
    let t = "0.166(66)".to_string();
    let res = true;
    assert_eq!(Solution::is_rational_equal(s, t), res);
    let s = "0.9(9)".to_string();
    let t = "1.".to_string();
    let res = true;
    assert_eq!(Solution::is_rational_equal(s, t), res);
}
