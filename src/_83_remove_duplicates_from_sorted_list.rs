struct Solution {}

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
    fn delete_duplicates(mut head: List) -> List {
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
    let p = ListNode::node(
        1,
        ListNode::node(
            1,
            ListNode::node(2, ListNode::node(3, ListNode::node(3, None))),
        ),
    );
    let q = ListNode::node(1, ListNode::node(2, ListNode::node(3, None)));
    assert_eq!(Solution::delete_duplicates(p), q);
}
