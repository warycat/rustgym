struct Solution;
use rustgym_util::*;

trait Convert {
    fn convert(self, n: usize) -> (TreeLink, ListLink);
    fn count(&self) -> usize;
}

impl Convert for ListLink {
    fn count(&self) -> usize {
        let mut p = self;
        let mut n = 0;
        while let Some(node) = p {
            p = &node.next;
            n += 1;
        }
        n
    }
    fn convert(self, n: usize) -> (TreeLink, ListLink) {
        if n == 0 {
            return (None, self);
        }
        if n == 1 {
            let mut node = self.unwrap();
            return (tree!(node.val), node.next.take());
        }
        let (left, next) = self.convert(n / 2);
        let (middle, next) = next.convert(1);
        let (right, next) = next.convert(n - n / 2 - 1);
        let node = middle.unwrap();
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        (Some(node), next)
    }
}

impl Solution {
    fn sorted_list_to_bst(head: ListLink) -> TreeLink {
        let n = head.count();
        let (tree, _) = head.convert(n);
        tree
    }
}

#[test]
fn test() {
    let head = list!(-10, -3, 0, 5, 9);
    let res = tree!(0, tree!(-3, tree!(-10), None), tree!(9, tree!(5), None));
    assert_eq!(Solution::sorted_list_to_bst(head), res);
}
