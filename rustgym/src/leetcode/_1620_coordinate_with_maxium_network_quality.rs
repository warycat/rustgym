struct Solution;

use std::cmp::Reverse;

impl Solution {
    fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut max = (0, Reverse(0), Reverse(0));
        for xi in 0..=50 {
            for yi in 0..=50 {
                let mut sum = 0;
                for point in towers.iter() {
                    let x = point[0];
                    let y = point[1];
                    let q = point[2];
                    if (x - xi) * (x - xi) + (y - yi) * (y - yi) > radius * radius {
                        continue;
                    }
                    let d = (((x - xi) * (x - xi) + (y - yi) * (y - yi)) as f64).sqrt();
                    sum += (q as f64 / (1.0 + d)).floor() as i32;
                }
                if (sum, Reverse(xi), Reverse(yi)) > max {
                    max = (sum, Reverse(xi), Reverse(yi));
                }
            }
        }
        vec![(max.1).0, (max.2).0]
    }
}

#[test]
fn test() {
    let towers = vec_vec_i32![[1, 2, 5], [2, 1, 7], [3, 1, 9]];
    let radius = 2;
    let res = vec![2, 1];
    assert_eq!(Solution::best_coordinate(towers, radius), res);
    let towers = vec_vec_i32![[23, 11, 21]];
    let radius = 9;
    let res = vec![23, 11];
    assert_eq!(Solution::best_coordinate(towers, radius), res);
    let towers = vec_vec_i32![[1, 2, 13], [2, 1, 7], [0, 1, 9]];
    let radius = 2;
    let res = vec![1, 2];
    assert_eq!(Solution::best_coordinate(towers, radius), res);
    let towers = vec_vec_i32![[2, 1, 9], [0, 1, 9]];
    let radius = 2;
    let res = vec![0, 1];
    assert_eq!(Solution::best_coordinate(towers, radius), res);
}
