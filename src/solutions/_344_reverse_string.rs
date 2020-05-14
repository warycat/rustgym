struct Solution;

impl Solution {
    fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[test]
fn test() {
    let mut input: Vec<char> = vec![];
    let output: Vec<char> = vec![];
    Solution::reverse_string(&mut input);
    assert_eq!(input, output);
    let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    let output: Vec<char> = vec!['o', 'l', 'l', 'e', 'h'];
    Solution::reverse_string(&mut input);
    assert_eq!(input, output);
}
