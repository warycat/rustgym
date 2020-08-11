use rustgym_util::*;

struct BSTIterator {
    root: TreeLink,
    stack: Vec<TreeLink>,
}

impl BSTIterator {
    fn new(root: TreeLink) -> Self {
        let mut stack = vec![];
        Self::push_all_left(root.clone(), &mut stack);
        BSTIterator { root, stack }
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn next(&mut self) -> i32 {
        let link = self.stack.pop().unwrap().unwrap();
        let val = link.borrow().val;
        Self::push_all_left(link.borrow().right.clone(), &mut self.stack);
        val
    }

    fn push_all_left(mut p: TreeLink, stack: &mut Vec<TreeLink>) {
        while let Some(link) = p.clone() {
            stack.push(p.clone());
            p = link.borrow().left.clone();
        }
    }
}

#[test]
fn test() {
    let root = tree!(7, tree!(3), tree!(15, tree!(9), tree!(20)));
    let mut it = BSTIterator::new(root);
    assert_eq!(it.next(), 3);
    assert_eq!(it.next(), 7);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 9);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 15);
    assert_eq!(it.has_next(), true);
    assert_eq!(it.next(), 20);
    assert_eq!(it.has_next(), false);
}
