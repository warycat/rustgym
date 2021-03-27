use rustgym_util::*;
struct CBTInserter {
    stack: Vec<TreeLink>,
}

impl CBTInserter {
    fn new(root: TreeLink) -> Self {
        let mut stack: Vec<TreeLink> = vec![root];
        let mut i = 0;
        while i < stack.len() {
            let left = stack[i].as_mut().unwrap().borrow_mut().left.clone();
            let right = stack[i].as_mut().unwrap().borrow_mut().right.clone();
            if left.is_some() {
                stack.push(left);
            }
            if right.is_some() {
                stack.push(right);
            }
            i += 1;
        }
        CBTInserter { stack }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let link = tree!(v);
        let n = self.stack.len();
        self.stack.push(link.clone());
        let mut parent = self.stack[(n - 1) / 2].as_mut().unwrap().borrow_mut();
        if n % 2 == 1 {
            parent.left = link;
        } else {
            parent.right = link;
        }
        parent.val
    }

    fn get_root(&self) -> TreeLink {
        self.stack[0].clone()
    }
}

#[test]
fn test() {
    let mut obj = CBTInserter::new(tree!(1));
    assert_eq!(obj.insert(2), 1);
    assert_eq!(obj.get_root(), tree!(1, tree!(2), None));
}
