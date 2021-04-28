struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut res: i64 = 0;
        let mut buy: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut sell: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        for order in orders {
            let price = order[0];
            let mut amount = order[1];
            res += amount as i64;
            if order[2] == 0 {
                while let Some(&(Reverse(p_sell), a_sell)) = sell.peek() {
                    if p_sell <= price {
                        sell.pop();
                        let a_min = a_sell.min(amount);
                        let a_left = a_sell - a_min;
                        amount -= a_min;
                        res -= 2 * a_min as i64;
                        if a_left > 0 {
                            sell.push((Reverse(p_sell), a_left));
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if amount > 0 {
                    buy.push((price, amount));
                }
            } else {
                while let Some(&(p_buy, a_buy)) = buy.peek() {
                    if p_buy >= price {
                        buy.pop();
                        let a_min = a_buy.min(amount);
                        let a_left = a_buy - a_min;
                        amount -= a_min;
                        res -= 2 * a_min as i64;
                        if a_left > 0 {
                            buy.push((p_buy, a_left));
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if amount > 0 {
                    sell.push((Reverse(price), amount));
                }
            }
        }
        (res % MOD) as i32
    }
}

#[test]
fn test() {
    let orders = vec_vec_i32![[10, 5, 0], [15, 2, 1], [25, 1, 1], [30, 4, 0]];
    let res = 6;
    assert_eq!(Solution::get_number_of_backlog_orders(orders), res);
    let orders = vec_vec_i32![[7, 1000000000, 1], [15, 3, 0], [5, 999999995, 0], [5, 1, 1]];
    let res = 999999984;
    assert_eq!(Solution::get_number_of_backlog_orders(orders), res);
}
