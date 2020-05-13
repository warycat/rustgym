struct Solution;
use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let mut res: Vec<char> = vec![];
        if (numerator < 0) ^ (denominator < 0) {
            res.push('-');
        }
        let n = (numerator as i64).abs();
        let d = (denominator as i64).abs();
        res.append(&mut format!("{}", n / d).chars().collect());
        let mut r = n % d;
        if r == 0 {
            return String::from_iter(res);
        }
        res.push('.');
        let mut hs: HashMap<i64, usize> = HashMap::new();
        while r != 0 {
            if let Some(index) = hs.insert(r, res.len()) {
                res.insert(index, '(');
                res.push(')');
                break;
            }
            r *= 10;
            res.append(&mut format!("{}", r / d).chars().collect());
            r %= d;
        }
        String::from_iter(res)
    }
}

#[test]
fn test() {
    let numerator = 1;
    let denominator = 2;
    let res = "0.5".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    let numerator = 2;
    let denominator = 1;
    let res = "2".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    let numerator = 2;
    let denominator = 3;
    let res = "0.(6)".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    let numerator = 1;
    let denominator = 333;
    let res = "0.(003)".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    let numerator = 1;
    let denominator = 6;
    let res = "0.1(6)".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    let numerator = -50;
    let denominator = 8;
    let res = "-6.25".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
    let numerator = -1;
    let denominator = -2_147_483_648;
    let res = "0.0000000004656612873077392578125".to_string();
    assert_eq!(Solution::fraction_to_decimal(numerator, denominator), res);
}
