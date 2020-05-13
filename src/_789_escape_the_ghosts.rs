struct Solution;

impl Solution {
    fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let min = Self::dist(&target, &[0, 0]);
        for ghost in &ghosts {
            if Self::dist(ghost, &target) <= min {
                return false;
            }
        }
        true
    }

    fn dist(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }
}

#[test]
fn test() {
    let ghosts = vec_vec_i32![[1, 0], [0, 3]];
    let target = vec![0, 1];
    let res = true;
    assert_eq!(Solution::escape_ghosts(ghosts, target), res);
    let ghosts = vec_vec_i32![[1, 0]];
    let target = vec![2, 0];
    let res = false;
    assert_eq!(Solution::escape_ghosts(ghosts, target), res);
    let ghosts = vec_vec_i32![[2, 0]];
    let target = vec![1, 0];
    let res = false;
    assert_eq!(Solution::escape_ghosts(ghosts, target), res);
}
