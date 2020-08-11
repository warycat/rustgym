use rustgym_util::*;
use std::collections::HashSet;

trait Preorder {
    fn recover(&mut self, x: i32, hs: &mut HashSet<i32>);
}

impl Preorder for TreeLink {
    fn recover(&mut self, x: i32, hs: &mut HashSet<i32>) {
        if let Some(node) = self {
            hs.insert(x);
            node.borrow_mut().val = x;
            node.borrow_mut().left.recover(2 * x + 1, hs);
            node.borrow_mut().right.recover(2 * x + 2, hs);
        }
    }
}

struct FindElements {
    root: TreeLink,
    hs: HashSet<i32>,
}

impl FindElements {
    fn new(mut root: TreeLink) -> Self {
        let mut hs = HashSet::new();
        root.recover(0, &mut hs);
        FindElements { root, hs }
    }

    fn find(&self, target: i32) -> bool {
        self.hs.contains(&target)
    }
}

#[test]
fn test() {
    let root = tree!(-1, None, tree!(-1));
    let fe = FindElements::new(root);
    let target = 1;
    let res = false;
    assert_eq!(fe.find(target), res);
    let target = 2;
    let res = true;
    assert_eq!(fe.find(target), res);
    let root = tree!(-1, tree!(-1, tree!(-1), tree!(-1)), tree!(-1));
    let fe = FindElements::new(root);
    let target = 1;
    let res = true;
    assert_eq!(fe.find(target), res);
    let target = 3;
    let res = true;
    assert_eq!(fe.find(target), res);
    let target = 5;
    let res = false;
    assert_eq!(fe.find(target), res);
    let root = tree!(-1, None, tree!(-1, tree!(-1, tree!(-1), None), None));
    let fe = FindElements::new(root);
    let target = 2;
    let res = true;
    assert_eq!(fe.find(target), res);
    let target = 3;
    let res = false;
    assert_eq!(fe.find(target), res);
    let target = 4;
    let res = false;
    assert_eq!(fe.find(target), res);
    let target = 5;
    let res = true;
    assert_eq!(fe.find(target), res);
}
