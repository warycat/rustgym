struct Solution;
use rustgym_util::*;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn num_components(head: ListLink, g: Vec<i32>) -> i32 {
        let mut p = head;
        let hs: HashSet<i32> = HashSet::from_iter(g);
        let mut open = false;
        let mut res = 0;
        while let Some(node) = p {
            if hs.contains(&node.val) {
                if !open {
                    open = true;
                }
            } else {
                if open {
                    open = false;
                    res += 1;
                }
            }
            p = node.next;
        }
        if open {
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(0, 1, 2, 3);
    let g = vec![0, 1, 3];
    let res = 2;
    assert_eq!(Solution::num_components(head, g), res);
    let head = list!(0, 1, 2, 3, 4);
    let g = vec![0, 3, 1, 4];
    let res = 2;
    assert_eq!(Solution::num_components(head, g), res);
}
