struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Link,
}

impl ListNode {
    fn node(val: i32, next: Link) -> Link {
        Some(Box::new(ListNode { next, val }))
    }
}

type Link = Option<Box<ListNode>>;

impl Solution {
    fn merge_two_lists(l1: Link, l2: Link) -> Link {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }
        let mut p1 = l1.unwrap();
        let mut p2 = l2.unwrap();
        if p1.val < p2.val {
            p1.next = Self::merge_two_lists(p1.next, Some(p2));
            Some(p1)
        } else {
            p2.next = Self::merge_two_lists(Some(p1), p2.next);
            Some(p2)
        }
    }
}

#[test]
fn test() {
    let a = ListNode::node(1, ListNode::node(2, ListNode::node(4, None)));
    let b = ListNode::node(1, ListNode::node(3, ListNode::node(4, None)));
    let c = ListNode::node(
        1,
        ListNode::node(
            1,
            ListNode::node(
                2,
                ListNode::node(3, ListNode::node(4, ListNode::node(4, None))),
            ),
        ),
    );
    assert_eq!(Solution::merge_two_lists(a, b), c);
}
