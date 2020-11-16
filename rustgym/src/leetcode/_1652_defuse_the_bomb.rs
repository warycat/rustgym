struct Solution;

impl Solution {
    fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        if k == 0 {
            return vec![0; n];
        }
        let mut res = vec![];
        for i in 0..n {
            let mut sum = 0;
            for j in 0..k.abs() {
                let index = if k > 0 {
                    (n as i32 + (i as i32 + 1 + j)) as usize % n
                } else {
                    (n as i32 + (i as i32 - 1 - j)) as usize % n
                };
                sum += code[index];
            }
            res.push(sum);
        }
        res
    }
}
#[test]
fn test() {
    let code = vec![5, 7, 1, 4];
    let k = 3;
    let res = vec![12, 10, 16, 13];
    assert_eq!(Solution::decrypt(code, k), res);
    let code = vec![1, 2, 3, 4];
    let k = 0;
    let res = vec![0, 0, 0, 0];
    assert_eq!(Solution::decrypt(code, k), res);
    let code = vec![2, 4, 9, 3];
    let k = -2;
    let res = vec![12, 5, 6, 13];
    assert_eq!(Solution::decrypt(code, k), res);
}
