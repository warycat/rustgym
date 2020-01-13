struct Solution;
use crate::util::*;

impl Solution {
    fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> TreeLink {
        let n = preorder.len();
        Self::parse_tree(&preorder[0..n], &inorder[0..n])
    }

    fn parse_tree(preorder: &[i32], inorder: &[i32]) -> TreeLink {
        if preorder.len() == 0 {
            return None;
        }
        let val = preorder[0];
        let n = preorder.len();
        if n == 1 {
            tree!(val)
        } else {
            let mut mid = 0;
            for i in 0..n {
                if inorder[i] == val {
                    mid = i;
                    break;
                }
            }
            tree!(
                val,
                Self::parse_tree(&preorder[1..mid + 1], &inorder[0..mid]),
                Self::parse_tree(&preorder[mid + 1..], &inorder[mid + 1..])
            )
        }
    }
}

#[test]
fn test() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::build_tree(preorder, inorder), res);
}
