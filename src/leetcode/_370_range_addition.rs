struct Solution;

impl Solution {
    fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let n = length as usize;
        let mut res = vec![0; n];
        let mut batched = vec![0; n + 1];
        for update in updates {
            let start = update[0] as usize;
            let end = update[1] as usize + 1;
            let inc = update[2];
            batched[start] += inc;
            batched[end] -= inc;
        }
        let mut sum = 0;
        for i in 0..n {
            sum += batched[i];
            res[i] = sum;
        }
        res
    }
}

#[test]
fn test() {
    let length = 5;
    let updates = vec_vec_i32![[1, 3, 2], [2, 4, 3], [0, 2, -2]];
    let res = vec![-2, 0, 3, 5, 3];
    assert_eq!(Solution::get_modified_array(length, updates), res);
}
