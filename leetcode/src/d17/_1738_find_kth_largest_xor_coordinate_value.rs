struct Solution;

impl Solution {
    fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let k = k as usize;
        let mut xor = vec![vec![0; m]; n];
        xor[0][0] = matrix[0][0];
        let mut arr = vec![];
        for i in 1..n {
            xor[i][0] = xor[i - 1][0] ^ matrix[i][0];
        }
        for j in 1..m {
            xor[0][j] = xor[0][j - 1] ^ matrix[0][j];
        }
        for i in 1..n {
            for j in 1..m {
                xor[i][j] = xor[i - 1][j - 1] ^ xor[i][j - 1] ^ xor[i - 1][j] ^ matrix[i][j];
            }
        }
        for i in 0..n {
            for j in 0..m {
                arr.push(xor[i][j]);
            }
        }
        arr.select_nth_unstable(n * m - k);
        arr[n * m - k]
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[5, 2], [1, 6]];
    let k = 1;
    let res = 7;
    assert_eq!(Solution::kth_largest_value(matrix, k), res);
    let matrix = vec_vec_i32![[5, 2], [1, 6]];
    let k = 2;
    let res = 5;
    assert_eq!(Solution::kth_largest_value(matrix, k), res);
    let matrix = vec_vec_i32![[5, 2], [1, 6]];
    let k = 3;
    let res = 4;
    assert_eq!(Solution::kth_largest_value(matrix, k), res);
    let matrix = vec_vec_i32![[5, 2], [1, 6]];
    let k = 4;
    let res = 0;
    assert_eq!(Solution::kth_largest_value(matrix, k), res);
}
