struct Solution;

impl Solution {
    fn partition(s: String) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut indexes: Vec<(usize, usize)> = vec![];
        Self::dfs(0, &mut indexes, &mut res, &s, n);
        res
    }
    fn dfs(
        start: usize,
        indexes: &mut Vec<(usize, usize)>,
        strings: &mut Vec<Vec<String>>,
        s: &[char],
        n: usize,
    ) {
        if start == n {
            let mut partition: Vec<String> = vec![];
            for &(l, r) in indexes.iter() {
                partition.push(s[l..r].iter().collect());
            }
            strings.push(partition);
        }
        for end in start + 1..=n {
            if Self::is_palindrome(&s[start..end]) {
                indexes.push((start, end));
                Self::dfs(end, indexes, strings, s, n);
                indexes.pop();
            }
        }
    }
    fn is_palindrome(s: &[char]) -> bool {
        let n = s.len();
        for i in 0..n {
            let j = n - 1 - i;
            if s[i] != s[j] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let s = "aab".to_string();
    let mut res = vec_vec_string![["aa", "b"], ["a", "a", "b"]];
    let mut ans = Solution::partition(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let s = "efe".to_string();
    let mut res = vec_vec_string![["e", "f", "e"], ["efe"]];
    let mut ans = Solution::partition(s);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
