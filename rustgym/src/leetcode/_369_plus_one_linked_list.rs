struct Solution;
use rustgym_util::*;

trait Plus {
    fn plus(self) -> (ListLink, i32);
}

impl Plus for ListLink {
    fn plus(self) -> (ListLink, i32) {
        if let Some(mut node) = self {
            let val = node.val;
            let next = node.next.take();
            if next.is_some() {
                let (tail, carry) = next.plus();
                node.next = tail;
                node.val = (val + carry) % 10;
                let carry = (val + carry) / 10;
                (Some(node), carry)
            } else {
                node.val = (val + 1) % 10;
                let carry = (val + 1) / 10;
                (Some(node), carry)
            }
        } else {
            (None, 0)
        }
    }
}

impl Solution {
    fn plus_one(head: ListLink) -> ListLink {
        let (tail, carry) = head.plus();
        if carry == 1 {
            ListLink::link(1, tail)
        } else {
            tail
        }
    }
}

#[test]
fn test() {
    let head = list!(1, 2, 3);
    let res = list!(1, 2, 4);
    assert_eq!(Solution::plus_one(head), res);
}
