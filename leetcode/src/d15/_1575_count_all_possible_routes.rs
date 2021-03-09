struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let mut memo: HashMap<(usize, i32), i64> = HashMap::new();
        let start = start as usize;
        let finish = finish as usize;
        memo.insert((start, fuel), 1);
        ((0..=fuel)
            .map(|f| Self::dp(finish, f, &mut memo, &locations, fuel, n))
            .sum::<i64>()
            % MOD) as i32
    }

    fn dp(
        i: usize,
        fuel: i32,
        memo: &mut HashMap<(usize, i32), i64>,
        locations: &[i32],
        max_fuel: i32,
        n: usize,
    ) -> i64 {
        if fuel > max_fuel {
            return 0;
        }
        if let Some(&res) = memo.get(&(i, fuel)) {
            return res;
        }
        let mut res = 0;
        for j in 0..n {
            if i != j {
                let cost = (locations[i] - locations[j]).abs();
                res += Self::dp(j, fuel + cost, memo, locations, max_fuel, n);
                res %= MOD;
            }
        }
        memo.insert((i, fuel), res);
        res
    }
}

#[test]
fn test() {
    let locations = vec![2, 3, 6, 8, 4];
    let start = 1;
    let finish = 3;
    let fuel = 5;
    let res = 4;
    assert_eq!(Solution::count_routes(locations, start, finish, fuel), res);
    let locations = vec![4, 3, 1];
    let start = 1;
    let finish = 0;
    let fuel = 6;
    let res = 5;
    assert_eq!(Solution::count_routes(locations, start, finish, fuel), res);
    let locations = vec![5, 2, 1];
    let start = 0;
    let finish = 2;
    let fuel = 3;
    let res = 0;
    assert_eq!(Solution::count_routes(locations, start, finish, fuel), res);
    let locations = vec![2, 1, 5];
    let start = 0;
    let finish = 0;
    let fuel = 3;
    let res = 2;
    assert_eq!(Solution::count_routes(locations, start, finish, fuel), res);
    let locations = vec![1, 2, 3];
    let start = 0;
    let finish = 2;
    let fuel = 40;
    let res = 615088286;
    assert_eq!(Solution::count_routes(locations, start, finish, fuel), res);
}
