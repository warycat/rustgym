struct Solution;

impl Solution {
    fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let aa = (a >> i) & 1;
            let bb = (b >> i) & 1;
            let cc = (c >> i) & 1;
            if cc == 0 {
                if aa == 1 {
                    res += 1;
                }
                if bb == 1 {
                    res += 1;
                }
            } else {
                if aa == 0 && bb == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = 2;
    let b = 6;
    let c = 5;
    let res = 3;
    assert_eq!(Solution::min_flips(a, b, c), res);
    let a = 4;
    let b = 2;
    let c = 7;
    let res = 1;
    assert_eq!(Solution::min_flips(a, b, c), res);
    let a = 1;
    let b = 2;
    let c = 3;
    let res = 0;
    assert_eq!(Solution::min_flips(a, b, c), res);
    let a = 8;
    let b = 3;
    let c = 5;
    let res = 3;
    assert_eq!(Solution::min_flips(a, b, c), res);
}
