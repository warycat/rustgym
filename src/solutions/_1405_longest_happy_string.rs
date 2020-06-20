struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut queue: BinaryHeap<(i32, char)> = BinaryHeap::new();
        if a > 0 {
            queue.push((a, 'a'));
        }
        if b > 0 {
            queue.push((b, 'b'));
        }
        if c > 0 {
            queue.push((c, 'c'));
        }
        let mut res = "".to_string();
        while !queue.is_empty() {
            match queue.len() {
                1 => {
                    let (size, c) = queue.pop().unwrap();
                    for _ in 0..2.min(size) {
                        res.push(c);
                    }
                }
                _ => {
                    let (mut size_a, char_a) = queue.pop().unwrap();
                    let (mut size_b, char_b) = queue.pop().unwrap();
                    for _ in 0..2.min(size_a) {
                        res.push(char_a);
                        size_a -= 1;
                    }
                    if size_b > size_a {
                        for _ in 0..2.min(size_b) {
                            res.push(char_b);
                            size_b -= 1;
                        }
                    } else {
                        res.push(char_b);
                        size_b -= 1;
                    }
                    if size_a > 0 {
                        queue.push((size_a, char_a));
                    }
                    if size_b > 0 {
                        queue.push((size_b, char_b));
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = 7;
    let b = 1;
    let c = 0;
    let res = "aabaa".to_string();
    assert_eq!(Solution::longest_diverse_string(a, b, c), res);
}
