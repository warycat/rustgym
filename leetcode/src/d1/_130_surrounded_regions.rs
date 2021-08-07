struct Solution;

impl Solution {
    fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        if n == 0 {
            return;
        }
        let m = board[0].len();
        let mut visited = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                    if board[i][j] == 'O' && !visited[i][j] {
                        Self::dfs(i, j, &mut visited, board, n, m);
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if !visited[i][j] {
                    board[i][j] = 'X';
                }
            }
        }
    }

    fn dfs(i: usize, j: usize, visited: &mut [Vec<bool>], board: &[Vec<char>], n: usize, m: usize) {
        visited[i][j] = true;
        if i > 0 && !visited[i - 1][j] && board[i - 1][j] == 'O' {
            Self::dfs(i - 1, j, visited, board, n, m);
        }
        if j > 0 && !visited[i][j - 1] && board[i][j - 1] == 'O' {
            Self::dfs(i, j - 1, visited, board, n, m);
        }
        if i + 1 < n && !visited[i + 1][j] && board[i + 1][j] == 'O' {
            Self::dfs(i + 1, j, visited, board, n, m);
        }
        if j + 1 < m && !visited[i][j + 1] && board[i][j + 1] == 'O' {
            Self::dfs(i, j + 1, visited, board, n, m);
        }
    }
}

#[test]
fn test() {
    let mut board = vec_vec_char![
        ['X', 'X', 'X', 'X'],
        ['X', 'O', 'O', 'X'],
        ['X', 'X', 'O', 'X'],
        ['X', 'O', 'X', 'X']
    ];
    let res = vec_vec_char![
        ['X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X'],
        ['X', 'X', 'X', 'X'],
        ['X', 'O', 'X', 'X']
    ];
    Solution::solve(&mut board);
    assert_eq!(board, res);
}
