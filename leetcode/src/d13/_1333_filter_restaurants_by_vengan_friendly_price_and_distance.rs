struct Solution;
use std::cmp::Reverse;

type Pair = (Reverse<i32>, Reverse<i32>);
impl Solution {
    fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut pairs: Vec<Pair> = restaurants
            .into_iter()
            .filter_map(|r| {
                if (vegan_friendly == 0 || r[2] == 1) && r[3] <= max_price && r[4] <= max_distance {
                    Some((Reverse(r[1]), Reverse(r[0])))
                } else {
                    None
                }
            })
            .collect();
        pairs.sort_unstable();
        pairs.into_iter().map(|p| (p.1).0).collect()
    }
}

#[test]
fn test() {
    let restaurants = vec_vec_i32![
        [1, 4, 1, 40, 10],
        [2, 8, 0, 50, 5],
        [3, 8, 1, 30, 4],
        [4, 10, 0, 10, 3],
        [5, 1, 1, 15, 1]
    ];
    let vegan_friendly = 1;
    let max_price = 50;
    let max_distance = 10;
    let res = vec![3, 1, 5];
    assert_eq!(
        Solution::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance),
        res
    );
    let restaurants = vec_vec_i32![
        [1, 4, 1, 40, 10],
        [2, 8, 0, 50, 5],
        [3, 8, 1, 30, 4],
        [4, 10, 0, 10, 3],
        [5, 1, 1, 15, 1]
    ];
    let vegan_friendly = 0;
    let max_price = 50;
    let max_distance = 10;
    let res = vec![4, 3, 2, 1, 5];
    assert_eq!(
        Solution::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance),
        res
    );
    let restaurants = vec_vec_i32![
        [1, 4, 1, 40, 10],
        [2, 8, 0, 50, 5],
        [3, 8, 1, 30, 4],
        [4, 10, 0, 10, 3],
        [5, 1, 1, 15, 1]
    ];
    let vegan_friendly = 0;
    let max_price = 30;
    let max_distance = 3;
    let res = vec![4, 5];
    assert_eq!(
        Solution::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance),
        res
    );
}
