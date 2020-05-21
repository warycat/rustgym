struct Solution;

impl Solution {
    fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut v: Vec<i32> = (1..=m).collect();
        let mut res = vec![];
        for q in queries {
            let p = v.iter().position(|&x| x == q).unwrap();
            v.remove(p);
            v.insert(0, q);
            res.push(p as i32);
        }
        res
    }
}

#[test]
fn test() {
    let queries = vec![3, 1, 2, 1];
    let m = 5;
    let res = vec![2, 1, 2, 1];
    assert_eq!(Solution::process_queries(queries, m), res);
    let queries = vec![4, 1, 2, 2];
    let m = 4;
    let res = vec![3, 1, 2, 0];
    assert_eq!(Solution::process_queries(queries, m), res);
    let queries = vec![7, 5, 5, 8, 3];
    let m = 8;
    let res = vec![6, 5, 0, 7, 5];
    assert_eq!(Solution::process_queries(queries, m), res);
}
