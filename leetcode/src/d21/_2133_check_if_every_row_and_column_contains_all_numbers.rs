struct Solution;

impl Solution {
    fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        for i in 0..n {
            let mut col = vec![false; n];
            for j in 0..n {
                let x = matrix[i][j];
                if (1..=n as i32).contains(&x) {
                    let k = (x - 1) as usize;
                    col[k] = true;
                } else {
                    return false;
                }
            }
            for j in 0..n {
                if !col[j] {
                    return false;
                }
            }
        }
        for i in 0..n {
            let mut row = vec![false; n];
            for j in 0..n {
                let x = matrix[j][i];
                if (1..=n as i32).contains(&x) {
                    let k = (x - 1) as usize;
                    row[k] = true;
                } else {
                    return false;
                }
            }
            for j in 0..n {
                if !row[j] {
                    return false;
                }
            }
        }

        true
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[1, 2, 3], [3, 1, 2], [2, 3, 1]];
    let res = true;
    assert_eq!(Solution::check_valid(matrix), res);
    let matrix = vec_vec_i32![[1, 1, 1], [1, 2, 3], [1, 2, 3]];
    let res = false;
    assert_eq!(Solution::check_valid(matrix), res);
}
