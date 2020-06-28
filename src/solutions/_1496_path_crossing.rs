struct Solution;
use std::collections::HashSet;

impl Solution {
    fn is_path_crossing(path: String) -> bool {
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        hs.insert((0, 0));
        let mut x = 0;
        let mut y = 0;
        for c in path.chars() {
            match c {
                'N' => {
                    y += 1;
                }
                'S' => {
                    y -= 1;
                }
                'E' => {
                    x += 1;
                }
                _ => {
                    x -= 1;
                }
            }
            if !hs.insert((x, y)) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let path = "NES".to_string();
    let res = false;
    assert_eq!(Solution::is_path_crossing(path), res);
    let path = "NESWW".to_string();
    let res = true;
    assert_eq!(Solution::is_path_crossing(path), res);
}
