struct Solution;

impl Solution {
    fn reorder_spaces(text: String) -> String {
        let space = text.chars().filter(|&c| c == ' ').count();
        let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
        if words.len() == 1 {
            let mut res = "".to_string();
            res.push_str(&words[0]);
            for _ in 0..space {
                res.push(' ');
            }
            res
        } else {
            let gap = words.len() - 1;
            let width = space / gap;
            let left = space - width * gap;
            let mut res = "".to_string();
            for i in 0..words.len() {
                res.push_str(&words[i]);
                for _ in 0..if i < gap { width } else { left } {
                    res.push(' ');
                }
            }
            res
        }
    }
}

#[test]
fn test() {
    let text = "  this   is  a sentence ".to_string();
    let res = "this   is   a   sentence".to_string();
    assert_eq!(Solution::reorder_spaces(text), res);
    let text = " practice   makes   perfect".to_string();
    let res = "practice   makes   perfect ".to_string();
    assert_eq!(Solution::reorder_spaces(text), res);
    let text = "hello   world".to_string();
    let res = "hello   world".to_string();
    assert_eq!(Solution::reorder_spaces(text), res);
    let text = "  walks  udp package   into  bar a".to_string();
    let res = "walks  udp  package  into  bar  a ".to_string();
    assert_eq!(Solution::reorder_spaces(text), res);
    let text = "a".to_string();
    let res = "a".to_string();
    assert_eq!(Solution::reorder_spaces(text), res);
}
