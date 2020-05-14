struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        use Direction::*;
        let n = n as usize;
        let mut res: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut i = 0;
        let mut j = 0;
        let mut d = Right;
        for k in 1..=n * n {
            res[i][j] = k as i32;
            match d {
                Right => {
                    if j + 1 < n && res[i][j + 1] == 0 {
                        j += 1;
                    } else {
                        d = Down;
                        i += 1;
                    }
                }
                Down => {
                    if i + 1 < n && res[i + 1][j] == 0 {
                        i += 1;
                    } else {
                        d = Left;
                        j -= 1;
                    }
                }
                Left => {
                    if j > 0 && res[i][j - 1] == 0 {
                        j -= 1;
                    } else {
                        d = Up;
                        i -= 1;
                    }
                }
                Up => {
                    if i > 0 && res[i - 1][j] == 0 {
                        i -= 1;
                    } else {
                        d = Right;
                        j += 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3], [8, 9, 4], [7, 6, 5]];
    assert_eq!(Solution::generate_matrix(n), res);
}
