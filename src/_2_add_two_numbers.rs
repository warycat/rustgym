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

impl Solution {
    fn add_two_numbers(l1: Link, l2: Link) -> Link {
        let mut sum: Link = None;
        let mut p1: &Link = &l1;
        let mut p2: &Link = &l2;
        let mut p3: &mut Link = &mut sum;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val = carry;
            if let Some(n1) = p1.as_ref() {
                val += n1.val;
                p1 = &n1.next;
            }
            if let Some(n2) = p2.as_ref() {
                val += n2.val;
                p2 = &n2.next;
            }
            carry = val / 10;
            *p3 = ListNode::node(val % 10, None);
            p3 = &mut p3.as_mut().unwrap().next;
        }
        sum
    }
}

#[test]
fn test() {
    let l1 = ListNode::node(2, ListNode::node(4, ListNode::node(3, None)));
    let l2 = ListNode::node(5, ListNode::node(6, ListNode::node(4, None)));
    let l3 = ListNode::node(7, ListNode::node(0, ListNode::node(8, None)));
    assert_eq!(Solution::add_two_numbers(l1, l2), l3);
}
