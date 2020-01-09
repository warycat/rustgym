struct Solution;
use crate::util::*;

impl Solution {
    fn delete_duplicates(mut head: ListLink) -> ListLink {
        let mut p = head.as_mut();
        while let Some(n) = p {
            while let Some(m) = n.next.as_mut() {
                if m.val != n.val {
                    break;
                }
                n.next = m.next.take();
            }
            p = n.next.as_mut();
        }
        head
    }
}

#[test]
fn test() {
    let p = ListNode::list(vec![1, 1, 2, 3, 3]);
    let q = ListNode::node(1, ListNode::node(2, ListNode::node(3, None)));
    assert_eq!(Solution::delete_duplicates(p), q);
}
