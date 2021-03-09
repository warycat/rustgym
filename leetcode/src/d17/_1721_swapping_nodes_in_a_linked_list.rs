struct Solution;

use rustgym_util::*;

impl Solution {
    fn swap_nodes(head: ListLink, k: i32) -> ListLink {
        let mut p = head;
        let mut nodes: Vec<Box<ListNode>> = vec![];
        let k = k as usize;
        while let Some(mut node) = p {
            p = node.next.take();
            nodes.push(node);
        }
        let n = nodes.len();
        nodes.swap(k - 1, n - k);
        let mut prev = None;
        while let Some(mut node) = nodes.pop() {
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3, 4, 5);
    let k = 2;
    let res = list!(1, 4, 3, 2, 5);
    assert_eq!(Solution::swap_nodes(head, k), res);
    let head = list!(7, 9, 6, 6, 7, 8, 3, 0, 9, 5);
    let k = 5;
    let res = list!(7, 9, 6, 6, 8, 7, 3, 0, 9, 5);
    assert_eq!(Solution::swap_nodes(head, k), res);
    let head = list!(1);
    let k = 1;
    let res = list!(1);
    assert_eq!(Solution::swap_nodes(head, k), res);
    let head = list!(1, 2);
    let k = 1;
    let res = list!(2, 1);
    assert_eq!(Solution::swap_nodes(head, k), res);
    let head = list!(1, 2, 3);
    let k = 2;
    let res = list!(1, 2, 3);
    assert_eq!(Solution::swap_nodes(head, k), res);
}
