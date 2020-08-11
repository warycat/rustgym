struct Solution;
use rustgym_util::*;

impl Solution {
    fn rotate_right(mut head: ListLink, k: i32) -> ListLink {
        let mut p = head.as_ref();
        let mut n = 0;
        while let Some(node) = p {
            p = node.next.as_ref();
            n += 1;
        }
        if n < 2 {
            return head;
        }
        let k = k as usize % n;
        if k == 0 {
            return head;
        }
        let mut i = 0;
        let mut p = head.as_mut();
        let mut new_head: ListLink = None;
        while let Some(node) = p {
            if i + k == n - 1 {
                new_head = node.next.take();
                break;
            } else {
                p = node.next.as_mut();
            }
            i += 1;
        }
        let mut p = new_head.as_mut();
        while let Some(node) = p {
            if node.next.is_none() {
                node.next = head;
                break;
            }
            p = node.next.as_mut();
        }
        new_head
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3, 4, 5);
    let k = 2;
    let res = list!(4, 5, 1, 2, 3);
    assert_eq!(Solution::rotate_right(head, k), res);
    let head = list!(0, 1, 2);
    let k = 4;
    let res = list!(2, 0, 1);
    assert_eq!(Solution::rotate_right(head, k), res);
    let head = list!(1);
    let k = 4;
    let res = list!(1);
    assert_eq!(Solution::rotate_right(head, k), res);
}
