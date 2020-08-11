struct Solution;
use rustgym_util::*;

struct List {
    head: ListLink,
}

impl List {
    fn new(head: ListLink) -> Self {
        List { head }
    }

    fn middle(&self) -> &ListLink {
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
    fn middle_node(head: ListLink) -> ListLink {
        let list = List::new(head);
        let middle: &ListLink = list.middle();
        middle.clone()
    }
}

#[test]
fn test() {
    let head = list![1, 2, 3, 4, 5];
    let middle = list![3, 4, 5];
    assert_eq!(Solution::middle_node(head), middle);
    let head = list![1, 2, 3, 4, 5, 6];
    let middle = list![4, 5, 6];
    assert_eq!(Solution::middle_node(head), middle);
}
