struct Solution;

impl Solution {
    fn get_max_grid_happiness(n: i32, m: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let mut res = 0;
        let m = m as usize;
        let n = n as usize;
        let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut cur = 0;
        Self::dfs(
            0,
            introverts_count,
            extroverts_count,
            &mut grid,
            &mut cur,
            &mut res,
            n,
            m,
        );
        res
    }
    fn dfs(
        start: usize,
        a: i32,
        b: i32,
        grid: &mut Vec<Vec<i32>>,
        cur: &mut i32,
        max: &mut i32,
        n: usize,
        m: usize,
    ) {
        if a == 0 && b == 0 || start == n * m {
            *max = (*max).max(*cur);
            return;
        }
        if *cur + a * 120 + b * 120 <= *max {
            return;
        }
        let i = start / m;
        let j = start % m;
        if a > 0 {
            grid[i][j] = 1;
            let mut base = 120;
            if i > 0 && grid[i - 1][j] == 1 {
                base -= 60;
            }
            if i > 0 && grid[i - 1][j] == 2 {
                base -= 10;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                base -= 60;
            }
            if j > 0 && grid[i][j - 1] == 2 {
                base -= 10;
            }
            *cur += base;
            Self::dfs(start + 1, a - 1, b, grid, cur, max, n, m);
            *cur -= base;
            grid[i][j] = 0;
        }
        if b > 0 {
            grid[i][j] = 2;
            let mut base = 40;
            if i > 0 && grid[i - 1][j] == 1 {
                base -= 10;
            }
            if i > 0 && grid[i - 1][j] == 2 {
                base += 40;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                base -= 10;
            }
            if j > 0 && grid[i][j - 1] == 2 {
                base += 40;
            }
            *cur += base;
            Self::dfs(start + 1, a, b - 1, grid, cur, max, n, m);
            *cur -= base;
            grid[i][j] = 0;
        }
        if (i > 0 && grid[i - 1][j] != 0) || (j > 0 && grid[i][j - 1] != 0) {
            Self::dfs(start + 1, a, b, grid, cur, max, n, m);
        }
    }

    fn happiness(grid: &[Vec<i32>], n: usize, m: usize) -> i32 {
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                let mut start = 0;
                match grid[i][j] {
                    1 => {
                        start = 120;
                        if i > 0 && grid[i - 1][j] != 0 {
                            start -= 30;
                        }
                        if j > 0 && grid[i][j - 1] != 0 {
                            start -= 30;
                        }
                        if i + 1 < n && grid[i + 1][j] != 0 {
                            start -= 30;
                        }
                        if j + 1 < m && grid[i][j + 1] != 0 {
                            start -= 30;
                        }
                    }
                    2 => {
                        start = 40;
                        if i > 0 && grid[i - 1][j] != 0 {
                            start += 20;
                        }
                        if j > 0 && grid[i][j - 1] != 0 {
                            start += 20;
                        }
                        if i + 1 < n && grid[i + 1][j] != 0 {
                            start += 20;
                        }
                        if j + 1 < m && grid[i][j + 1] != 0 {
                            start += 20;
                        }
                    }
                    _ => {}
                }
                res += start;
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let m = 3;
    let introverts_count = 1;
    let extroverts_count = 2;
    let res = 240;
    assert_eq!(
        Solution::get_max_grid_happiness(n, m, introverts_count, extroverts_count),
        res
    );
    let n = 3;
    let m = 1;
    let introverts_count = 2;
    let extroverts_count = 1;
    let res = 260;
    assert_eq!(
        Solution::get_max_grid_happiness(n, m, introverts_count, extroverts_count),
        res
    );
    let n = 2;
    let m = 2;
    let introverts_count = 4;
    let extroverts_count = 0;
    let res = 240;
    assert_eq!(
        Solution::get_max_grid_happiness(n, m, introverts_count, extroverts_count),
        res
    );
}
