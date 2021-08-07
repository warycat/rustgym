struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut queue: VecDeque<String> = VecDeque::new();
        let max_width = max_width as usize;
        let mut width = 0;
        let mut res = vec![];
        for word in words {
            if width + queue.len() + word.len() > max_width as usize {
                res.push(Self::make_a_line(&mut queue, max_width, width));
                width = word.len();
            } else {
                width += word.len();
            }
            queue.push_back(word);
        }
        res.push(Self::make_the_last_line(&mut queue, max_width, width));
        res
    }

    fn make_a_line(queue: &mut VecDeque<String>, max_width: usize, width: usize) -> String {
        let mut line = "".to_string();
        let mut space = max_width - width;
        while let Some(s) = queue.pop_front() {
            line += &s;
            let d = if !queue.is_empty() {
                space / queue.len() + if space % queue.len() != 0 { 1 } else { 0 }
            } else {
                space
            };
            space -= d;
            for _ in 0..d {
                line.push(' ');
            }
        }
        line
    }

    fn make_the_last_line(queue: &mut VecDeque<String>, max_width: usize, width: usize) -> String {
        let mut line = "".to_string();
        let mut space = max_width - width;
        while let Some(s) = queue.pop_front() {
            line += &s;
            if queue.is_empty() {
                for _ in 0..space {
                    line.push(' ');
                }
            } else {
                line.push(' ');
                space -= 1;
            }
        }
        line
    }
}

#[test]
fn test() {
    let words = vec_string![
        "This",
        "is",
        "an",
        "example",
        "of",
        "text",
        "justification."
    ];
    let max_width = 16;
    let res = vec_string!["This    is    an", "example  of text", "justification.  "];
    assert_eq!(Solution::full_justify(words, max_width), res);
    let words = vec_string!["What", "must", "be", "acknowledgment", "shall", "be"];
    let max_width = 16;
    let res = vec_string!["What   must   be", "acknowledgment  ", "shall be        "];
    assert_eq!(Solution::full_justify(words, max_width), res);
    let words = vec_string!["What", "must", "be", "acknowledgment", "shall", "be"];
    let max_width = 16;
    let res = vec_string!["What   must   be", "acknowledgment  ", "shall be        "];
    assert_eq!(Solution::full_justify(words, max_width), res);
    let words = vec_string![
        "Science",
        "is",
        "what",
        "we",
        "understand",
        "well",
        "enough",
        "to",
        "explain",
        "to",
        "a",
        "computer.",
        "Art",
        "is",
        "everything",
        "else",
        "we",
        "do"
    ];
    let max_width = 20;
    let res = vec_string![
        "Science  is  what we",
        "understand      well",
        "enough to explain to",
        "a  computer.  Art is",
        "everything  else  we",
        "do                  "
    ];
    assert_eq!(Solution::full_justify(words, max_width), res);
}
