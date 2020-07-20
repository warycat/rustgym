struct Solution;

impl Solution {
    fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut count: Vec<i32> = vec![0; 26];
        for c in letters {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let n = words.len();
        let scores: Vec<i32> = words
            .iter()
            .map(|s| s.bytes().map(|b| score[(b - b'a') as usize]).sum())
            .collect();
        let mut res = 0;
        Self::dfs(0, 0, &mut count, &mut res, &words, &scores, n);
        res
    }

    fn dfs(
        start: usize,
        sum: i32,
        count: &mut Vec<i32>,
        max: &mut i32,
        words: &[String],
        scores: &[i32],
        n: usize,
    ) {
        if start == n {
            *max = (*max).max(sum);
        } else {
            Self::dfs(start + 1, sum, count, max, words, scores, n);
            Self::update(count, &words[start], -1);
            if Self::check(count, &words[start]) {
                Self::dfs(start + 1, sum + scores[start], count, max, words, scores, n);
            }
            Self::update(count, &words[start], 1);
        }
    }

    fn update(count: &mut Vec<i32>, s: &str, val: i32) {
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += val;
        }
    }

    fn check(count: &mut Vec<i32>, s: &str) -> bool {
        for c in s.chars() {
            if count[(c as u8 - b'a') as usize] < 0 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let words = vec_string!["dog", "cat", "dad", "good"];
    let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
    let score = vec![
        1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let res = 23;
    assert_eq!(Solution::max_score_words(words, letters, score), res);
    let words = vec_string!["xxxz", "ax", "bx", "cx"];
    let letters = vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
    let score = vec![
        4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
    ];
    let res = 27;
    assert_eq!(Solution::max_score_words(words, letters, score), res);
    let words = vec_string!["leetcode"];
    let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
    let score = vec![
        0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ];
    let res = 0;
    assert_eq!(Solution::max_score_words(words, letters, score), res);
}
