struct Solution;
use rustgym_util::*;
use std::collections::HashMap;

impl Solution {
    fn delete_duplicates_unsorted(head: ListLink) -> ListLink {
        let mut nodes: Vec<Box<ListNode>> = vec![];
        let mut p = head;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        while let Some(mut node) = p {
            p = node.next.take();
            let val = node.val;
            *hm.entry(val).or_default() += 1;
            nodes.push(node);
        }
        let mut res = None;
        while let Some(mut node) = nodes.pop() {
            if hm[&node.val] == 1 {
                node.next = res;
                res = Some(node);
            }
        }
        res
    }
}

#[test]
fn test() {
    let head = list![1, 2, 3, 2];
    let res = list![1, 3];
    assert_eq!(Solution::delete_duplicates_unsorted(head), res);
    let head = list![2, 1, 1, 2];
    let res = list![];
    assert_eq!(Solution::delete_duplicates_unsorted(head), res);
    let head = list![3, 2, 2, 1, 3, 2, 4];
    let res = list![1, 4];
    assert_eq!(Solution::delete_duplicates_unsorted(head), res);
}
