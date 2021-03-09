struct Solution;

impl Solution {
    fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![0; 9];
        let mut cols = vec![0; 9];
        let mut zones = vec![vec![0; 3]; 3];
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c != '.' {
                    let bit = 1 << (c as u8 - b'1');
                    rows[i] |= bit;
                    cols[j] |= bit;
                    zones[i / 3][j / 3] |= bit;
                }
            }
        }
        Self::dfs(0, board, &mut rows, &mut cols, &mut zones);
    }

    fn dfs(
        start: usize,
        board: &mut Vec<Vec<char>>,
        rows: &mut Vec<u32>,
        cols: &mut Vec<u32>,
        zones: &mut Vec<Vec<u32>>,
    ) -> bool {
        if start == 81 {
            return true;
        }
        let i = start / 9;
        let j = start % 9;
        if board[i][j] == '.' {
            for k in (0..9).rev() {
                let bit = 1 << k;
                if rows[i] & bit == 0 && cols[j] & bit == 0 && zones[i / 3][j / 3] & bit == 0 {
                    board[i][j] = (b'1' + k as u8) as char;
                    rows[i] |= bit;
                    cols[j] |= bit;
                    zones[i / 3][j / 3] |= bit;
                    if Self::dfs(start + 1, board, rows, cols, zones) {
                        return true;
                    }
                    board[i][j] = '.';
                    rows[i] &= !bit;
                    cols[j] &= !bit;
                    zones[i / 3][j / 3] &= !bit;
                }
            }
            false
        } else {
            Self::dfs(start + 1, board, rows, cols, zones)
        }
    }
}

#[test]
fn test() {
    let mut board = vec_vec_char![
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ];
    let res = vec_vec_char![
        ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        ['3', '4', '5', '2', '8', '6', '1', '7', '9']
    ];
    Solution::solve_sudoku(&mut board);
    assert_eq!(board, res);
}
