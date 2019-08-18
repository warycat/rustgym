struct Solution;

impl Solution {
    fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(&top) = stack.last() {
                if top == c {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            } else {
                stack.push(c)
            }
        }
        stack.iter().collect()
    }
}

#[test]
fn test() {
    let s = "abbaca".to_string();
    let t = "ca".to_string();
    assert_eq!(Solution::remove_duplicates(s), t);
}
