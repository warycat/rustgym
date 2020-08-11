struct Solution;
use rustgym_util::*;

impl Solution {
    fn remove_elements(mut head: ListLink, val: i32) -> ListLink {
        let mut p = &mut head;
        while p.is_some() {
            if p.as_ref().unwrap().val == val {
                *p = p.take().unwrap().next.take();
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }
        head
    }
}

#[test]
fn test() {
    let input = list!(1, 2, 6, 3, 4, 5, 6);
    let output = list!(1, 2, 3, 4, 5);
    assert_eq!(Solution::remove_elements(input, 6), output);
}
