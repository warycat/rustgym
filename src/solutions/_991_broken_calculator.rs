struct Solution;

impl Solution {
    fn broken_calc(x: i32, mut y: i32) -> i32 {
        let mut res = 0;
        while y > x {
            if y % 2 == 0 {
                y /= 2;
            } else {
                y += 1;
            }
            res += 1;
        }
        res + x - y
    }
}

#[test]
fn test() {
    let x = 2;
    let y = 3;
    let res = 2;
    assert_eq!(Solution::broken_calc(x, y), res);
    let x = 5;
    let y = 8;
    let res = 2;
    assert_eq!(Solution::broken_calc(x, y), res);
    let x = 3;
    let y = 10;
    let res = 3;
    assert_eq!(Solution::broken_calc(x, y), res);
    let x = 1024;
    let y = 1;
    let res = 1023;
    assert_eq!(Solution::broken_calc(x, y), res);
}
