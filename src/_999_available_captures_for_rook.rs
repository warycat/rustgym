struct Solution;

struct Chess {
    r: usize,
    c: usize,
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    fn search_rook(board: &Vec<Vec<char>>) -> Chess {
        for r in 0..8 {
            for c in 0..8 {
                if board[r][c] == 'R' {
                    return Chess { r, c };
                }
            }
        }
        unreachable!()
    }
    fn search_pawn(board: &Vec<Vec<char>>, rook: &Chess, direction: Direction) -> bool {
        let mut r = rook.r;
        let mut c = rook.c;

        match direction {
            Direction::Right => {
                while c + 1 < 8 && board[r][c + 1] != 'B' {
                    c += 1;
                    if board[r][c] == 'p' {
                        return true;
                    }
                }
            }
            Direction::Down => {
                while r + 1 < 8 && board[r + 1][c] != 'B' {
                    r += 1;
                    if board[r][c] == 'p' {
                        return true;
                    }
                }
            }
            Direction::Left => {
                while c > 0 && board[r][c - 1] != 'B' {
                    c -= 1;
                    if board[r][c] == 'p' {
                        return true;
                    }
                }
            }
            Direction::Up => {
                while r > 0 && board[r - 1][c] != 'B' {
                    r -= 1;
                    if board[r][c] == 'p' {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut sum = 0;
        let rook: Chess = Self::search_rook(&board);
        for direction in vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ] {
            if Self::search_pawn(&board, &rook, direction) {
                sum += 1;
            }
        }
        sum
    }
}

#[test]
fn test() {
    let v: [[&str; 8]; 8] = [
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", "p", ".", ".", ".", "."],
        [".", ".", ".", "R", ".", ".", ".", "p"],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", "p", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
    ];
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            board[i][j] = v[i][j].chars().next().unwrap();
        }
    }
    assert_eq!(Solution::num_rook_captures(board), 3);
    let v: [[&str; 8]; 8] = [
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", "p", "p", "p", "p", "p", ".", "."],
        [".", "p", "p", "B", "p", "p", ".", "."],
        [".", "p", "B", "R", "B", "p", ".", "."],
        [".", "p", "p", "B", "p", "p", ".", "."],
        [".", "p", "p", "p", "p", "p", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
    ];
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            board[i][j] = v[i][j].chars().next().unwrap();
        }
    }
    assert_eq!(Solution::num_rook_captures(board), 0);
    let v: [[&str; 8]; 8] = [
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", "p", ".", ".", ".", "."],
        [".", ".", ".", "p", ".", ".", ".", "."],
        ["p", "p", ".", "R", ".", "p", "B", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", "B", ".", ".", ".", "."],
        [".", ".", ".", "p", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
    ];
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            board[i][j] = v[i][j].chars().next().unwrap();
        }
    }
    assert_eq!(Solution::num_rook_captures(board), 3);
}
