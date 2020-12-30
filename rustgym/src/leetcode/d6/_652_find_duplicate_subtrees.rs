struct Solution;
use rustgym_util::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;

trait Preorder {
    fn preorder(&self, all: &mut Vec<TreeLink>, count: &mut HashMap<u64, usize>);
    fn hash<H: Hasher>(&self, hasher: &mut H);
}

impl Preorder for TreeLink {
    fn preorder(&self, all: &mut Vec<TreeLink>, count: &mut HashMap<u64, usize>) {
        if self.is_some() {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let hash = hasher.finish();
            *count.entry(hash).or_default() += 1;
            if count[&hash] == 2 {
                all.push(self.clone())
            }
        }
        if let Some(node) = self {
            let node = node.borrow();
            node.left.preorder(all, count);
            node.right.preorder(all, count);
        }
    }

    fn hash<H: Hasher>(&self, hasher: &mut H) {
        if let Some(node) = self {
            let node = node.borrow();
            hasher.write_i32(node.val);
            node.left.hash(hasher);
            node.right.hash(hasher);
        } else {
            hasher.write_i32(std::i32::MAX);
        }
    }
}

impl Solution {
    fn find_duplicate_subtrees(root: TreeLink) -> Vec<TreeLink> {
        let mut res: Vec<TreeLink> = vec![];
        let mut count: HashMap<u64, usize> = HashMap::new();
        root.preorder(&mut res, &mut count);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(2, tree!(4), None),
        tree!(3, tree!(2, tree!(4), None), tree!(4))
    );
    let mut res = vec![tree!(2, tree!(4), None), tree!(4)];
    let mut ans = Solution::find_duplicate_subtrees(root);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let root = tree!(
        0,
        tree!(0, tree!(0), None),
        tree!(0, None, tree!(0, None, tree!(0)))
    );
    let mut res = vec![tree!(0)];
    let mut ans = Solution::find_duplicate_subtrees(root);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
