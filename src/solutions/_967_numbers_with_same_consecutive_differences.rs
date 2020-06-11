struct Solution;

impl Solution {
    fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let n = n as usize;
        let mut cur: Vec<i32> = vec![];
        let mut res = vec![];
        for i in 0..10 {
            if i == 0 && n != 1 {
                continue;
            }
            cur.push(i);
            Self::dfs(1, &mut cur, &mut res, k, n);
            cur.pop();
        }
        res
    }
    fn dfs(start: usize, cur: &mut Vec<i32>, all: &mut Vec<i32>, k: i32, n: usize) {
        if start == n {
            let mut x = 0;
            for i in 0..n {
                x *= 10;
                x += cur[i];
            }
            all.push(x);
        } else {
            for i in 0..10 {
                if (cur[start - 1] - i).abs() != k {
                    continue;
                }
                cur.push(i);
                Self::dfs(start + 1, cur, all, k, n);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 7;
    let res = vec![181, 292, 707, 818, 929];
    assert_eq!(Solution::nums_same_consec_diff(n, k), res);
    let n = 2;
    let k = 1;
    let res = vec![
        10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98,
    ];
    assert_eq!(Solution::nums_same_consec_diff(n, k), res);
}
