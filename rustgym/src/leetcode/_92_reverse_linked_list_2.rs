struct Solution;
use rustgym_util::*;

impl Solution {
    fn reverse_between(mut head: ListLink, m: i32, n: i32) -> ListLink {
        let mut v1: Vec<ListLink> = vec![];
        let mut v2: Vec<ListLink> = vec![];
        let mut v3: Vec<ListLink> = vec![];
        let mut i = 0;
        while let Some(mut node) = head {
            i += 1;
            head = node.next.take();
            if i < m {
                v1.push(Some(node));
            } else if i <= n {
                v2.push(Some(node));
            } else {
                v3.push(Some(node));
            }
        }
        let mut prev: ListLink = None;
        for link in v3.into_iter().rev() {
            let mut node = link.unwrap();
            node.next = prev;
            prev = Some(node);
        }
        for link in v2.into_iter() {
            let mut node = link.unwrap();
            node.next = prev;
            prev = Some(node);
        }
        for link in v1.into_iter().rev() {
            let mut node = link.unwrap();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3, 4, 5);
    let m = 2;
    let n = 4;
    let res = list!(1, 4, 3, 2, 5);
    assert_eq!(Solution::reverse_between(head, m, n), res);
}
