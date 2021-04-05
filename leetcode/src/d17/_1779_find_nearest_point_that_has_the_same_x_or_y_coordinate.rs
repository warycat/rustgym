struct Solution;

impl Solution {
    fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut res: Option<(i32, usize)> = None;
        for i in 0..n {
            if points[i][0] == x || points[i][1] == y {
                let dist = (points[i][0] - x).abs() + (points[i][1] - y).abs();
                if let Some((min_dist, min_index)) = res {
                    res = Some((min_dist, min_index).min((dist, i)));
                } else {
                    res = Some((dist, i));
                }
            }
        }
        if let Some((_, min_index)) = res {
            min_index as i32
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let x = 3;
    let y = 4;
    let points = vec_vec_i32![[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]];
    let res = 2;
    assert_eq!(Solution::nearest_valid_point(x, y, points), res);
    let x = 3;
    let y = 4;
    let points = vec_vec_i32![[3, 4]];
    let res = 0;
    assert_eq!(Solution::nearest_valid_point(x, y, points), res);
    let x = 3;
    let y = 4;
    let points = vec_vec_i32![[2, 3]];
    let res = -1;
    assert_eq!(Solution::nearest_valid_point(x, y, points), res);
}
