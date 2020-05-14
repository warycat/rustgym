struct Solution;

impl Solution {
    fn judge_square_sum(c: i32) -> bool {
        if c == 0 {
            return true;
        }
        let x = (c as f64).sqrt() as i32;
        for a in 0..=x {
            let bb = c - a * a;
            let b = (bb as f64).sqrt() as i32;
            if bb == b * b {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(3), false);
    assert_eq!(Solution::judge_square_sum(2_147_483_646), false);
}
