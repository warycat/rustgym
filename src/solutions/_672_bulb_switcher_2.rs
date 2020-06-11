struct Solution;

impl Solution {
    fn flip_lights(n: i32, m: i32) -> i32 {
        let n = n.min(3);
        if m == 0 || n == 0 {
            return 1;
        }
        if n == 1 {
            return 2;
        }
        if n == 2 {
            return if m == 1 { 3 } else { 4 };
        }
        if m == 1 {
            return 4;
        }
        if m == 2 {
            7
        } else {
            8
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let m = 1;
    let res = 2;
    assert_eq!(Solution::flip_lights(n, m), res);
    let n = 2;
    let m = 1;
    let res = 3;
    assert_eq!(Solution::flip_lights(n, m), res);
    let n = 3;
    let m = 1;
    let res = 4;
    assert_eq!(Solution::flip_lights(n, m), res);
}
