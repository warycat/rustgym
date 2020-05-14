struct Solution;
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end: Option<String>,
}

impl Trie {
    fn insert(&mut self, word: String) {
        let mut link = self;
        for c in word.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = Some(word);
    }
}

impl Solution {
    fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::default();
        for word in words {
            trie.insert(word);
        }
        let n = board.len();
        let m = board[0].len();
        let mut res: Vec<String> = vec![];
        for i in 0..n {
            for j in 0..m {
                Self::dfs(i, j, &mut board, &mut res, &mut trie, n, m);
            }
        }
        res.into_iter().collect()
    }

    fn dfs(
        i: usize,
        j: usize,
        board: &mut Vec<Vec<char>>,
        all: &mut Vec<String>,
        trie: &mut Trie,
        n: usize,
        m: usize,
    ) {
        let c = board[i][j];
        if let Some(trie) = trie.children.get_mut(&c) {
            board[i][j] = ' ';
            if trie.end.is_some() {
                all.push(trie.end.take().unwrap());
            }
            if i + 1 < n {
                Self::dfs(i + 1, j, board, all, trie, n, m);
            }
            if j + 1 < m {
                Self::dfs(i, j + 1, board, all, trie, n, m);
            }
            if i > 0 {
                Self::dfs(i - 1, j, board, all, trie, n, m);
            }
            if j > 0 {
                Self::dfs(i, j - 1, board, all, trie, n, m);
            }
            board[i][j] = c;
        }
    }
}

#[test]
fn test() {
    let board = vec_vec_char![
        ['o', 'a', 'a', 'n'],
        ['e', 't', 'a', 'e'],
        ['i', 'h', 'k', 'r'],
        ['i', 'f', 'l', 'v']
    ];
    let words = vec_string!["oath", "pea", "eat", "rain"];
    let mut res = vec_string!["eat", "oath"];
    let mut ans = Solution::find_words(board, words);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
