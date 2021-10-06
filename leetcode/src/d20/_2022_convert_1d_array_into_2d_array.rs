struct Solution;

impl Solution {
    fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let k = original.len();
        let m = m as usize;
        let n = n as usize;
        if m * n == k {
            let mut res = vec![];
            let mut index = 0;
            for _ in 0..m {
                let mut row = vec![];
                for _ in 0..n {
                    row.push(original[index]);
                    index += 1;
                }
                res.push(row);
            }
            res
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    let original = vec![1, 2, 3, 4];
    let m = 2;
    let n = 2;
    let res = vec_vec_i32![[1, 2], [3, 4]];
    assert_eq!(Solution::construct2_d_array(original, m, n), res);
    let original = vec![1, 2, 3];
    let m = 1;
    let n = 3;
    let res = vec_vec_i32![[1, 2, 3]];
    assert_eq!(Solution::construct2_d_array(original, m, n), res);
    let original = vec![1, 2];
    let m = 1;
    let n = 1;
    let res = vec_vec_i32![];
    assert_eq!(Solution::construct2_d_array(original, m, n), res);
    let original = vec![3];
    let m = 1;
    let n = 2;
    let res = vec_vec_i32![];
    assert_eq!(Solution::construct2_d_array(original, m, n), res);
}
