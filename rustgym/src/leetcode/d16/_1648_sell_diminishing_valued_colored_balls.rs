struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn max_profit(mut inventory: Vec<i32>, mut orders: i32) -> i32 {
        let n = inventory.len();
        inventory.sort_unstable();
        let mut res: i64 = 0;
        for i in (0..n).rev() {
            let diff = if i > 0 {
                inventory[i] - inventory[i - 1]
            } else {
                inventory[i]
            };
            let w = (n - i) as i32;
            if orders >= diff * w {
                res += (inventory[i] - diff + 1 + inventory[i]) as i64 * diff as i64 / 2 * w as i64;
                res %= MOD;
                orders -= diff * w;
            } else {
                let diff = orders / w;
                res += (inventory[i] - diff + 1 + inventory[i]) as i64 * diff as i64 / 2 * w as i64;
                res %= MOD;
                orders -= diff * w;
                let h = inventory[i] - diff;
                while orders > 0 {
                    res += h as i64;
                    res %= MOD;
                    orders -= 1;
                }
                break;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let inventory = vec![2, 5];
    let orders = 4;
    let res = 14;
    assert_eq!(Solution::max_profit(inventory, orders), res);
    let inventory = vec![3, 5];
    let orders = 6;
    let res = 19;
    assert_eq!(Solution::max_profit(inventory, orders), res);
    let inventory = vec![2, 8, 4, 10, 6];
    let orders = 20;
    let res = 110;
    assert_eq!(Solution::max_profit(inventory, orders), res);
    let inventory = vec![2, 8, 4, 10, 6];
    let orders = 20;
    let res = 110;
    assert_eq!(Solution::max_profit(inventory, orders), res);
    let inventory = vec![1000000000];
    let orders = 1000000000;
    let res = 21;
    assert_eq!(Solution::max_profit(inventory, orders), res);
}
