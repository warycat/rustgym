struct Solution;

impl Solution {
    fn min_distance(
        _: i32,
        _: i32,
        tree: Vec<i32>,
        squirrel: Vec<i32>,
        nuts: Vec<Vec<i32>>,
    ) -> i32 {
        let mut sum = 0;
        let mut diff = std::i32::MIN;
        for nut in nuts {
            let t = Self::dist(&nut, &tree);
            let s = Self::dist(&nut, &squirrel);
            sum += t * 2;
            diff = diff.max(t - s);
        }
        sum - diff
    }

    fn dist(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }
}

#[test]
fn test() {
    let tree = vec![2, 2];
    let squirrle = vec![4, 4];
    let nuts = vec_vec_i32![[3, 0], [2, 5]];
    let res = 12;
    assert_eq!(Solution::min_distance(0, 0, tree, squirrle, nuts), res);
}
