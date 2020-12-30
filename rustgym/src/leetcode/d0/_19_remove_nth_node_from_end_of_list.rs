struct Solution;
use rustgym_util::*;

impl Solution {
    fn remove_nth_from_end(mut head: ListLink, n: i32) -> ListLink {
        let mut v: Vec<ListLink> = vec![];
        while let Some(mut node) = head {
            head = node.next.take();
            v.push(Some(node));
        }
        let mut res = None;
        for (i, link) in v.into_iter().rev().enumerate() {
            if i != (n - 1) as usize {
                let mut node = link.unwrap();
                node.next = res;
                res = Some(node);
            }
        }
        res
    }
}

#[test]
fn test() {
    let head = list![1, 2, 3, 4, 5];
    let res = list![1, 2, 3, 5];
    let n = 2;
    assert_eq!(Solution::remove_nth_from_end(head, n), res);
}
