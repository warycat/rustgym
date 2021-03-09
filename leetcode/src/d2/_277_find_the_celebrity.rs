struct Solution {
    adj: Vec<Vec<bool>>,
    n: usize,
}

impl Solution {
    fn new(graph: Vec<Vec<i32>>) -> Self {
        let n = graph.len();
        let mut adj = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                adj[i][j] = graph[i][j] == 1;
            }
        }
        Solution { adj, n }
    }

    fn knows(&self, a: i32, b: i32) -> bool {
        self.adj[a as usize][b as usize]
    }
}

impl Solution {
    fn find_celebrity(&self, n: i32) -> i32 {
        let n = n as usize;
        for i in 0..n {
            let mut x = 0;
            let mut y = 0;
            for j in 0..n {
                if i != j {
                    if self.knows(i as i32, j as i32) {
                        x += 1;
                    }
                    if self.knows(j as i32, i as i32) {
                        y += 1;
                    }
                }
            }
            if x == 0 && y == n - 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let graph = vec_vec_i32![[1, 1, 0], [0, 1, 0], [1, 1, 1]];
    let solution = Solution::new(graph);
    let n = 3;
    let res = 1;
    assert_eq!(solution.find_celebrity(n), res);
    let graph = vec_vec_i32![[1, 0, 1], [1, 1, 0], [0, 1, 1]];
    let solution = Solution::new(graph);
    let n = 3;
    let res = -1;
    assert_eq!(solution.find_celebrity(n), res);
}
