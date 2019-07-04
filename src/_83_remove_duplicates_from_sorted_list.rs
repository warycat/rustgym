struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Box<ListNode> {
        Box::new(ListNode { val, next: None })
    }
}

impl Solution {
    fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head.as_mut();
        while let Some(n) = p {
            while let Some(m) = n.next.as_mut(){
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
    let mut n1 = ListNode::new(1);
    let mut n2 = ListNode::new(1);
    let mut n3 = ListNode::new(2);
    let mut n4 = ListNode::new(3);
    let n5 = ListNode::new(3);

    n4.next = Some(n5);
    n3.next = Some(n4);
    n2.next = Some(n3);
    n1.next = Some(n2);

    let mut m1 = ListNode::new(1);
    let mut m2 = ListNode::new(2);
    let m3 = ListNode::new(3);

    m2.next = Some(m3);
    m1.next = Some(m2);

    assert_eq!(Solution::delete_duplicates(Some(n1)), Some(m1));
}
