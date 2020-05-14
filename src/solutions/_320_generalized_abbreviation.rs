struct Solution;

impl Solution {
    fn generate_abbreviations(word: String) -> Vec<String> {
        let n = word.len();
        let word: Vec<char> = word.chars().collect();
        let mut cur: String = "".to_string();
        let mut res = vec![];
        Self::dfs(0, 0, &mut cur, &mut res, &word, n);
        res
    }

    fn dfs(
        start: usize,
        count: usize,
        cur: &mut String,
        all: &mut Vec<String>,
        word: &[char],
        n: usize,
    ) {
        let len = cur.len();
        if start == n {
            if count > 0 {
                *cur += &count.to_string();
            }
            all.push((*cur).to_string());
        } else {
            Self::dfs(start + 1, count + 1, cur, all, word, n);
            if count > 0 {
                *cur += &count.to_string();
            }
            cur.push(word[start]);
            Self::dfs(start + 1, 0, cur, all, word, n);
        }
        cur.truncate(len);
    }
}

#[test]
fn test() {
    let word = "word".to_string();
    let mut res = vec_string![
        "word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1", "w1r1", "1o2",
        "2r1", "3d", "w3", "4"
    ];
    let mut ans = Solution::generate_abbreviations(word);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
