struct Solution;

impl Solution {
    fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for c in moves.chars() {
            match c {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => (),
            }
        }
        x == 0 && y == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::judge_circle("UD".to_string()), true);
    assert_eq!(Solution::judge_circle("LL".to_string()), false);
}
