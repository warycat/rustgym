struct Solution;

impl Solution {
    fn dfs(m: &mut Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize, n: usize) {
        for j in 0..n {
            if m[i][j] == 1 && !visited[j] {
                visited[j] = true;
                Self::dfs(m, visited, j, n);
            }
        }
    }

    fn find_circle_num(mut m: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = m.len();
        let mut visited: Vec<bool> = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                visited[i] = true;
                Self::dfs(&mut m, &mut visited, i, n);
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let m: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 0], [1, 1, 0], [0, 0, 1]];
    assert_eq!(Solution::find_circle_num(m), 2);
    let m: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 0], [1, 1, 1], [0, 1, 1]];
    assert_eq!(Solution::find_circle_num(m), 1);
}
