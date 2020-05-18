struct Solution;

impl Solution {
    fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut rows: Vec<Vec<i32>> = vec![];
        for i in 0..n {
            for (j, &v) in nums[i].iter().enumerate() {
                let k = i + j;
                if rows.len() == k {
                    rows.push(vec![]);
                }
                rows[k].push(v);
            }
        }
        rows.into_iter().flat_map(|q| q.into_iter().rev()).collect()
    }
}

#[test]
fn test() {
    let nums = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = vec![1, 4, 2, 7, 5, 3, 8, 6, 9];
    assert_eq!(Solution::find_diagonal_order(nums), res);
    let nums = vec_vec_i32![
        [1, 2, 3, 4, 5],
        [6, 7],
        [8],
        [9, 10, 11],
        [12, 13, 14, 15, 16]
    ];
    let res = vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16];
    assert_eq!(Solution::find_diagonal_order(nums), res);
    let nums = vec_vec_i32![[1, 2, 3], [4], [5, 6, 7], [8], [9, 10, 11]];
    let res = vec![1, 4, 2, 5, 3, 8, 6, 9, 7, 10, 11];
    assert_eq!(Solution::find_diagonal_order(nums), res);
    let nums = vec_vec_i32![[1, 2, 3, 4, 5, 6]];
    let res = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::find_diagonal_order(nums), res);
}
