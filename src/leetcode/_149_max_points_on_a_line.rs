struct Solution;

use std::collections::HashMap;

impl Solution {
    fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 0 {
            return 0;
        }
        let mut res = 1;
        for i in 0..n {
            res = res.max(Self::from_point(i, &points));
        }
        res as i32
    }

    fn from_point(i: usize, points: &[Vec<i32>]) -> usize {
        let n = points.len();
        let x1 = points[i][0];
        let y1 = points[i][1];
        let mut hm: HashMap<(i32, i32), usize> = HashMap::new();
        let mut origin = 0;
        for j in 0..n {
            let x2 = points[j][0];
            let y2 = points[j][1];
            let mut dx = x2 - x1;
            let mut dy = y2 - y1;
            if dx == 0 && dy == 0 {
                origin += 1;
            } else {
                if dx == 0 {
                    *hm.entry((0, 1)).or_default() += 1;
                    continue;
                }
                if dy == 0 {
                    *hm.entry((1, 0)).or_default() += 1;
                    continue;
                }
                if dy < 0 {
                    dy *= -1;
                    dx *= -1;
                }
                let z = Self::gcd(dx, dy);
                dy /= z;
                dx /= z;
                *hm.entry((dx, dy)).or_default() += 1;
            }
        }
        hm.values().max().unwrap_or(&0) + origin
    }

    fn gcd(mut m: i32, mut n: i32) -> i32 {
        while m != 0 {
            let temp = m;
            m = n % temp;
            n = temp;
        }
        n.abs()
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[1, 1], [2, 2], [3, 3]];
    let res = 3;
    assert_eq!(Solution::max_points(points), res);
    let points = vec_vec_i32![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    let res = 4;
    assert_eq!(Solution::max_points(points), res);
}
