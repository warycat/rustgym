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
    fn search_rook(board: &[Vec<char>]) -> Chess {
        for r in 0..8 {
            for c in 0..8 {
                if board[r][c] == 'R' {
                    return Chess { r, c };
                }
            }
        }
        unreachable!()
    }
    fn search_pawn(board: &[Vec<char>], rook: &Chess, direction: Direction) -> bool {
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
    let board = vec_vec_char![
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', 'p', '.', '.', '.', '.'],
        ['.', '.', '.', 'R', '.', '.', '.', 'p'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', 'p', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.']
    ];
    let res = 3;
    assert_eq!(Solution::num_rook_captures(board), res);
    let board = vec_vec_char![
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
        ['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
        ['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
        ['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
        ['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.']
    ];
    let res = 0;
    assert_eq!(Solution::num_rook_captures(board), res);
    let board = vec_vec_char![
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', 'p', '.', '.', '.', '.'],
        ['.', '.', '.', 'p', '.', '.', '.', '.'],
        ['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', 'B', '.', '.', '.', '.'],
        ['.', '.', '.', 'p', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.']
    ];
    let res = 3;
    assert_eq!(Solution::num_rook_captures(board), res);
}
