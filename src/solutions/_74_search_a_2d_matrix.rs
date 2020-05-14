struct Solution;

impl Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut a = vec![];
        for row in matrix {
            for x in row {
                a.push(x);
            }
        }
        a.binary_search(&target).is_ok()
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]];
    let target = 3;
    let res = true;
    assert_eq!(Solution::search_matrix(matrix, target), res);
}
