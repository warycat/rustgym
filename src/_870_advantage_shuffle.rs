struct Solution;

type Pair = (usize, i32);

impl Solution {
    fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut b: Vec<Pair> = b.into_iter().enumerate().collect();
        let mut l = 0;
        let mut r = n - 1;
        a.sort_unstable();
        b.sort_unstable_by_key(|p| p.1);
        let mut res: Vec<i32> = vec![0; n];
        for i in 0..n {
            if a[i] <= b[l].1 {
                res[b[r].0] = a[i];
                r -= 1;
            } else {
                res[b[l].0] = a[i];
                l += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![2, 7, 11, 15];
    let b = vec![1, 10, 4, 11];
    let res = vec![2, 11, 7, 15];
    assert_eq!(Solution::advantage_count(a, b), res);
    let a = vec![12, 24, 8, 32];
    let b = vec![13, 25, 32, 11];
    let res = vec![24, 32, 8, 12];
    assert_eq!(Solution::advantage_count(a, b), res);
}
