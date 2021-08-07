struct Solution;

impl Solution {
    fn permutation(s: &[char], n: usize, i: usize, t: &mut String, res: &mut Vec<String>) {
        if i == n {
            res.push(t.clone());
        } else {
            if s[i].is_alphabetic() {
                let lower: char = s[i].to_ascii_lowercase();
                let upper: char = s[i].to_ascii_uppercase();
                t.push(lower);
                Self::permutation(s, n, i + 1, t, res);
                t.pop();
                t.push(upper);
            } else {
                t.push(s[i]);
            }
            Self::permutation(s, n, i + 1, t, res);
            t.pop();
        }
    }
    fn letter_case_permutation(s: String) -> Vec<String> {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut res: Vec<String> = vec![];
        let mut t: String = "".to_string();
        Self::permutation(&s, n, 0, &mut t, &mut res);
        res
    }
}

#[test]
fn test() {
    let s: String = "a1b2".to_string();
    let res: Vec<String> = vec_string!["a1b2", "a1B2", "A1b2", "A1B2"];
    assert_eq!(Solution::letter_case_permutation(s), res);
}
