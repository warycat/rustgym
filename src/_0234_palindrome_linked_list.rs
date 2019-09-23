struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Link,
}

type Link = Option<Box<ListNode>>;

impl ListNode {
    fn node(val: i32, next: Link) -> Link {
        Some(Box::new(ListNode { val, next }))
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct List {
    head: Link,
}

impl List {
    fn list(head: Link) -> Self {
        List { head }
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    fn into_iter(self) -> IntoIter {
        IntoIter { list: self }
    }
}

struct IntoIter {
    list: List,
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl Solution {
    fn is_palindrome(head: Link) -> bool {
        let list = List::list(head);
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
    let head: Link = ListNode::node(1, ListNode::node(2, ListNode::node(3, None)));
    assert_eq!(Solution::is_palindrome(head), false);
}
