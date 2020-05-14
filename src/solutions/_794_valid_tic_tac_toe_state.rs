struct Solution;

impl Solution {
    fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut a = vec![vec![]; 3];
        let mut x = 0;
        let mut o = 0;
        for (i, row) in board.iter().enumerate() {
            for c in row.chars() {
                match c {
                    'X' => {
                        x += 1;
                    }
                    'O' => {
                        o += 1;
                    }
                    _ => {}
                }
                a[i].push(c);
            }
        }
        let win_x = Self::win(&a, 'X');
        let win_o = Self::win(&a, 'O');
        x == o + 1 && win_x >= 0 && win_o == 0 || x == o && win_x == 0 && win_o <= 1
    }

    fn win(board: &[Vec<char>], c: char) -> i32 {
        let mut rows = vec![0; 3];
        let mut cols = vec![0; 3];
        let mut diagonals = vec![0; 2];
        for i in 0..3 {
            for j in 0..3 {
                let v = if board[i][j] == c { 1 } else { 0 };
                rows[i] += v;
                cols[j] += v;
                if i == j {
                    diagonals[0] += v;
                }
                if i + j == 2 {
                    diagonals[1] += v;
                }
            }
        }
        let mut sum = 0;
        if rows.iter().any(|&row| row == 3) {
            sum += 1;
        }
        if cols.iter().any(|&col| col == 3) {
            sum += 1;
        }
        if diagonals.iter().any(|&diagonal| diagonal == 3) {
            sum += 1;
        }
        sum
    }
}

#[test]
fn test() {
    let board = vec_string!["O  ", "   ", "   "];
    let res = false;
    assert_eq!(Solution::valid_tic_tac_toe(board), res);
    let board = vec_string!["XOX", " X ", "   "];
    let res = false;
    assert_eq!(Solution::valid_tic_tac_toe(board), res);
    let board = vec_string!["XXX", "   ", "OOO"];
    let res = false;
    assert_eq!(Solution::valid_tic_tac_toe(board), res);
    let board = vec_string!["XOX", "O O", "XOX"];
    let res = true;
    assert_eq!(Solution::valid_tic_tac_toe(board), res);
    let board = vec_string!["XXX", "OOX", "OOX"];
    let res = true;
    assert_eq!(Solution::valid_tic_tac_toe(board), res);
}
