struct Solution;

impl Solution {
    fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut res = 0;
        let mut visited = vec![vec![false; 3]; 3];
        Self::dfs(0, None, &mut visited, &mut res, m as usize, n as usize);
        res as i32
    }

    fn dfs(
        start: usize,
        prev: Option<(usize, usize)>,
        visited: &mut Vec<Vec<bool>>,
        all: &mut usize,
        m: usize,
        n: usize,
    ) {
        if start >= m {
            *all += 1;
        }
        if start == n {
            return;
        }
        if let Some((r, c)) = prev {
            for i in 0..3 {
                for j in 0..3 {
                    if !visited[i][j] {
                        if ((i == r && j + c == 2)
                            || (j == c && i + r == 2)
                            || (i + r == 2 && j + c == 2))
                            && !visited[(i + r) / 2][(j + c) / 2]
                        {
                            continue;
                        }
                        visited[i][j] = true;
                        Self::dfs(start + 1, Some((i, j)), visited, all, m, n);
                        visited[i][j] = false;
                    }
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    visited[i][j] = true;
                    Self::dfs(start + 1, Some((i, j)), visited, all, m, n);
                    visited[i][j] = false;
                }
            }
        }
    }
}

#[test]
fn test() {
    let m = 1;
    let n = 1;
    let res = 9;
    assert_eq!(Solution::number_of_patterns(m, n), res);
    let m = 1;
    let n = 2;
    let res = 65;
    assert_eq!(Solution::number_of_patterns(m, n), res);
}
