struct Solution;

#[derive(Eq, PartialEq, Clone, Debug)]
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
    fn remove_elements(mut head: List, val: i32) -> List {
        let mut p = &mut head;
        while p.is_some() {
            if p.as_ref().unwrap().val == val {
                *p = p.take().unwrap().next.take();
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[test]
fn test() {
    let input = ListNode::node(
        1,
        ListNode::node(
            2,
            ListNode::node(
                6,
                ListNode::node(
                    3,
                    ListNode::node(4, ListNode::node(5, ListNode::node(6, None))),
                ),
            ),
        ),
    );
    let output = ListNode::node(
        1,
        ListNode::node(
            2,
            ListNode::node(3, ListNode::node(4, ListNode::node(5, None))),
        ),
    );
    assert_eq!(Solution::remove_elements(input, 6), output);
}
