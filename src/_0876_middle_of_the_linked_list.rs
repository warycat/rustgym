struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
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

struct List {
    head: Link,
}

impl List {
    fn new(head: Link) -> Self {
        List { head }
    }

    fn middle(&self) -> &Link {
        let mut slow = &self.head;
        let mut fast = &self.head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow
    }
}

impl Solution {
    fn middle_node(head: Link) -> Link {
        let list = List::new(head);
        let middle: &Link = list.middle();
        middle.clone()
    }
}

#[test]
fn test() {
    let head = ListNode::node(
        1,
        ListNode::node(
            2,
            ListNode::node(3, ListNode::node(4, ListNode::node(5, None))),
        ),
    );
    let middle = ListNode::node(3, ListNode::node(4, ListNode::node(5, None)));
    assert_eq!(Solution::middle_node(head), middle);
    let head = ListNode::node(
        1,
        ListNode::node(
            2,
            ListNode::node(
                3,
                ListNode::node(4, ListNode::node(5, ListNode::node(6, None))),
            ),
        ),
    );
    let middle = ListNode::node(4, ListNode::node(5, ListNode::node(6, None)));
    assert_eq!(Solution::middle_node(head), middle);
}
