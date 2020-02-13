struct NumMatrix {
    matrix: Vec<Vec<i32>>,
    sum: Vec<Vec<i32>>,
    n: usize,
    m: usize,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        if n == 0 {
            return NumMatrix {
                matrix,
                sum: vec![],
                n: 0,
                m: 0,
            };
        }
        let m = matrix[0].len();
        let mut sum = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                sum[i + 1][j + 1] = matrix[i][j];
                sum[i + 1][j + 1] += sum[i + 1][j];
                sum[i + 1][j + 1] += sum[i][j + 1];
                sum[i + 1][j + 1] -= sum[i][j];
            }
        }
        NumMatrix { matrix, sum, n, m }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize + 1;
        let c2 = col2 as usize + 1;
        let mut res = 0;
        res += self.sum[r2][c2];
        res -= self.sum[r1][c2];
        res -= self.sum[r2][c1];
        res += self.sum[r1][c1];
        res
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![
        [3, 0, 1, 4, 2],
        [5, 6, 3, 2, 1],
        [1, 2, 0, 1, 5],
        [4, 1, 0, 1, 7],
        [1, 0, 3, 0, 5]
    ];
    let nm = NumMatrix::new(matrix);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
}
