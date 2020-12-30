struct Solution;
use rustgym_util::*;

impl Solution {
    fn all_possible_fbt(n: i32) -> Vec<TreeLink> {
        if n % 2 == 0 {
            return vec![];
        }
        if n == 1 {
            vec![tree!(0)]
        } else {
            let mut res = vec![];
            let mut l = 1;
            let mut r = n - 1 - l;
            while r > 0 {
                let left_trees = Self::all_possible_fbt(l);
                let right_trees = Self::all_possible_fbt(r);
                for left in &left_trees {
                    for right in &right_trees {
                        res.push(tree!(0, left.clone(), right.clone()));
                    }
                }
                r -= 2;
                l += 2;
            }
            res
        }
    }
}

#[test]
fn test() {
    let n = 7;
    let res = [
        tree!(
            0,
            tree!(0),
            tree!(0, tree!(0), tree!(0, tree!(0), tree!(0)))
        ),
        tree!(
            0,
            tree!(0),
            tree!(0, tree!(0, tree!(0), tree!(0)), tree!(0))
        ),
        tree!(
            0,
            tree!(0, tree!(0), tree!(0)),
            tree!(0, tree!(0), tree!(0))
        ),
        tree!(
            0,
            tree!(0, tree!(0), tree!(0, tree!(0), tree!(0))),
            tree!(0)
        ),
        tree!(
            0,
            tree!(0, tree!(0, tree!(0), tree!(0)), tree!(0)),
            tree!(0)
        ),
    ];
    assert_eq!(Solution::all_possible_fbt(n), res);
}
