struct Solution;
use std::fmt;
use std::ops::Mul;

struct Complex {
    r: i32,
    i: i32,
}

impl Complex {
    fn new(r: i32, i: i32) -> Self {
        Complex { r, i }
    }
    fn from_string(s: String) -> Self {
        let p = s.find('+').unwrap();
        let n = s.len();
        let r = s[0..p].parse::<i32>().unwrap();
        let i = s[p + 1..n - 1].parse::<i32>().unwrap();
        Self::new(r, i)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Self::new(
            self.r * rhs.r - self.i * rhs.i,
            self.r * rhs.i + self.i * rhs.r,
        )
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}i", self.r, self.i)
    }
}

impl Solution {
    fn complex_number_multiply(a: String, b: String) -> String {
        (Complex::from_string(a) * Complex::from_string(b)).to_string()
    }
}

#[test]
fn test() {
    let a = "1+1i".to_string();
    let b = "1+1i".to_string();
    let res = "0+2i".to_string();
    assert_eq!(Solution::complex_number_multiply(a, b), res);
    let a = "1+-1i".to_string();
    let b = "1+-1i".to_string();
    let res = "0+-2i".to_string();
    assert_eq!(Solution::complex_number_multiply(a, b), res);
}
