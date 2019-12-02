struct Solution;

#[allow(clippy::wrong_self_convention)]
impl Solution {
    fn to_hexspeak(num: String) -> String {
        let x: i64 = num.parse::<i64>().unwrap();
        let s = format!("{:X}", x);
        let mut res: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                '0' => res.push('O'),
                '1' => res.push('I'),
                c @ 'A'..='F' => res.push(c),
                _ => return "ERROR".to_string(),
            }
        }
        res.iter().collect()
    }
}

#[test]
fn test() {
    let num = "257".to_string();
    let res = "IOI".to_string();
    assert_eq!(Solution::to_hexspeak(num), res);
    let num = "3".to_string();
    let res = "ERROR".to_string();
    assert_eq!(Solution::to_hexspeak(num), res);
    let num = "619879596177".to_string();
    let res = "ERROR".to_string();
    assert_eq!(Solution::to_hexspeak(num), res);
}
