struct Solution;

impl Solution {
    fn print_vertically(s: String) -> Vec<String> {
        let v: Vec<&str> = s.split_whitespace().collect();
        let mut res = vec![];
        for (j, s) in v.iter().enumerate() {
            for (i, c) in s.char_indices() {
                if i == res.len() {
                    res.push("".to_string());
                }
                while res[i].len() < j {
                    res[i].push(' ');
                }
                res[i].push(c);
            }
        }
        res
    }
}
#[test]
fn test() {
    let s = "HOW ARE YOU".to_string();
    let res = vec_string!["HAY", "ORO", "WEU"];
    assert_eq!(Solution::print_vertically(s), res);
    let s = "TO BE OR NOT TO BE".to_string();
    let res = vec_string!["TBONTB", "OEROOE", "   T"];
    assert_eq!(Solution::print_vertically(s), res);
    let s = "CONTEST IS COMING".to_string();
    let res = vec_string!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"];
    assert_eq!(Solution::print_vertically(s), res);
}
