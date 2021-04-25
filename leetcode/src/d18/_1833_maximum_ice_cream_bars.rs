struct Solution;

impl Solution {
    fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort_unstable();
        costs.reverse();
        let mut res = 0;
        while coins > 0 {
            if let Some(cheapest) = costs.pop() {
                if cheapest <= coins {
                    coins -= cheapest;
                    res += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let costs = vec![1, 3, 2, 4, 1];
    let coins = 7;
    let res = 4;
    assert_eq!(Solution::max_ice_cream(costs, coins), res);

    let costs = vec![10, 6, 8, 7, 7, 8];
    let coins = 5;
    let res = 0;
    assert_eq!(Solution::max_ice_cream(costs, coins), res);

    let costs = vec![1, 6, 3, 1, 2, 5];
    let coins = 20;
    let res = 6;
    assert_eq!(Solution::max_ice_cream(costs, coins), res);
}
