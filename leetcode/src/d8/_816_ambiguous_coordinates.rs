struct Solution;

trait Nums {
    fn nums(self) -> Vec<String>;
}

impl Nums for &[char] {
    fn nums(self) -> Vec<String> {
        let n = self.len();
        let mut res = vec![];
        for i in 1..=n {
            let left: String = self[..i].iter().collect();
            let right: String = self[i..].iter().collect();
            if left.starts_with('0') && left != "0" {
                continue;
            }
            if right.ends_with('0') {
                continue;
            }
            res.push(format!(
                "{}{}{}",
                left,
                if i == n { "" } else { "." },
                right
            ));
        }
        res
    }
}

impl Solution {
    fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut res = vec![];
        for i in 2..n - 1 {
            let left = &s[1..i];
            let right = &s[i..n - 1];
            for l in left.nums() {
                for r in right.nums() {
                    res.push(format!("({}, {})", l, r));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "(123)".to_string();
    let mut res = vec_string!["(1, 23)", "(12, 3)", "(1.2, 3)", "(1, 2.3)"];
    let mut ans = Solution::ambiguous_coordinates(s);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "(00011)".to_string();
    let mut res = vec_string!["(0.001, 1)", "(0, 0.011)"];
    let mut ans = Solution::ambiguous_coordinates(s);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "(0123)".to_string();
    let mut res = vec_string![
        "(0, 123)",
        "(0, 12.3)",
        "(0, 1.23)",
        "(0.1, 23)",
        "(0.1, 2.3)",
        "(0.12, 3)"
    ];
    let mut ans = Solution::ambiguous_coordinates(s);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "(100)".to_string();
    let mut res = vec_string!["(10, 0)"];
    let mut ans = Solution::ambiguous_coordinates(s);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
