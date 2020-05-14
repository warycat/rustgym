struct Solution;

use std::collections::HashSet;

impl Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                if !rows[i].insert(c) {
                    return false;
                }

                if !cols[j].insert(c) {
                    return false;
                }

                let k = (i / 3) * 3 + (j / 3);
                if !boxes[k].insert(c) {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let board = vec_vec_char![
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
    assert_eq!(Solution::is_valid_sudoku(board), true);
    let board = vec_vec_char![
        ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ];
    assert_eq!(Solution::is_valid_sudoku(board), false);
}
