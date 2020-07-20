struct Solution;

use std::collections::HashSet;

macro_rules! d {
    ($a:expr, $b:expr) => {
        ($a[0] - $b[0]) * ($a[0] - $b[0]) + ($a[1] - $b[1]) * ($a[1] - $b[1])
    };
}

type Point = Vec<i32>;

impl Solution {
    fn valid_square(p1: Point, p2: Point, p3: Point, p4: Point) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        let v: Vec<Point> = vec![p1, p2, p3, p4];
        for i in 0..4 {
            for j in i + 1..4 {
                hs.insert(d!(v[i], v[j]));
            }
        }
        hs.len() == 2 && !hs.contains(&0)
    }
}

#[test]
fn test() {
    let p1 = vec![0, 0];
    let p2 = vec![1, 1];
    let p3 = vec![1, 0];
    let p4 = vec![0, 1];
    let res = true;
    assert_eq!(Solution::valid_square(p1, p2, p3, p4), res);
    let p1 = vec![0, 0];
    let p2 = vec![1, 1];
    let p3 = vec![0, 0];
    let p4 = vec![0, 0];
    let res = false;
    assert_eq!(Solution::valid_square(p1, p2, p3, p4), res);
}
