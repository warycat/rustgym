struct Solution{}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val:i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val:i32) -> Box<Self>{
        let node = ListNode{
            val,
            next:None,
        };
        Box::new(node)
    }
}

impl Solution{
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;
        let mut prev = None;
        while let Some(mut node) = p {
                p = node.next;
                node.next = prev;
                prev = Some(node);
        }
        prev
    }
}

#[test]
fn none() {
    assert_eq!(Solution::reverse_list(None), None);
}

#[test]
fn one_node() {
    let l1 = ListNode::new(3);
    let l2 = ListNode::new(3);
    assert_eq!(Solution::reverse_list(Some(l1)), Some(l2));
}

#[test]
fn two_nodes() {
    let mut l1 = ListNode::new(1);
    let l2 = ListNode::new(2);
    l1.next = Some(l2);

    let mut l3 = ListNode::new(2);
    let l4 = ListNode::new(1);
    l3.next = Some(l4);
    assert_eq!(Solution::reverse_list(Some(l1)), Some(l3));
}

#[test]
fn three_nodes() {
    let mut l1 = ListNode::new(1);
    let mut l2 = ListNode::new(2);
    let l3 = ListNode::new(3);
    l2.next = Some(l3);
    l1.next = Some(l2);

    let mut l4 = ListNode::new(3);
    let mut l5 = ListNode::new(2);
    let l6 = ListNode::new(1);
    l5.next = Some(l6);
    l4.next = Some(l5);
    
    assert_eq!(Solution::reverse_list(Some(l1)), Some(l4));
}