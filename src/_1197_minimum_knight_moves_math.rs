struct Solution;

use std::mem::swap;

impl Solution {
    fn min_knight_moves(mut x: i32, mut y: i32) -> i32 {
        x = x.abs();
        y = y.abs();
        if x < y {
            swap(&mut x, &mut y);
        }
        if x == 1 && y == 0 {
            return 3;
        }
        if x == 2 && y == 2 {
            return 4;
        }
        let delta = x - y;
        if y > delta {
            delta - 2 * ((delta - y) as f64 / 3.0).floor() as i32
        } else {
            delta - 2 * ((delta - y) as f64 / 4.0).floor() as i32
        }
    }
}

#[test]
fn test() {
    let x = 2;
    let y = 1;
    assert_eq!(Solution::min_knight_moves(x, y), 1);
    let x = 5;
    let y = 5;
    assert_eq!(Solution::min_knight_moves(x, y), 4);
}
