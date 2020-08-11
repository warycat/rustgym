struct Solution;

impl Solution {
    fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        if a == b {
            let mut res = "".to_string();
            for _ in 0..a {
                res += "ab";
            }
            return res;
        }
        if a > b {
            let mut res = "".to_string();
            while a > 0 {
                res += "a";
                a -= 1;
                if a > b {
                    res += "a";
                    a -= 1;
                }
                if b > 0 {
                    res += "b";
                    b -= 1;
                }
            }
            return res;
        }
        if b > a {
            let mut res = "".to_string();
            while b > 0 {
                res += "b";
                b -= 1;
                if b > a {
                    res += "b";
                    b -= 1;
                }
                if a > 0 {
                    res += "a";
                    a -= 1;
                }
            }
            return res;
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let a = 1;
    let b = 2;
    let res = "bab".to_string();
    assert_eq!(Solution::str_without3a3b(a, b), res);
}
