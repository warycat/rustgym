struct Solution;

impl Solution {
    fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
        while sx < tx && sy < ty {
            if tx < ty {
                ty %= tx;
            } else {
                tx %= ty;
            }
        }
        sx == tx && sy <= ty && (ty - sy) % sx == 0 || sy == ty && sx <= tx && (tx - sx) % sy == 0
    }
}

#[test]
fn test() {
    let sx = 1;
    let sy = 1;
    let tx = 3;
    let ty = 5;
    let res = true;
    assert_eq!(Solution::reaching_points(sx, sy, tx, ty), res);
    let sx = 1;
    let sy = 1;
    let tx = 2;
    let ty = 2;
    let res = false;
    assert_eq!(Solution::reaching_points(sx, sy, tx, ty), res);
    let sx = 1;
    let sy = 1;
    let tx = 1;
    let ty = 1;
    let res = true;
    assert_eq!(Solution::reaching_points(sx, sy, tx, ty), res);
}
