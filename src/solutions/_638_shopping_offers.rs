struct Solution;

use std::collections::HashMap;

impl Solution {
    fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();
        let m = special.len();
        let mut memo: HashMap<Vec<i32>, i32> = HashMap::new();
        memo.insert(vec![0; n], 0);
        Self::dfs(needs, &mut memo, &price, &special, n, m)
    }
    fn dfs(
        needs: Vec<i32>,
        memo: &mut HashMap<Vec<i32>, i32>,
        price: &[i32],
        special: &[Vec<i32>],
        n: usize,
        m: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&needs) {
            return res;
        }
        let mut res: i32 = needs.iter().zip(price.iter()).map(|(a, b)| a * b).sum();
        'special: for j in 0..m {
            for i in 0..n {
                if special[j][i] > needs[i] {
                    continue 'special;
                }
            }
            let mut needs = needs.to_vec();
            for i in 0..n {
                needs[i] -= special[j][i];
            }
            res = res.min(Self::dfs(needs, memo, price, special, n, m) + special[j][n]);
        }
        memo.insert(needs, res);
        res
    }
}

#[test]
fn test() {
    let price = vec![2, 5];
    let special = vec_vec_i32![[3, 0, 5], [1, 2, 10]];
    let needs = vec![3, 2];
    let res = 14;
    assert_eq!(Solution::shopping_offers(price, special, needs), res);
    let price = vec![2, 3, 4];
    let special = vec_vec_i32![[1, 1, 0, 4], [2, 2, 1, 9]];
    let needs = vec![1, 2, 1];
    let res = 11;
    assert_eq!(Solution::shopping_offers(price, special, needs), res);
}
