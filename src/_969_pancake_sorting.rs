struct Solution;

impl Solution {
    fn pancake_sort(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![];
        for i in 0..n {
            let (_, j) = (0..n - i).fold(
                (a[0], 0),
                |acc, j| {
                    if a[j] > acc.0 {
                        (a[j], j)
                    } else {
                        acc
                    }
                },
            );
            a[0..=j].reverse();
            a[0..n - i].reverse();
            res.push(j + 1);
            res.push(n - i);
        }
        res.iter().map(|&x| x as i32).collect()
    }
}

#[test]
fn test() {
    let a = vec![3, 2, 4, 1];
    let res = vec![3, 4, 2, 3, 1, 2, 1, 1];
    assert_eq!(Solution::pancake_sort(a), res);
}
