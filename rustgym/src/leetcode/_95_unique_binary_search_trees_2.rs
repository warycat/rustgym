struct Solution;
use rustgym_util::*;

impl Solution {
    fn generate_trees(n: i32) -> Vec<TreeLink> {
        if n == 0 {
            return vec![];
        }
        Self::generate(1, n)
    }

    fn generate(left: i32, right: i32) -> Vec<TreeLink> {
        let mut res = vec![];
        if left > right {
            return vec![None];
        }
        if left == right {
            return vec![tree!(left)];
        }
        for middle in left..=right {
            for i in Self::generate(left, middle - 1) {
                for j in Self::generate(middle + 1, right) {
                    res.push(tree!(middle, i.clone(), j));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let mut res = vec![
        tree!(1, None, tree!(3, tree!(2), None)),
        tree!(3, tree!(2, tree!(1), None), None),
        tree!(3, tree!(1, None, tree!(2)), None),
        tree!(2, tree!(1), tree!(3)),
        tree!(1, None, tree!(2, None, tree!(3))),
    ];
    let mut ans = Solution::generate_trees(n);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
