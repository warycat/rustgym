struct Solution;

use rustgym_util::*;

impl Solution {
    fn merge_in_between(list1: ListLink, a: i32, b: i32, list2: ListLink) -> ListLink {
        let mut l1 = vec![];
        let mut l2 = vec![];
        let mut p1 = list1;
        let start = a as usize;
        let end = (b + 1) as usize;
        while let Some(mut node) = p1 {
            p1 = node.next.take();
            l1.push(node);
        }
        let mut p2 = list2;
        while let Some(mut node) = p2 {
            p2 = node.next.take();
            l2.push(node);
        }
        let mut prev = None;
        for _ in (end..l1.len()).rev() {
            let mut node = l1.pop().unwrap();
            node.next = prev;
            prev = Some(node);
        }
        for _ in (start..end).rev() {
            l1.pop();
        }
        for _ in (0..l2.len()).rev() {
            let mut node = l2.pop().unwrap();
            node.next = prev;
            prev = Some(node);
        }
        for _ in (0..start).rev() {
            let mut node = l1.pop().unwrap();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[test]
fn test() {
    let list1 = list![0, 1, 2, 3, 4, 5];
    let a = 3;
    let b = 4;
    let list2 = list![1000000, 1000001, 1000002];
    let res = list![0, 1, 2, 1000000, 1000001, 1000002, 5];
    assert_eq!(Solution::merge_in_between(list1, a, b, list2), res);
    let list1 = list![0, 1, 2, 3, 4, 5, 6];
    let a = 2;
    let b = 5;
    let list2 = list![1000000, 1000001, 1000002, 1000003, 1000004];
    let res = list![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6];
    assert_eq!(Solution::merge_in_between(list1, a, b, list2), res);
}
