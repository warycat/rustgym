struct Solution;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Solution {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        if n == 0 {
            return vec![];
        }
        let m = matrix[0].len();
        if m == 0 {
            return vec![];
        }
        let mut res: Vec<i32> = vec![];
        let mut i = 0;
        let mut j = 0;
        let mut left = 0;
        let mut top = 0;
        let mut bottom = n - 1;
        let mut right = m - 1;
        let mut direction = Direction::Right;
        loop {
            res.push(matrix[i][j]);
            match direction {
                Direction::Right => {
                    if j < right {
                        j += 1;
                    } else {
                        if top < bottom {
                            top += 1;
                            direction = Direction::Down;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }
                Direction::Down => {
                    if i < bottom {
                        i += 1;
                    } else {
                        if left < right {
                            right -= 1;
                            direction = Direction::Left;
                            j -= 1;
                        } else {
                            break;
                        }
                    }
                }
                Direction::Left => {
                    if j > left {
                        j -= 1;
                    } else {
                        if top < bottom {
                            bottom -= 1;
                            direction = Direction::Up;
                            i -= 1;
                        } else {
                            break;
                        }
                    }
                }
                Direction::Up => {
                    if i > top {
                        i -= 1;
                    } else {
                        if left < right {
                            left += 1;
                            direction = Direction::Right;
                            j += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    assert_eq!(Solution::spiral_order(matrix), res);
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[3], [2]];
    let res = vec![3, 2];
    assert_eq!(Solution::spiral_order(matrix), res);
}
