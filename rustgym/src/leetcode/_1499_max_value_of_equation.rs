struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = points.len();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut res = std::i32::MIN;
        for j in 0..n {
            let xj = points[j][0];
            let yj = points[j][1];
            while let Some(&(_, xi)) = queue.front() {
                if xi + k < xj {
                    queue.pop_front();
                } else {
                    break;
                }
            }
            if let Some(&(diff, _)) = queue.front() {
                res = res.max(diff + yj + xj);
            }
            while let Some(&(diff, xi)) = queue.back() {
                if (diff, xi) < (yj - xj, xj) {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back((yj - xj, xj));
        }
        res
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[1, 3], [2, 0], [5, 10], [6, -10]];
    let k = 1;
    let res = 4;
    assert_eq!(Solution::find_max_value_of_equation(points, k), res);
    let points = vec_vec_i32![[0, 0], [3, 0], [9, 2]];
    let k = 3;
    let res = 3;
    assert_eq!(Solution::find_max_value_of_equation(points, k), res);
}
