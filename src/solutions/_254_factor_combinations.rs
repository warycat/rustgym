struct Solution;

impl Solution {
    fn get_factors(n: i32) -> Vec<Vec<i32>> {
        let mut cur = vec![];
        let mut res = vec![];
        Self::dfs(2, n, &mut cur, &mut res);
        res
    }

    fn dfs(start: i32, n: i32, cur: &mut Vec<i32>, all: &mut Vec<Vec<i32>>) {
        if n == 1 {
            if cur.len() > 1 {
                all.push(cur.to_vec());
            }
        } else {
            for i in start..=n {
                if n % i == 0 {
                    cur.push(i);
                    Self::dfs(i, n / i, cur, all);
                    cur.pop();
                }
            }
        }
    }
}

#[test]
fn test() {
    let n = 1;
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::get_factors(n), res);
    let n = 37;
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::get_factors(n), res);
    let n = 12;
    let mut res: Vec<Vec<i32>> = vec_vec_i32![[2, 6], [2, 2, 3], [3, 4]];
    let mut ans = Solution::get_factors(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let n = 32;
    let mut res: Vec<Vec<i32>> = vec_vec_i32![
        [2, 16],
        [2, 2, 8],
        [2, 2, 2, 4],
        [2, 2, 2, 2, 2],
        [2, 4, 4],
        [4, 8]
    ];
    let mut ans = Solution::get_factors(n);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
