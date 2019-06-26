struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Box<ListNode> {
        Box::new(ListNode { next: None, val })
    }
}

impl Solution {
    fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
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
            return Some(p1);
        } else {
            p2.next = Self::merge_two_lists(Some(p1), p2.next);
            return Some(p2);
        }
    }
}

#[test]
fn test() {
    let mut a1 = ListNode::new(1);
    let mut a2 = ListNode::new(2);
    let a3 = ListNode::new(4);
    let mut b1 = ListNode::new(1);
    let mut b2 = ListNode::new(3);
    let b3 = ListNode::new(4);

    a2.next = Some(a3);
    a1.next = Some(a2);
    b2.next = Some(b3);
    b1.next = Some(b2);

    let mut c1 = ListNode::new(1);
    let mut c2 = ListNode::new(1);
    let mut c3 = ListNode::new(2);
    let mut c4 = ListNode::new(3);
    let mut c5 = ListNode::new(4);
    let c6 = ListNode::new(4);

    c5.next = Some(c6);
    c4.next = Some(c5);
    c3.next = Some(c4);
    c2.next = Some(c3);
    c1.next = Some(c2);

    assert_eq!(Solution::merge_two_lists(Some(a1), Some(b1)), Some(c1))
}
