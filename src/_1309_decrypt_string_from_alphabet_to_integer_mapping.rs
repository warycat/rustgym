struct Solution;

impl Solution {
    fn freq_alphabets(s: String) -> String {
        let mut v: Vec<char> = vec![];
        let mut u: Vec<char> = s.chars().collect();
        while let Some(c) = u.pop() {
            let d = match c {
                '#' => (u.pop().unwrap() as u8 - b'0') + 10 * (u.pop().unwrap() as u8 - b'0'),
                _ => c as u8 - b'0',
            } - 1;
            v.insert(0, (b'a' + d) as char);
        }
        v.iter().collect()
    }
}

#[test]
fn test() {
    let s = "10#11#12".to_string();
    let res = "jkab".to_string();
    assert_eq!(Solution::freq_alphabets(s), res);
    let s = "1326#".to_string();
    let res = "acz".to_string();
    assert_eq!(Solution::freq_alphabets(s), res);
    let s = "25#".to_string();
    let res = "y".to_string();
    assert_eq!(Solution::freq_alphabets(s), res);
    let s = "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string();
    let res = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::freq_alphabets(s), res);
}
