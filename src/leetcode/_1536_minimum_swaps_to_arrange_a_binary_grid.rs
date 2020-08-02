struct Solution;

impl Solution {
    fn min_swaps(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut res = 0;
        'outer: for i in 0..n {
            for j in i..n {
                if grid[j][i + 1..].iter().all(|&x| x == 0) {
                    let mut k = j;
                    while k > i {
                        grid.swap(k, k - 1);
                        res += 1;
                        k -= 1;
                    }
                    continue 'outer;
                }
            }
            return -1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[0, 0, 1], [1, 1, 0], [1, 0, 0]];
    let res = 3;
    assert_eq!(Solution::min_swaps(grid), res);
    let grid = vec_vec_i32![
        [1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 1]
    ];
    let res = 4;
    assert_eq!(Solution::min_swaps(grid), res);
    let grid = vec_vec_i32![[0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0]];
    let res = -1;
    assert_eq!(Solution::min_swaps(grid), res);
    let grid = vec_vec_i32![[1, 0, 0], [1, 1, 0], [1, 1, 1]];
    let res = 0;
    assert_eq!(Solution::min_swaps(grid), res);
}
