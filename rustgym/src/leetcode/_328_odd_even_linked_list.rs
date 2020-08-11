struct Solution;
use rustgym_util::*;

impl Solution {
    fn odd_even_list(mut head: ListLink) -> ListLink {
        let mut odd: Vec<ListLink> = vec![];
        let mut even: Vec<ListLink> = vec![];
        let mut i = 1;
        while let Some(mut node) = head {
            head = node.next.take();
            if i % 2 == 1 {
                odd.push(Some(node));
            } else {
                even.push(Some(node));
            }
            i += 1;
        }
        let mut res: ListLink = None;
        while let Some(mut link) = even.pop() {
            link.as_mut().unwrap().next = res;
            res = link;
        }
        while let Some(mut link) = odd.pop() {
            link.as_mut().unwrap().next = res;
            res = link;
        }
        res
    }
}

#[test]
fn test() {
    let head = list![1, 2, 3, 4, 5];
    let res = list![1, 3, 5, 2, 4];
    assert_eq!(Solution::odd_even_list(head), res);
    let head = list![2, 1, 3, 5, 6, 4, 7];
    let res = list![2, 3, 6, 7, 1, 5, 4];
    assert_eq!(Solution::odd_even_list(head), res);
}
