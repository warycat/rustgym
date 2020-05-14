struct Solution;

impl Solution {
    fn generate_possible_next_moves(s: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let n = s.len();
        for i in 1..n {
            if &s[i - 1..=i] == "++" {
                res.push(format!("{}{}{}", &s[0..i - 1], "--", &s[i + 1..]));
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "++++".to_string();
    let res: Vec<String> = vec_string!["--++", "+--+", "++--"];
    assert_eq!(Solution::generate_possible_next_moves(s), res);
}
