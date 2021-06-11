struct Solution;

trait Rotation {
    fn rotate(self) -> Vec<Vec<i32>>;
}

impl Rotation for Vec<Vec<i32>> {
    fn rotate(self) -> Vec<Vec<i32>> {
        let n = self.len();
        let mut mat: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                mat[j][n - 1 - i] = self[i][j];
            }
        }
        mat
    }
}

impl Solution {
    fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if mat == target {
            return true;
        }
        let mat90 = mat.rotate();
        if mat90 == target {
            return true;
        }
        let mat180 = mat90.rotate();
        if mat180 == target {
            return true;
        }
        let mat270 = mat180.rotate();
        if mat270 == target {
            return true;
        }
        false
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[0, 1], [1, 0]];
    let target = vec_vec_i32![[1, 0], [0, 1]];
    let res = true;
    assert_eq!(Solution::find_rotation(mat, target), res);
    let mat = vec_vec_i32![[0, 1], [1, 1]];
    let target = vec_vec_i32![[1, 0], [0, 1]];
    let res = false;
    assert_eq!(Solution::find_rotation(mat, target), res);
    let mat = vec_vec_i32![[0, 0, 0], [0, 1, 0], [1, 1, 1]];
    let target = vec_vec_i32![[1, 1, 1], [0, 1, 0], [0, 0, 0]];
    let res = true;
    assert_eq!(Solution::find_rotation(mat, target), res);
}
