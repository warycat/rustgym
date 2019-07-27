struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: List,
}

impl ListNode {
    fn node(val: i32, next: List) -> List {
        Some(Box::new(ListNode { val, next }))
    }
}

type List = Option<Box<ListNode>>;

impl Solution {
    fn reverse_list(head: List) -> List {
        let mut p = head;
        let mut prev = None;
        while let Some(mut node) = p {
            p = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[test]
fn none() {
    assert_eq!(Solution::reverse_list(None), None);
}

#[test]
fn one_node() {
    let a = ListNode::node(1, None);
    let b = ListNode::node(1, None);
    assert_eq!(Solution::reverse_list(a), b);
}

#[test]
fn two_nodes() {
    let a = ListNode::node(1, ListNode::node(2, None));
    let b = ListNode::node(2, ListNode::node(1, None));
    assert_eq!(Solution::reverse_list(a), b);
}

#[test]
fn three_nodes() {
    let a = ListNode::node(1, ListNode::node(2, ListNode::node(3, None)));
    let b = ListNode::node(3, ListNode::node(2, ListNode::node(1, None)));
    assert_eq!(Solution::reverse_list(a), b);
}
