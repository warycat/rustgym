struct Solution;
use crate::util::*;

impl Solution {
    fn swap_pairs(head: ListLink) -> ListLink {
        if let Some(mut first) = head {
            if let Some(mut second) = first.next.take() {
                let rest = second.next.take();
                first.next = Self::swap_pairs(rest);
                second.next = Some(first);
                Some(second)
            } else {
                Some(first)
            }
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let head = ListNode::list(vec![1, 2, 3, 4]);
    let res = ListNode::list(vec![2, 1, 4, 3]);
    assert_eq!(Solution::swap_pairs(head), res);
}
