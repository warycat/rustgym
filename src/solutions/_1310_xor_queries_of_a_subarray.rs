struct Solution;

impl Solution {
    fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arr.len();
        for i in 1..n {
            arr[i] ^= arr[i - 1];
        }
        let mut res = vec![];
        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize;
            let x = if l > 0 { arr[r] ^ arr[l - 1] } else { arr[r] };
            res.push(x);
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, 3, 4, 8];
    let queries = vec_vec_i32![[0, 1], [1, 2], [0, 3], [3, 3]];
    let res = vec![2, 7, 14, 8];
    assert_eq!(Solution::xor_queries(arr, queries), res);
    let arr = vec![4, 8, 2, 10];
    let queries = vec_vec_i32![[2, 3], [1, 3], [0, 0], [0, 3]];
    let res = vec![8, 0, 4, 4];
    assert_eq!(Solution::xor_queries(arr, queries), res);
}
