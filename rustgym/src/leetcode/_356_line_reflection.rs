struct Solution;

use std::collections::HashSet;

impl Solution {
    fn is_reflected(points: Vec<Vec<i32>>) -> bool {
        let min = points.iter().map(|v| v[0]).min().unwrap();
        let max = points.iter().map(|v| v[0]).max().unwrap();
        let x = max + min;
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        for v in &points {
            hs.insert((v[0], v[1]));
        }
        for v in &points {
            if !hs.contains(&(x - v[0], v[1])) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[1, 1], [-1, 1]];
    let res = true;
    assert_eq!(Solution::is_reflected(points), res);
    let points = vec_vec_i32![[1, 1], [-1, -1]];
    let res = false;
    assert_eq!(Solution::is_reflected(points), res);
}
