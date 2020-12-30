struct Solution;
use rustgym_util::*;

impl Solution {
    fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        let mut v1: Vec<i32> = vec![];
        let mut v2: Vec<i32> = vec![];
        let mut p1 = &l1;
        let mut p2 = &l2;
        while let Some(n1) = p1 {
            v1.push(n1.val);
            p1 = &n1.next;
        }
        while let Some(n2) = p2 {
            v2.push(n2.val);
            p2 = &n2.next;
        }
        let mut carry = 0;
        let mut res = None;
        while !v1.is_empty() || !v2.is_empty() || carry != 0 {
            let mut sum = 0;
            if let Some(x1) = v1.pop() {
                sum += x1;
            }
            if let Some(x2) = v2.pop() {
                sum += x2;
            }
            sum += carry;
            res = ListLink::link(sum % 10, res);
            carry = sum / 10;
        }
        res
    }
}

#[test]
fn test() {
    let l1 = list!(7, 2, 4, 3);
    let l2 = list!(5, 6, 4);
    let res = list!(7, 8, 0, 7);
    assert_eq!(Solution::add_two_numbers(l1, l2), res);
}
