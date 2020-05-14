struct Solution;

impl Solution {
    fn reverse_words(s: &mut Vec<char>) {
        s.reverse();
        for word in s.split_mut(|&c| c == ' ') {
            word.reverse();
        }
    }
}

#[test]
fn test() {
    let mut s = vec![
        't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
    ];
    let res = vec![
        'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
    ];
    Solution::reverse_words(&mut s);
    assert_eq!(s, res);
}
