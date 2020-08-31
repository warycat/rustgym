struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut table = vec![];
        for i in 0..=n {
            table.push(vec![1; i + 1]);
            for j in 1..i {
                table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MOD;
            }
        }
        Self::ways(nums, &table) as i32 - 1
    }

    fn ways(nums: Vec<i32>, table: &[Vec<i64>]) -> i64 {
        if nums.is_empty() {
            return 1;
        }
        let root = nums[0];
        let left: Vec<i32> = nums.iter().filter(|&&x| x < root).copied().collect();
        let right: Vec<i32> = nums.iter().filter(|&&x| x > root).copied().collect();
        let l = left.len();
        let r = right.len();
        let mut res = table[l + r][l];
        res %= MOD;
        res *= Self::ways(right, table);
        res %= MOD;
        res *= Self::ways(left, table);
        res %= MOD;
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 3];
    let res = 1;
    assert_eq!(Solution::num_of_ways(nums), res);
    let nums = vec![3, 4, 5, 1, 2];
    let res = 5;
    assert_eq!(Solution::num_of_ways(nums), res);
    let nums = vec![1, 2, 3];
    let res = 0;
    assert_eq!(Solution::num_of_ways(nums), res);
    let nums = vec![3, 1, 2, 5, 4, 6];
    let res = 19;
    assert_eq!(Solution::num_of_ways(nums), res);
    let nums = vec![
        9, 4, 2, 1, 3, 6, 5, 7, 8, 14, 11, 10, 12, 13, 16, 15, 17, 18,
    ];
    let res = 216212978;
    assert_eq!(Solution::num_of_ways(nums), res);
}
