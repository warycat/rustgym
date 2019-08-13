struct Solution;

impl Solution {
    fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        if n == 0 {
            return;
        }
        let mut i: usize = 0;
        let mut j: usize = n - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[test]
fn test() {
    let mut input: Vec<char> = vec![];
    let output: Vec<char> = vec![];
    Solution::reverse_string(&mut input);
    assert_eq!(input, output);
    let mut input: Vec<char> = ["h", "e", "l", "l", "o"]
        .iter()
        .map(|s| s.chars().nth(0).unwrap())
        .collect();
    let output: Vec<char> = vec!["o", "l", "l", "e", "h"]
        .iter()
        .map(|s| s.chars().nth(0).unwrap())
        .collect();
    Solution::reverse_string(&mut input);
    assert_eq!(input, output);
}
