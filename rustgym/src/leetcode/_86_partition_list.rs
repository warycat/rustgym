struct Solution;
use rustgym_util::*;

impl Solution {
    fn partition(mut head: ListLink, x: i32) -> ListLink {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        while let Some(node) = head {
            let val = node.val;
            if val < x {
                left.push(val);
            } else {
                right.push(val);
            }
            head = node.next;
        }
        let mut res = None;
        while let Some(val) = right.pop() {
            res = ListLink::link(val, res);
        }
        while let Some(val) = left.pop() {
            res = ListLink::link(val, res);
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(1, 4, 3, 2, 5, 2);
    let x = 3;
    let res = list!(1, 2, 2, 4, 3, 5);
    assert_eq!(Solution::partition(head, x), res);
}
