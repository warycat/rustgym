struct Solution;

impl Solution {
    fn solve_equation(equation: String) -> String {
        let mut it = equation.split('=');
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        let (a, b) = Self::parse(left);
        let (c, d) = Self::parse(right);
        if a == c {
            if b == d {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            format!("x={}", (d - b) / (a - c))
        }
    }
    fn parse(s: &str) -> (i32, i32) {
        let mut sign = 1;
        let mut x = false;
        let mut val = None;
        let mut a = 0;
        let mut b = 0;
        for c in s.chars() {
            match c {
                'x' => {
                    if val.is_none() {
                        val = Some(1);
                    }
                    x = true;
                }
                '+' => {
                    if let Some(v) = val {
                        if x {
                            a += sign * v;
                        } else {
                            b += sign * v;
                        }
                    }
                    val = None;
                    x = false;
                    sign = 1;
                }
                '-' => {
                    if let Some(v) = val {
                        if x {
                            a += sign * v;
                        } else {
                            b += sign * v;
                        }
                    }
                    val = None;
                    x = false;
                    sign = -1;
                }
                _ => {
                    val = if let Some(mut v) = val {
                        v *= 10;
                        v += (c as u8 - b'0') as i32;
                        Some(v)
                    } else {
                        Some((c as u8 - b'0') as i32)
                    };
                }
            }
        }
        if x {
            if val.is_none() {
                val = Some(1);
            }
            a += sign * val.unwrap();
        } else {
            b += sign * val.unwrap();
        }
        (a, b)
    }
}

#[test]
fn test() {
    let equation = "x+5-3+x=6+x-2".to_string();
    let res = "x=2".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "x=x".to_string();
    let res = "Infinite solutions".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "2x=x".to_string();
    let res = "x=0".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "2x+3x-6x=x+2".to_string();
    let res = "x=-1".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "x=x+2".to_string();
    let res = "No solution".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "0x=0".to_string();
    let res = "Infinite solutions".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
    let equation = "-x=-1".to_string();
    let res = "x=1".to_string();
    assert_eq!(Solution::solve_equation(equation), res);
}
