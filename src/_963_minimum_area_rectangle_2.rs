struct Solution;
use std::collections::HashMap;

type Cross = (i32, i32, i32);
type Point = (i32, i32);

impl Solution {
    fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let mut hm: HashMap<Cross, Vec<Point>> = HashMap::new();
        let n = points.len();
        let mut res = std::f64::MAX;
        for i in 0..n {
            for j in i + 1..n {
                let x1 = points[i][0];
                let x2 = points[j][0];
                let y1 = points[i][1];
                let y2 = points[j][1];
                let c = (
                    x1 + x2,
                    y1 + y2,
                    (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1),
                );
                let points = hm.entry(c).or_default();
                for point in points.iter() {
                    let e1 = Self::edge(x1, y1, point.0, point.1);
                    let e2 = Self::edge(x2, y2, point.0, point.1);
                    res = res.min(e1 * e2);
                }
                points.push((x1, y1));
            }
        }
        if res == std::f64::MAX {
            0.0
        } else {
            res
        }
    }
    fn edge(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
        (((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f64).sqrt()
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let points = vec_vec_i32![[1, 2], [2, 1], [1, 0], [0, 1]];
    let res = 2.0;
    assert_approx_eq!(Solution::min_area_free_rect(points), res);
    let points = vec_vec_i32![[0, 1], [2, 1], [1, 1], [1, 0], [2, 0]];
    let res = 1.0;
    assert_approx_eq!(Solution::min_area_free_rect(points), res);
    let points = vec_vec_i32![[0, 3], [1, 2], [3, 1], [1, 3], [2, 1]];
    let res = 0.0;
    assert_approx_eq!(Solution::min_area_free_rect(points), res);
    let points = vec_vec_i32![
        [3, 1],
        [1, 1],
        [0, 1],
        [2, 1],
        [3, 3],
        [3, 2],
        [0, 2],
        [2, 3]
    ];
    let res = 2.0;
    assert_approx_eq!(Solution::min_area_free_rect(points), res);
}
