struct Solution;
use rustgym_util::*;

impl Solution {
    fn sort_list(head: ListLink) -> ListLink {
        let mut cur = head;
        let mut v: Vec<i32> = vec![];
        while let Some(node) = cur {
            v.push(node.val);
            cur = node.next;
        }
        v.sort_unstable();
        let mut prev = None;
        while let Some(last) = v.pop() {
            prev = ListLink::link(last, prev);
        }
        prev
    }
}

#[test]
fn test() {
    let head = list![4, 2, 1, 3];
    let res = list![1, 2, 3, 4];
    assert_eq!(Solution::sort_list(head), res);
}
