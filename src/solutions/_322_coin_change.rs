struct Solution;

impl Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = (amount + 1) as usize;
        let mut a = vec![-1; n];
        a[0] = 0;
        for i in 1..n {
            for &coin in &coins {
                if coin as usize <= i {
                    let j = i - coin as usize;
                    if a[j] != -1 {
                        if a[i] == -1 {
                            a[i] = a[j] + 1
                        } else {
                            a[i] = i32::min(a[i], a[j] + 1);
                        }
                    }
                }
            }
        }
        a[amount as usize]
    }
}

#[test]
fn test() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    let res = 3;
    assert_eq!(Solution::coin_change(coins, amount), res);
    let coins = vec![2];
    let amount = 3;
    let res = -1;
    assert_eq!(Solution::coin_change(coins, amount), res);
}
