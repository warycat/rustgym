struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        if n == 0 {
            return false;
        }
        let m = matrix[0].len();
        if m == 0 {
            return false;
        }
        let mut i = 0;
        let mut j = m - 1;
        loop {
            match matrix[i][j].cmp(&target) {
                Equal => {
                    break true;
                }
                Greater => {
                    if j > 0 {
                        j -= 1;
                    } else {
                        break false;
                    }
                }
                Less => {
                    if i + 1 < n {
                        i += 1;
                    } else {
                        break false;
                    }
                }
            }
        }
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30]
    ];
    let target = 5;
    let res = true;
    assert_eq!(Solution::search_matrix(matrix, target), res);
    let matrix = vec_vec_i32![
        [1, 4, 7, 11, 15],
        [2, 5, 8, 12, 19],
        [3, 6, 9, 16, 22],
        [10, 13, 14, 17, 24],
        [18, 21, 23, 26, 30]
    ];
    let target = 20;
    let res = false;
    assert_eq!(Solution::search_matrix(matrix, target), res);
}
