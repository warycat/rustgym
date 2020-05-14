pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if let Some(mut word_search) = WordSearch::new(board, word) {
            word_search.exist()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct WordSearch {
    n: usize,
    m: usize,
    visited: Vec<Vec<bool>>,
    board: Vec<Vec<char>>,
    word: Vec<char>,
}

impl WordSearch {
    pub fn new(board: Vec<Vec<char>>, word: String) -> Option<Self> {
        let n = board.len();
        if n == 0 {
            return None;
        }
        let m = board[0].len();
        if m == 0 {
            return None;
        }
        if word.is_empty() {
            return None;
        }
        let visited = vec![vec![false; m]; n];
        let word = word.chars().collect();

        Some(Self {
            n,
            m,
            visited,
            board,
            word,
        })
    }

    pub fn dfs(&mut self, i: usize, j: usize, k: usize) -> bool {
        if self.visited[i][j] || self.word[k] != self.board[i][j] {
            return false;
        }
        if k == self.word.len() - 1 {
            return true;
        } else {
            self.visited[i][j] = true;
            if (j > 0 && self.dfs(i, j - 1, k + 1))
                || (j < self.m - 1 && self.dfs(i, j + 1, k + 1))
                || (i > 0 && self.dfs(i - 1, j, k + 1))
                || (i < self.n - 1 && self.dfs(i + 1, j, k + 1))
            {
                return true;
            }
            self.visited[i][j] = false;
        }
        false
    }

    pub fn exist(&mut self) -> bool {
        for i in 0..self.n {
            for j in 0..self.m {
                if self.dfs(i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test_empty_board() {
    let board = vec_vec_char![];
    let word = "AC".to_string();
    assert_eq!(Solution::exist(board, word), false);
}

#[test]
fn test_empty_word() {
    let board = vec_vec_char![['A', 'C'], ['A', 'D']];
    let word = "".to_string();
    assert_eq!(Solution::exist(board, word), false);
}
