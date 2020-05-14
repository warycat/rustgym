struct Solution;

impl Solution {
    fn sum_even_after_queries(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = a.iter().filter(|&x| x % 2 == 0).sum();
        let mut res: Vec<i32> = vec![];
        for query in queries {
            let v = query[0];
            let i = query[1] as usize;
            let x = a[i];
            let y = a[i] + v;
            if x % 2 == 0 {
                sum -= x;
            }
            if y % 2 == 0 {
                sum += y;
            }
            a[i] = y;
            res.push(sum);
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 4];
    let queries: Vec<Vec<i32>> = vec_vec_i32![[1, 0], [-3, 1], [-4, 0], [2, 3]];
    let res = vec![8, 6, 2, 4];
    assert_eq!(Solution::sum_even_after_queries(a, queries), res);
}
