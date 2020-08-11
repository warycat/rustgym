struct Solution;
use rustgym_util::*;

impl Solution {
    fn delete_duplicates(mut head: ListLink) -> ListLink {
        let mut res: ListLink = None;
        let mut stack: Vec<(i32, usize)> = vec![];
        while let Some(node) = head {
            let val = node.val;
            match stack.last() {
                Some(&(pval, count)) if pval == val => {
                    stack.pop();
                    stack.push((val, count + 1));
                }
                _ => {
                    stack.push((val, 1));
                }
            }
            head = node.next;
        }
        while let Some((val, count)) = stack.pop() {
            if count == 1 {
                res = ListLink::link(val, res);
            }
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3, 3, 4, 4, 5);
    let res = list!(1, 2, 5);
    assert_eq!(Solution::delete_duplicates(head), res);
    let head = list!(1, 1, 1, 2, 3);
    let res = list!(2, 3);
    assert_eq!(Solution::delete_duplicates(head), res);
}
