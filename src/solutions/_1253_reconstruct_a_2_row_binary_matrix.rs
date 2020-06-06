struct Solution;

impl Solution {
    fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = colsum.len();
        let mut res = vec![vec![0; n]; 2];
        for j in 0..n {
            match colsum[j] {
                2 => {
                    res[0][j] = 1;
                    res[1][j] = 1;
                    upper -= 1;
                    lower -= 1;
                }
                1 => {
                    if upper >= lower {
                        res[0][j] = 1;
                        upper -= 1;
                    } else {
                        res[1][j] = 1;
                        lower -= 1;
                    }
                }
                _ => {}
            }
        }
        if upper == 0 && lower == 0 {
            res
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    let upper = 2;
    let lower = 1;
    let colsum = vec![1, 1, 1];
    let res = vec_vec_i32![[1, 1, 0], [0, 0, 1]];
    assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum), res);
    let upper = 2;
    let lower = 3;
    let colsum = vec![2, 2, 1, 1];
    let res = vec_vec_i32![];
    assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum), res);
    let upper = 5;
    let lower = 5;
    let colsum = vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1];
    let res = vec_vec_i32![
        [1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
        [1, 0, 1, 0, 1, 0, 0, 1, 0, 1]
    ];
    assert_eq!(Solution::reconstruct_matrix(upper, lower, colsum), res);
}
