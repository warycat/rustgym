struct Solution;

impl Solution {
    fn mirror_reflection(mut p: i32, mut q: i32) -> i32 {
        while p % 2 == 0 && q % 2 == 0 {
            p /= 2;
            q /= 2;
        }
        if p % 2 == 0 {
            return 2;
        }
        if q % 2 == 0 {
            return 0;
        }
        1
    }
}

#[test]
fn test() {
    let p = 2;
    let q = 1;
    let res = 2;
    assert_eq!(Solution::mirror_reflection(p, q), res);
}
