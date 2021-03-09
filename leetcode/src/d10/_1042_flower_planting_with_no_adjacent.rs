struct Solution;

impl Solution {
    fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for path in paths {
            let u = (path[0] - 1) as usize;
            let v = (path[1] - 1) as usize;
            g[u].push(v);
            g[v].push(u);
        }
        let mut colors: Vec<i32> = vec![0; n];
        for i in 0..n {
            let mut used: Vec<bool> = vec![false; 5];
            for &j in &g[i] {
                used[colors[j] as usize] = true;
            }
            for c in 1..5 {
                if !used[c] {
                    colors[i] = c as i32;
                    break;
                }
            }
        }
        colors
    }
}

#[test]
fn test() {
    let n = 3;
    let paths: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [2, 3], [3, 1]];
    let res = vec![1, 2, 3];
    assert_eq!(Solution::garden_no_adj(n, paths), res);
    let n = 4;
    let paths: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [3, 4]];
    let res = vec![1, 2, 1, 2];
    assert_eq!(Solution::garden_no_adj(n, paths), res);
    let n = 4;
    let paths: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]];
    let res = vec![1, 2, 3, 4];
    assert_eq!(Solution::garden_no_adj(n, paths), res);
}
