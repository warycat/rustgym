struct Solution;

use std::collections::HashMap;

impl Solution {
    fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut row: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut col: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let r = stones[i][0];
            let c = stones[i][1];
            row.entry(r).or_default().push(i);
            col.entry(c).or_default().push(i);
        }
        let mut visited: Vec<bool> = vec![false; n];
        let mut island = 0;
        for i in 0..n {
            if !visited[i] {
                visited[i] = true;
                Self::dfs(i, &mut visited, &stones, &row, &col);
                island += 1;
            }
        }
        (n - island) as i32
    }

    fn dfs(
        u: usize,
        visited: &mut [bool],
        stones: &[Vec<i32>],
        row: &HashMap<i32, Vec<usize>>,
        col: &HashMap<i32, Vec<usize>>,
    ) {
        let r = stones[u][0];
        let c = stones[u][1];
        for &v in &row[&r] {
            if !visited[v] {
                visited[v] = true;
                Self::dfs(v, visited, stones, row, col);
            }
        }
        for &v in &col[&c] {
            if !visited[v] {
                visited[v] = true;
                Self::dfs(v, visited, stones, row, col);
            }
        }
    }
}

#[test]
fn test() {
    let stones = vec_vec_i32![[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]];
    let res = 5;
    assert_eq!(Solution::remove_stones(stones), res);
    let stones = vec_vec_i32![[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]];
    let res = 3;
    assert_eq!(Solution::remove_stones(stones), res);
    let stones = vec_vec_i32![[0, 0]];
    let res = 0;
    assert_eq!(Solution::remove_stones(stones), res);
}
