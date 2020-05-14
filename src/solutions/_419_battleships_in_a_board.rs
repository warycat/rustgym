struct Solution;

impl Solution {
    fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let n = board.len();
        let m = board[0].len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if Self::is_head(i, j, &board) {
                    res += 1;
                }
            }
        }
        res
    }

    fn is_head(i: usize, j: usize, board: &[Vec<char>]) -> bool {
        if board[i][j] == '.' {
            return false;
        }
        if i > 0 && board[i - 1][j] == 'X' {
            return false;
        }
        if j > 0 && board[i][j - 1] == 'X' {
            return false;
        }
        true
    }
}

#[test]
fn test() {
    let board = vec_vec_char![
        ['X', '.', '.', 'X'],
        ['.', '.', '.', 'X'],
        ['.', '.', '.', 'X']
    ];
    let res = 2;
    assert_eq!(Solution::count_battleships(board), res);
}
