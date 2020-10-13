struct Solution;

impl Solution {
    fn expand(s: String) -> Vec<String> {
        let mut v: Vec<Vec<char>> = vec![];
        let mut stack: Vec<char> = vec![];
        let mut inbrace = false;
        for c in s.chars() {
            match c {
                '{' => {
                    inbrace = true;
                }
                '}' => {
                    stack.sort_unstable();
                    v.push(stack);
                    stack = vec![];
                    inbrace = false;
                }
                ',' => {}
                _ => {
                    if inbrace {
                        stack.push(c);
                    } else {
                        v.push(vec![c]);
                    }
                }
            }
        }
        let n = v.len();
        let mut cur = vec![];
        let mut res = vec![];
        Self::dfs(0, &mut cur, &mut res, &v, n);
        res
    }

    fn dfs(start: usize, cur: &mut Vec<char>, all: &mut Vec<String>, v: &[Vec<char>], n: usize) {
        if start == n {
            all.push(cur.iter().copied().collect());
        } else {
            for &c in &v[start] {
                cur.push(c);
                Self::dfs(start + 1, cur, all, v, n);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let s = "{a,b}c{d,e}f".to_string();
    let res: Vec<String> = vec_string!["acdf", "acef", "bcdf", "bcef"];
    assert_eq!(Solution::expand(s), res);
    let s = "abcd".to_string();
    let res: Vec<String> = vec_string!["abcd"];
    assert_eq!(Solution::expand(s), res);
}
