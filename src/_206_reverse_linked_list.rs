struct Solution;
use crate::util::*;

impl Solution {
    fn reverse_list(head: ListLink) -> ListLink {
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
fn test() {
    let head = ListNode::list(vec![]);
    let res = ListNode::list(vec![]);
    assert_eq!(Solution::reverse_list(head), res);
    let head = ListNode::list(vec![1]);
    let res = ListNode::list(vec![1]);
    assert_eq!(Solution::reverse_list(head), res);
    let head = ListNode::list(vec![1, 2]);
    let res = ListNode::list(vec![2, 1]);
    assert_eq!(Solution::reverse_list(head), res);
    let head = ListNode::list(vec![1, 2, 3]);
    let res = ListNode::list(vec![3, 2, 1]);
    assert_eq!(Solution::reverse_list(head), res);
}
