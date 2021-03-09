struct Solution;

impl Solution {
    fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        let mut visited = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                let c = grid[i][j];
                if Self::dfs(
                    i,
                    j,
                    0,
                    std::usize::MAX,
                    std::usize::MAX,
                    &mut visited,
                    &grid,
                    c,
                    n,
                    m,
                ) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        i: usize,
        j: usize,
        dist: usize,
        pi: usize,
        pj: usize,
        visited: &mut Vec<Vec<bool>>,
        grid: &[Vec<char>],
        c: char,
        n: usize,
        m: usize,
    ) -> bool {
        if dist >= 4 && visited[i][j] {
            return true;
        }
        if visited[i][j] {
            return false;
        }
        visited[i][j] = true;
        if i > 0 && grid[i - 1][j] == c && i - 1 != pi {
            if Self::dfs(i - 1, j, dist + 1, i, j, visited, grid, c, n, m) {
                return true;
            }
        }
        if j > 0 && grid[i][j - 1] == c && j - 1 != pj {
            if Self::dfs(i, j - 1, dist + 1, i, j, visited, grid, c, n, m) {
                return true;
            }
        }
        if i + 1 < n && grid[i + 1][j] == c && i + 1 != pi {
            if Self::dfs(i + 1, j, dist + 1, i, j, visited, grid, c, n, m) {
                return true;
            }
        }
        if j + 1 < m && grid[i][j + 1] == c && j + 1 != pj {
            if Self::dfs(i, j + 1, dist + 1, i, j, visited, grid, c, n, m) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let grid = vec_vec_char![
        ['a', 'a', 'a', 'a'],
        ['a', 'b', 'b', 'a'],
        ['a', 'b', 'b', 'a'],
        ['a', 'a', 'a', 'a']
    ];
    let res = true;
    assert_eq!(Solution::contains_cycle(grid), res);
    let grid = vec_vec_char![
        ['c', 'c', 'c', 'a'],
        ['c', 'd', 'c', 'c'],
        ['c', 'c', 'e', 'c'],
        ['f', 'c', 'c', 'c']
    ];
    let res = true;
    assert_eq!(Solution::contains_cycle(grid), res);
    let grid = vec_vec_char![['a', 'b', 'b'], ['b', 'z', 'b'], ['b', 'b', 'a']];
    let res = false;
    assert_eq!(Solution::contains_cycle(grid), res);
    let grid = vec_vec_char![
        ['c', 'a', 'd'],
        ['a', 'a', 'a'],
        ['a', 'a', 'd'],
        ['a', 'c', 'd'],
        ['a', 'b', 'c']
    ];
    let res = true;
    assert_eq!(Solution::contains_cycle(grid), res);
    let grid = vec_vec_char![
        ['b', 'a', 'c'],
        ['c', 'a', 'c'],
        ['d', 'd', 'c'],
        ['b', 'c', 'c']
    ];
    let res = false;
    assert_eq!(Solution::contains_cycle(grid), res);
}
