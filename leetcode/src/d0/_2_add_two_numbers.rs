struct Solution;
use rustgym_util::*;

impl Solution {
    fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        let mut sum: ListLink = None;
        let mut p1: &ListLink = &l1;
        let mut p2: &ListLink = &l2;
        let mut p3: &mut ListLink = &mut sum;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val = carry;
            if let Some(n1) = p1.as_ref() {
                val += n1.val;
                p1 = &n1.next;
            }
            if let Some(n2) = p2.as_ref() {
                val += n2.val;
                p2 = &n2.next;
            }
            carry = val / 10;
            *p3 = ListLink::link(val % 10, None);
            p3 = &mut p3.as_mut().unwrap().next;
        }
        sum
    }
}

#[test]
fn test() {
    let l1 = list!(2, 4, 3);
    let l2 = list!(5, 6, 4);
    let l3 = list!(7, 0, 8);
    assert_eq!(Solution::add_two_numbers(l1, l2), l3);
}
