struct Solution;

impl Solution {
    fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let total = (r * c) as usize;
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut d = 0;
        let mut step = 1;
        let mut i = r0;
        let mut j = c0;
        let mut k = 0;
        while res.len() < total {
            if i >= 0 && i < r && j >= 0 && j < c {
                res.push(vec![i, j]);
            }
            i += directions[d].0;
            j += directions[d].1;
            k += 1;
            if k == step {
                d += 1;
                d %= 4;
                k = 0;
                if d % 2 == 0 {
                    step += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let r = 1;
    let c = 4;
    let r0 = 0;
    let c0 = 0;
    let res = vec_vec_i32![[0, 0], [0, 1], [0, 2], [0, 3]];
    assert_eq!(Solution::spiral_matrix_iii(r, c, r0, c0), res);
    let r = 5;
    let c = 6;
    let r0 = 1;
    let c0 = 4;
    let res = vec_vec_i32![
        [1, 4],
        [1, 5],
        [2, 5],
        [2, 4],
        [2, 3],
        [1, 3],
        [0, 3],
        [0, 4],
        [0, 5],
        [3, 5],
        [3, 4],
        [3, 3],
        [3, 2],
        [2, 2],
        [1, 2],
        [0, 2],
        [4, 5],
        [4, 4],
        [4, 3],
        [4, 2],
        [4, 1],
        [3, 1],
        [2, 1],
        [1, 1],
        [0, 1],
        [4, 0],
        [3, 0],
        [2, 0],
        [1, 0],
        [0, 0]
    ];
    assert_eq!(Solution::spiral_matrix_iii(r, c, r0, c0), res);
}
