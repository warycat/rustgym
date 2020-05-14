struct Solution;

use std::collections::HashMap;

impl Solution {
    fn distance_square(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])
    }
    fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let a = &points[i];
                let b = &points[j];
                let distance_square = Self::distance_square(a, b);
                let ea = hm.entry(distance_square).or_default();
                *ea += 1;
            }
            for &value in hm.values() {
                if value > 1 {
                    sum += value * (value - 1);
                }
            }
            hm.clear();
        }
        sum
    }
}

#[test]
fn test() {
    let points: Vec<Vec<i32>> = vec_vec_i32![[0, 0], [1, 0], [2, 0]];
    assert_eq!(Solution::number_of_boomerangs(points), 2);
}
