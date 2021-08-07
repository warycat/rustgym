struct Solution;
use rustgym_util::*;

impl Solution {
    fn is_same_tree(p: TreeLink, q: TreeLink) -> bool {
        p == q
    }
}

#[test]
fn test() {
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let p = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let res = true;
    assert_eq!(Solution::is_same_tree(p, q), res);
}
