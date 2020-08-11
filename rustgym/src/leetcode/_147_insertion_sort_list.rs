struct Solution;
use rustgym_util::*;

trait Insertion {
    fn insert(self, link: ListLink) -> ListLink;
}

impl Insertion for ListLink {
    fn insert(self, mut link: ListLink) -> ListLink {
        let val = link.as_ref().unwrap().val;
        if let Some(mut node) = self {
            if node.val > val {
                link.as_mut().unwrap().next = Some(node);
                link
            } else {
                node.next = node.next.take().insert(link);
                Some(node)
            }
        } else {
            link
        }
    }
}

impl Solution {
    fn insertion_sort_list(mut head: ListLink) -> ListLink {
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            prev = prev.insert(Some(node));
        }
        prev
    }
}

#[test]
fn test() {
    let head = list!(4, 2, 1, 3);
    let res = list!(1, 2, 3, 4);
    assert_eq!(Solution::insertion_sort_list(head), res);
    let head = list!(-1, 5, 3, 4, 0);
    let res = list!(-1, 0, 3, 4, 5);
    assert_eq!(Solution::insertion_sort_list(head), res);
}
