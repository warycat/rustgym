struct Solution;

struct BinaryMatrix {
    data: Vec<Vec<i32>>,
}

impl BinaryMatrix {
    fn new(data: Vec<Vec<i32>>) -> Self {
        BinaryMatrix { data }
    }
    fn dimensions(&self) -> Vec<i32> {
        vec![self.data.len() as i32, self.data[0].len() as i32]
    }
    fn get(&self, i: i32, j: i32) -> i32 {
        self.data[i as usize][j as usize]
    }
}

impl Solution {
    fn left_most_column_with_one(matrix: &BinaryMatrix) -> i32 {
        let d = matrix.dimensions();
        let n = d[0];
        let m = d[1];
        let mut j = m;
        while j > 0 {
            let mut count = 0;
            for i in 0..n {
                while j > 0 && matrix.get(i, j - 1) == 1 {
                    count += 1;
                    j -= 1;
                }
            }
            if count == 0 {
                break;
            }
        }
        if j == m {
            -1
        } else {
            j as i32
        }
    }
}

#[test]
fn test() {
    let data = vec_vec_i32![[0, 0], [1, 1]];
    let matrix = BinaryMatrix::new(data);
    let res = 0;
    assert_eq!(Solution::left_most_column_with_one(&matrix), res);

    let data = vec_vec_i32![[0, 0], [0, 1]];
    let matrix = BinaryMatrix::new(data);
    let res = 1;
    assert_eq!(Solution::left_most_column_with_one(&matrix), res);

    let data = vec_vec_i32![[0, 0], [0, 0]];
    let matrix = BinaryMatrix::new(data);
    let res = -1;
    assert_eq!(Solution::left_most_column_with_one(&matrix), res);

    let data = vec_vec_i32![[0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1]];
    let matrix = BinaryMatrix::new(data);
    let res = 1;
    assert_eq!(Solution::left_most_column_with_one(&matrix), res);
}
