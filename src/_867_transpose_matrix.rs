struct Solution;

impl Solution {
    fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        let m = a[0].len();
        let mut res: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                res[j][i] = a[i][j];
            }
        }
        res
    }
}

#[test]
fn test() {
    let a: Vec<Vec<i32>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
    let b: Vec<Vec<i32>> = [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
    assert_eq!(Solution::transpose(a), b);
    let a: Vec<Vec<i32>> = [[1, 2, 3], [4, 5, 6]].iter().map(|x| x.to_vec()).collect();
    let b: Vec<Vec<i32>> = [[1, 4], [2, 5], [3, 6]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
    assert_eq!(Solution::transpose(a), b);
}
