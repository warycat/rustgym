struct Solution;

impl Solution {
    fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pairs: Vec<(i32, i32)> = vec![];
        for trip in trips {
            pairs.push((trip[1], trip[0]));
            pairs.push((trip[2], -trip[0]));
        }
        pairs.sort_unstable();
        let mut max = 0;
        let mut count = 0;
        for pair in pairs {
            count += pair.1;
            max = max.max(count);
        }
        max <= capacity
    }
}

#[test]
fn test() {
    let trips = vec_vec_i32![[2, 1, 5], [3, 3, 7]];
    let capacity = 4;
    let res = false;
    assert_eq!(Solution::car_pooling(trips, capacity), res);
    let trips = vec_vec_i32![[2, 1, 5], [3, 3, 7]];
    let capacity = 5;
    let res = true;
    assert_eq!(Solution::car_pooling(trips, capacity), res);
    let trips = vec_vec_i32![[2, 1, 5], [3, 5, 7]];
    let capacity = 3;
    let res = true;
    assert_eq!(Solution::car_pooling(trips, capacity), res);
    let trips = vec_vec_i32![[3, 2, 7], [3, 7, 9], [8, 3, 9]];
    let capacity = 11;
    let res = true;
    assert_eq!(Solution::car_pooling(trips, capacity), res);
}
