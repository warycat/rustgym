struct Solution;

impl Solution {
    fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let m = nums[0].len();
        let r = r as usize;
        let c = c as usize;
        if n * m != r as usize * c as usize {
            return nums;
        }
        let mut res: Vec<Vec<i32>> = vec![vec![0; c]; r];
        for i in 0..n {
            for j in 0..m {
                let k = i * m + j;
                res[k / c][k % c] = nums[i][j];
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [3, 4]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3, 4]];
    assert_eq!(Solution::matrix_reshape(nums, 1, 4), res);
}
