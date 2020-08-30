struct Solution;

use std::ops::Range;

impl Solution {
    fn find_min_step(board: String, hand: String) -> i32 {
        let n = hand.len();
        let board: Vec<char> = board.chars().collect();
        let hand: Vec<char> = hand.chars().collect();
        let mut res = std::i32::MAX;
        Self::dfs(0, 0, board, &mut res, &hand, n);
        if res == std::i32::MAX {
            -1
        } else {
            res
        }
    }

    fn dfs(start: usize, state: u32, board: Vec<char>, res: &mut i32, hand: &[char], n: usize) {
        if start == n {
            return;
        }
        for i in 0..board.len() {
            if i == 0 || board[i] != board[i - 1] {
                if let Some(next_state) = Self::find_next_state(board[i], state, hand, n) {
                    let mut new_board: Vec<char> = board.to_vec();
                    new_board.insert(i, board[i]);
                    while let Some(range) = Self::dropable(&new_board) {
                        new_board.drain(range);
                        Self::dfs(start + 1, next_state, new_board.to_vec(), res, hand, n);
                    }
                    if new_board.is_empty() {
                        *res = (*res).min((start + 1) as i32);
                    } else {
                        Self::dfs(start + 1, next_state, new_board, res, hand, n);
                    }
                }
            }
        }
    }

    fn find_next_state(c: char, state: u32, hand: &[char], n: usize) -> Option<u32> {
        for i in 0..n {
            if hand[i] == c && state & 1 << i == 0 {
                return Some(state | 1 << i);
            }
        }
        None
    }

    fn dropable(board: &[char]) -> Option<Range<usize>> {
        let n = board.len();
        let mut l = 0;
        let mut r = 0;
        while r < n {
            if board[l] == board[r] {
                r += 1;
            } else {
                if r - l >= 3 {
                    return Some(l..r);
                } else {
                    l = r;
                }
            }
        }
        if r - l >= 3 {
            return Some(l..r);
        }
        None
    }
}

#[test]
fn test() {
    let board = "WRRBBW".to_string();
    let hand = "RB".to_string();
    let res = -1;
    assert_eq!(Solution::find_min_step(board, hand), res);
    let board = "WWRRBBWW".to_string();
    let hand = "WRBRW".to_string();
    let res = 2;
    assert_eq!(Solution::find_min_step(board, hand), res);
    let board = "G".to_string();
    let hand = "GGGGG".to_string();
    let res = 2;
    assert_eq!(Solution::find_min_step(board, hand), res);
    let board = "RBYYBBRRB".to_string();
    let hand = "YRBGB".to_string();
    let res = 3;
    assert_eq!(Solution::find_min_step(board, hand), res);
    let board = "RRWWRRBBRR".to_string();
    let hand = "WB".to_string();
    let res = 2;
    assert_eq!(Solution::find_min_step(board, hand), res);
}
