struct Solution;

impl Solution {
    fn min_swap(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        let mut keep = vec![n; n];
        let mut swap = vec![n; n];
        keep[0] = 0;
        swap[0] = 1;
        for i in 1..n {
            if a[i - 1] < a[i] && b[i - 1] < b[i] {
                keep[i] = keep[i - 1];
                swap[i] = swap[i - 1] + 1;
            }
            if a[i - 1] < b[i] && b[i - 1] < a[i] {
                keep[i] = keep[i].min(swap[i - 1]);
                swap[i] = swap[i].min(keep[i - 1] + 1);
            }
        }
        swap[n - 1].min(keep[n - 1]) as i32
    }
}

#[test]
fn test() {
    let a = vec![1, 3, 5, 4];
    let b = vec![1, 2, 3, 7];
    let res = 1;
    assert_eq!(Solution::min_swap(a, b), res);
}
