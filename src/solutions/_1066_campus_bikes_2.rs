struct Solution;

impl Solution {
    fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let n = workers.len();
        let m = bikes.len();
        let mut cur = (0..m).collect();
        let mut res = std::i32::MAX;
        Self::dfs(0, 0, &mut cur, &mut res, &workers, &bikes, n, m);
        res
    }

    fn dfs(
        start: usize,
        sum: i32,
        cur: &mut Vec<usize>,
        min: &mut i32,
        workers: &[Vec<i32>],
        bikes: &[Vec<i32>],
        n: usize,
        m: usize,
    ) {
        if start == n {
            *min = (*min).min(sum);
        } else {
            for i in start..m {
                cur.swap(start, i);
                let dist = Self::dist(&workers[start], &bikes[cur[start]]);
                if sum + dist < *min {
                    Self::dfs(start + 1, sum + dist, cur, min, workers, bikes, n, m);
                }
                cur.swap(start, i);
            }
        }
    }

    fn dist(worker: &[i32], bike: &[i32]) -> i32 {
        (worker[0] - bike[0]).abs() + (worker[1] - bike[1]).abs()
    }
}

#[test]
fn test() {
    let workers = vec_vec_i32![[0, 0], [2, 1]];
    let bikes = vec_vec_i32![[1, 2], [3, 3]];
    let res = 6;
    assert_eq!(Solution::assign_bikes(workers, bikes), res);
    let workers = vec_vec_i32![[0, 0], [1, 1], [2, 0]];
    let bikes = vec_vec_i32![[1, 0], [2, 2], [2, 1]];
    let res = 4;
    assert_eq!(Solution::assign_bikes(workers, bikes), res);
}
