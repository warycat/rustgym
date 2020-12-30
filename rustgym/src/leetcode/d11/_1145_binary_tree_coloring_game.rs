struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, x: i32, left: &mut usize, right: &mut usize) -> usize;
}

impl Postorder for TreeLink {
    fn postorder(&self, x: i32, left: &mut usize, right: &mut usize) -> usize {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let l = node.left.postorder(x, left, right);
            let r = node.right.postorder(x, left, right);
            if val == x {
                *left = l;
                *right = r;
            }
            l + r + 1
        } else {
            0
        }
    }
}

impl Solution {
    fn btree_game_winning_move(root: TreeLink, n: i32, x: i32) -> bool {
        let mut left = 0;
        let mut right = 0;
        root.postorder(x, &mut left, &mut right);
        let n = n as usize;
        let top = n - 1 - left - right;
        top.max(left.max(right)) > n / 2
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(
            2,
            tree!(4, tree!(8), tree!(9)),
            tree!(5, tree!(10), tree!(11))
        ),
        tree!(3, tree!(6), tree!(7))
    );
    let n = 11;
    let x = 3;
    let res = true;
    assert_eq!(Solution::btree_game_winning_move(root, n, x), res);
}
