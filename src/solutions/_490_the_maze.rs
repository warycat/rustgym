struct Solution;

impl Solution {
    fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        let n = maze.len();
        let m = maze[0].len();
        let r = start[0] as usize;
        let c = start[1] as usize;
        let x = destination[0] as usize;
        let y = destination[1] as usize;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        Self::dfs(r, c, &mut visited, &maze, x, y, n, m);
        visited[x][y]
    }

    fn dfs(
        r: usize,
        c: usize,
        visited: &mut Vec<Vec<bool>>,
        maze: &[Vec<i32>],
        x: usize,
        y: usize,
        n: usize,
        m: usize,
    ) {
        if visited[r][c] || visited[x][y] {
            return;
        }
        visited[r][c] = true;
        let mut i = r;
        let mut j = c;
        while i > 0 && maze[i - 1][j] == 0 {
            i -= 1;
        }
        if !visited[i][j] {
            Self::dfs(i, j, visited, maze, x, y, n, m);
        }
        i = r;
        j = c;
        while j > 0 && maze[i][j - 1] == 0 {
            j -= 1;
        }
        if !visited[i][j] {
            Self::dfs(i, j, visited, maze, x, y, n, m);
        }
        i = r;
        j = c;
        while i + 1 < n && maze[i + 1][j] == 0 {
            i += 1;
        }
        if !visited[i][j] {
            Self::dfs(i, j, visited, maze, x, y, n, m);
        }
        i = r;
        j = c;
        while j + 1 < m && maze[i][j + 1] == 0 {
            j += 1;
        }
        if !visited[i][j] {
            Self::dfs(i, j, visited, maze, x, y, n, m);
        }
    }
}

#[test]
fn test() {
    let maze: Vec<Vec<i32>> = vec_vec_i32![
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 1, 1],
        [0, 0, 0, 0, 0]
    ];
    let start = vec![0, 4];
    let destination = vec![4, 4];
    let res = true;
    assert_eq!(Solution::has_path(maze, start, destination), res);
    let maze: Vec<Vec<i32>> = vec_vec_i32![
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 1, 1],
        [0, 0, 0, 0, 0]
    ];
    let start = vec![0, 4];
    let destination = vec![3, 2];
    let res = false;
    assert_eq!(Solution::has_path(maze, start, destination), res);
}
