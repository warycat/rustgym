struct Solution;

impl Solution {
    fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        let n = coins.len();
        coins.sort_unstable();
        let mut prefix = 0;
        for i in 0..n {
            if coins[i] > prefix + 1 {
                break;
            } else {
                prefix += coins[i];
            }
        }
        prefix + 1
    }
}

#[test]
fn test() {
    let coins = vec![1, 3];
    let res = 2;
    assert_eq!(Solution::get_maximum_consecutive(coins), res);
    let coins = vec![1, 1, 1, 4];
    let res = 8;
    assert_eq!(Solution::get_maximum_consecutive(coins), res);
    let coins = vec![1, 4, 10, 3, 1];
    let res = 20;
    assert_eq!(Solution::get_maximum_consecutive(coins), res);
}
