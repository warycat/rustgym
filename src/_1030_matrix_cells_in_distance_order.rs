struct Solution;

impl Solution {
    fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut cells: Vec<Vec<i32>> = vec![];
        for i in 0..r {
            for j in 0..c {
                cells.push(vec![i, j]);
            }
        }
        cells.sort_unstable_by_key(|v| (v[0] - r0).abs() + (v[1] - c0).abs());
        cells
    }
}

#[test]
fn test() {
    let r = 1;
    let c = 2;
    let r0 = 0;
    let c0 = 0;
    let res: Vec<Vec<i32>> = vec_vec_i32![[0, 0], [0, 1]];
    assert_eq!(Solution::all_cells_dist_order(r, c, r0, c0), res);
    let r = 2;
    let c = 2;
    let r0 = 0;
    let c0 = 1;
    let res: Vec<Vec<i32>> = vec_vec_i32![[0, 1], [0, 0], [1, 1], [1, 0]];
    assert_eq!(Solution::all_cells_dist_order(r, c, r0, c0), res);
    let r = 2;
    let c = 3;
    let r0 = 1;
    let c0 = 2;
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [0, 2], [1, 1], [0, 1], [1, 0], [0, 0]];
    assert_eq!(Solution::all_cells_dist_order(r, c, r0, c0), res);
}
