struct Solution;
use rustgym_util::*;

impl Solution {
    fn delete_duplicates(mut head: ListLink) -> ListLink {
        let mut p = head.as_mut();
        while let Some(n) = p {
            while let Some(m) = n.next.as_mut() {
                if m.val != n.val {
                    break;
                }
                n.next = m.next.take();
            }
            p = n.next.as_mut();
        }
        head
    }
}

#[test]
fn test() {
    let p = list!(1, 1, 2, 3, 3);
    let q = list!(1, 2, 3);
    assert_eq!(Solution::delete_duplicates(p), q);
}
