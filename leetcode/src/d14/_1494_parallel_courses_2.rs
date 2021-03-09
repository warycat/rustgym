struct Solution;

struct Solver {
    n: usize,
    k: usize,
    pre: Vec<u32>,
    memo: Vec<i32>,
}

impl Solver {
    fn new(n: usize, k: usize, pre: Vec<u32>) -> Self {
        let mut memo: Vec<i32> = vec![-1; (1 << n) as usize];
        memo[0] = 0;
        Solver { n, k, pre, memo }
    }

    fn dfs(&self, k: usize, cur: u32, available: u32, groups: &mut Vec<u32>) {
        if k == 0 {
            groups.push(cur);
        }
        if available != 0 && k > 0 {
            let bit = 1 << available.trailing_zeros();
            self.dfs(k - 1, cur | bit, available & !bit, groups);
            self.dfs(k, cur, available & !bit, groups);
        }
    }

    fn dp(&mut self, left: u32) -> i32 {
        if self.memo[left as usize] != -1 {
            self.memo[left as usize]
        } else {
            let mut available: u32 = 0;
            for i in 0..self.n {
                if left & 1 << i != 0 && self.pre[i] & left == 0 {
                    available |= 1 << i;
                }
            }
            let res = if available.count_ones() <= self.k as u32 {
                1 + self.dp(left & !available)
            } else {
                let mut groups: Vec<u32> = vec![];
                self.dfs(self.k, 0, available, &mut groups);
                groups
                    .into_iter()
                    .map(|g| 1 + self.dp(left & !g))
                    .min()
                    .unwrap()
            };
            self.memo[left as usize] = res;
            res
        }
    }
}

impl Solution {
    fn min_number_of_semesters(n: i32, dependencies: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut pre = vec![0; n];
        for dependency in dependencies {
            let x = dependency[0] as usize - 1;
            let y = dependency[1] as usize - 1;
            pre[y] |= 1 << x;
        }
        let mut solver = Solver::new(n, k, pre);
        solver.dp((1 << n) - 1)
    }
}

#[test]
fn test() {
    let n = 4;
    let dependencies = vec_vec_i32![[2, 1], [3, 1], [1, 4]];
    let k = 2;
    let res = 3;
    assert_eq!(Solution::min_number_of_semesters(n, dependencies, k), res);
    let n = 5;
    let dependencies = vec_vec_i32![[2, 1], [3, 1], [4, 1], [1, 5]];
    let k = 2;
    let res = 4;
    assert_eq!(Solution::min_number_of_semesters(n, dependencies, k), res);
    let n = 11;
    let dependencies = vec_vec_i32![];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::min_number_of_semesters(n, dependencies, k), res);
}
