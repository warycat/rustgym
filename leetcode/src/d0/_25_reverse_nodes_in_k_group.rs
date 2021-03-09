struct Solution;
use rustgym_util::*;
use std::collections::VecDeque;

impl Solution {
    fn reverse_k_group(head: ListLink, k: i32) -> ListLink {
        let mut p = head;
        let mut count = 0;
        let mut queue: VecDeque<ListLink> = VecDeque::new();
        let k = k as usize;
        while let Some(mut node) = p {
            p = node.next.take();
            queue.push_back(Some(node));
            count += 1;
            if count == k {
                break;
            }
        }
        if queue.len() == k {
            let mut prev: ListLink = Self::reverse_k_group(p, k as i32);
            while let Some(link) = queue.pop_front() {
                if let Some(mut node) = link {
                    node.next = prev;
                    prev = Some(node);
                }
            }
            prev
        } else {
            let mut prev: ListLink = None;
            while let Some(link) = queue.pop_back() {
                if let Some(mut node) = link {
                    node.next = prev;
                    prev = Some(node);
                }
            }
            prev
        }
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3, 4, 5);
    let k = 2;
    let res = list!(2, 1, 4, 3, 5);
    assert_eq!(Solution::reverse_k_group(head, k), res);
    let head = list!(1, 2, 3, 4, 5);
    let k = 3;
    let res = list!(3, 2, 1, 4, 5);
    assert_eq!(Solution::reverse_k_group(head, k), res);
    let head = list!(1, 2);
    let k = 2;
    let res = list!(2, 1);
    assert_eq!(Solution::reverse_k_group(head, k), res);
}
