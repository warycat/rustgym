struct Solution;
use rustgym_util::*;

impl Solution {
    fn split_list_to_parts(head: ListLink, k: i32) -> Vec<ListLink> {
        let mut nodes = vec![];
        let mut p = head;
        while let Some(mut node) = p {
            p = node.next.take();
            nodes.push(node);
        }
        let n = nodes.len();
        let k = k as usize;
        let left = n % k;
        let right = k - left;
        let m = n / k;
        let mut res = vec![None; k];
        for i in 0..right {
            let mut prev = None;
            for _ in 0..m {
                let mut node = nodes.pop().unwrap();
                node.next = prev;
                prev = Some(node);
            }
            res[k - 1 - i] = prev;
        }
        for i in 0..left {
            let mut prev = None;
            for _ in 0..=m {
                let mut node = nodes.pop().unwrap();
                node.next = prev;
                prev = Some(node);
            }
            res[k - right - 1 - i] = prev;
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3);
    let k = 5;
    let res = vec![list!(1), list!(2), list!(3), None, None];
    assert_eq!(Solution::split_list_to_parts(head, k), res);
}
