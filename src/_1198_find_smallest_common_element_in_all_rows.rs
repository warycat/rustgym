struct Solution;

impl Solution {
    fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut count: Vec<usize> = vec![0; 10001];
        let n = mat.len();
        let m = mat[0].len();
        for i in 0..n {
            for j in 0..m {
                count[mat[i][j] as usize] += 1;
            }
        }
        for j in 0..m {
            let x = mat[0][j];
            if count[x as usize] == n {
                return x;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![
        [1, 2, 3, 4, 5],
        [2, 4, 5, 8, 10],
        [3, 5, 7, 9, 11],
        [1, 3, 5, 7, 9]
    ];
    let res = 5;
    assert_eq!(Solution::smallest_common_element(mat), res);
}
