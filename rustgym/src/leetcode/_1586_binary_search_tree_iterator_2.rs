use rustgym_util::*;

trait Inorder {
    fn inorder(&self, nodes: &mut Vec<i32>);
}

impl Inorder for TreeLink {
    fn inorder(&self, nodes: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.inorder(nodes);
            nodes.push(node.val);
            node.right.inorder(nodes);
        }
    }
}

struct BSTIterator {
    nodes: Vec<i32>,
    index: i32,
    size: usize,
}

impl BSTIterator {
    fn new(root: TreeLink) -> Self {
        let mut nodes = vec![];
        root.inorder(&mut nodes);
        let index = -1;
        let size = nodes.len();
        BSTIterator { nodes, index, size }
    }

    fn has_next(&self) -> bool {
        ((self.index + 1) as usize) < self.size
    }

    fn next(&mut self) -> i32 {
        self.index += 1;
        self.nodes[self.index as usize]
    }

    fn has_prev(&mut self) -> bool {
        self.index > 0
    }

    fn prev(&mut self) -> i32 {
        self.index -= 1;
        self.nodes[self.index as usize]
    }
}

#[test]
fn test() {
    let root = tree!(7, tree!(3), tree!(15, tree!(9), tree!(20)));
    let mut obj = BSTIterator::new(root);
    assert_eq!(obj.next(), 3);
    assert_eq!(obj.next(), 7);
    assert_eq!(obj.prev(), 3);
    assert_eq!(obj.next(), 7);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 9);
    assert_eq!(obj.next(), 15);
    assert_eq!(obj.next(), 20);
    assert_eq!(obj.has_next(), false);
    assert_eq!(obj.has_prev(), true);
    assert_eq!(obj.prev(), 15);
    assert_eq!(obj.prev(), 9);
}
