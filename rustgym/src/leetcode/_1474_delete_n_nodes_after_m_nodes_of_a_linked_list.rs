struct Solution;
use rustgym_util::*;

impl Solution {
    fn delete_nodes(mut head: ListLink, m: i32, n: i32) -> ListLink {
        let mut i = 0;
        let mut j = 0;
        let mut v: Vec<i32> = vec![];
        while let Some(node) = head {
            if i == m && j == n {
                i = 0;
                j = 0;
            }
            if i < m {
                v.push(node.val);
                i += 1;
            } else {
                j += 1;
            }
            head = node.next;
        }
        let mut prev = None;
        while let Some(last) = v.pop() {
            prev = ListLink::link(last, prev);
        }
        prev
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let m = 2;
    let n = 3;
    let res = list!(1, 2, 6, 7, 11, 12);
    assert_eq!(Solution::delete_nodes(head, m, n), res);
    let head = list!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let m = 2;
    let n = 3;
    let res = list!(1, 2, 6, 7, 11, 12);
    assert_eq!(Solution::delete_nodes(head, m, n), res);
}
