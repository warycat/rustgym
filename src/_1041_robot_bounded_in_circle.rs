struct Solution;

impl Solution {
    fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut i = 0;
        let d = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        for c in instructions.chars() {
            match c {
                'G' => {
                    x += d[i][0];
                    y += d[i][1];
                }
                'L' => {
                    i = (i + 1) % 3;
                }
                'R' => {
                    i = (i + 3) % 3;
                }
                _ => (),
            }
        }
        x == 0 && y == 0 || i != 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_robot_bounded("GGLLGG".to_string()), true);
    assert_eq!(Solution::is_robot_bounded("GG".to_string()), false);
    assert_eq!(Solution::is_robot_bounded("GL".to_string()), true);
}
