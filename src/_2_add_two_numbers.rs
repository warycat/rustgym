struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: List,
}

type List = Option<Box<ListNode>>;

impl ListNode {
    fn node(val: i32, next: List) -> List {
        Some(Box::new(ListNode { val, next }))
    }
}

impl Solution {
    fn add_two_numbers(l1: List, l2: List) -> List {
        let mut sum: List = None;
        let mut p1: &List = &l1;
        let mut p2: &List = &l2;
        let mut p3: &mut List = &mut sum;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val = carry;
            if p1.is_some() {
                let n1 = p1.as_ref().unwrap();
                val += n1.val;
                p1 = &n1.next;
            }
            if p2.is_some() {
                let n2 = p2.as_ref().unwrap();
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
