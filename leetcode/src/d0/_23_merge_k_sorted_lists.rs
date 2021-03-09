struct Solution;
use rustgym_util::*;
use std::collections::VecDeque;

impl Solution {
    fn merge_k_lists(lists: Vec<ListLink>) -> ListLink {
        if lists.is_empty() {
            return None;
        }
        let mut queue: VecDeque<ListLink> = lists.into_iter().collect();
        while queue.len() > 1 {
            let merged_list = Self::merge(queue.pop_front().unwrap(), queue.pop_front().unwrap());
            queue.push_back(merged_list);
        }
        queue.pop_back().unwrap()
    }

    fn merge(a: ListLink, b: ListLink) -> ListLink {
        if a.is_none() && b.is_none() {
            return None;
        }
        if a.is_none() {
            return b;
        }
        if b.is_none() {
            return a;
        }
        let mut a = a.unwrap();
        let mut b = b.unwrap();
        if a.val < b.val {
            a.next = Self::merge(a.next.take(), Some(b));
            Some(a)
        } else {
            b.next = Self::merge(Some(a), b.next.take());
            Some(b)
        }
    }
}

#[test]
fn test() {
    let lists = vec![list!(1, 4, 5), list!(1, 3, 4), list!(2, 6)];
    let res = list!(1, 1, 2, 3, 4, 4, 5, 6);
    assert_eq!(Solution::merge_k_lists(lists), res);
}
