struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn node(val: i32, next: List) -> List {
        Some(Box::new(ListNode { next, val }))
    }
}

type List = Option<Box<ListNode>>;

impl Solution {
    fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut p: &List = &head;
        let mut res = 0;
        while let Some(n) = p {
            res *= 2;
            res += n.val;
            p = &n.next;
        }
        res
    }
}

#[test]
fn test() {
    let head = ListNode::node(1, ListNode::node(0, ListNode::node(1, None)));
    assert_eq!(Solution::get_decimal_value(head), 5);
}
