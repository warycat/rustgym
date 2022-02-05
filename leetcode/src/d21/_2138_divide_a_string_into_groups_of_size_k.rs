struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let mut answer = s.chars().collect::<Vec<char>>();
        answer.resize(s.len() + (k - s.len() % k) % k, fill);
        answer
            .chunks(k as usize)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
    }
}

#[test]
fn test() {
    let s = "abcdefghi".to_string();
    let k = 3;
    let fill = 'x';
    let res = vec_string!["abc", "def", "ghi"];
    assert_eq!(Solution::divide_string(s, k, fill), res);

    let s = "abcdefghij".to_string();
    let k = 3;
    let fill = 'x';
    let res = vec_string!["abc", "def", "ghi", "jxx"];
    assert_eq!(Solution::divide_string(s, k, fill), res);
}
