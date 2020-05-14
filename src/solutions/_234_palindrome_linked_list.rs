pub struct Solution;
use util::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List {
    pub head: ListLink,
}

impl List {
    pub fn new(head: ListLink) -> Self {
        List { head }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter { list: self }
    }
}

pub struct IntoIter {
    list: List,
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl Solution {
    pub fn is_palindrome(head: ListLink) -> bool {
        let list = List::new(head);
        let vec: Vec<i32> = list.into_iter().collect();
        for (i, &v) in vec.iter().rev().enumerate() {
            if v != vec[i] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3);
    assert_eq!(Solution::is_palindrome(head), false);
}
