struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let mut answer = s
            .chars()
            .collect::<Vec<char>>()
            .chunks(k as usize)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        if s.len() % k != 0 {
            answer[s.len() / k]
                .push_str(&vec![fill; k - s.len() % k].into_iter().collect::<String>());
        }
        return answer;
    }
}

#[test]
fn test_1() {
    let s = "abcdefghi".to_string();
    let k = 3;
    let fill = 'x';
    let res = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
    assert_eq!(Solution::divide_string(s, k, fill), res);
}

#[test]
fn test_2() {
    let s = "abcdefghij".to_string();
    let k = 3;
    let fill = 'x';
    let res = vec![
        "abc".to_string(),
        "def".to_string(),
        "ghi".to_string(),
        "jxx".to_string(),
    ];
    assert_eq!(Solution::divide_string(s, k, fill), res);
}
