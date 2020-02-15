struct Solution;
use std::collections::HashSet;
use std::i32;

impl Solution {
    fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..n {
            hs.insert((points[i][0], points[i][1]));
        }
        let mut min = i32::MAX;
        for i in 0..n - 1 {
            for j in i + 1..n {
                let x1 = points[i][0];
                let y1 = points[i][1];
                let x2 = points[j][0];
                let y2 = points[j][1];
                if x2 != x1 && y2 != y1 && hs.contains(&(x1, y2)) && hs.contains(&(x2, y1)) {
                    min = min.min((x2 - x1).abs() * (y2 - y1).abs())
                }
            }
        }
        if min == i32::MAX {
            0
        } else {
            min
        }
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[1, 1], [1, 3], [3, 1], [3, 3], [2, 2]];
    let res = 4;
    assert_eq!(Solution::min_area_rect(points), res);
    let points = vec_vec_i32![[1, 1], [1, 3], [3, 1], [3, 3], [4, 1], [4, 3]];
    let res = 2;
    assert_eq!(Solution::min_area_rect(points), res);
}
