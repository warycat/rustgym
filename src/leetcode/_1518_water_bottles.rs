struct Solution;

impl Solution {
    fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut full = num_bottles;
        let mut empty = 0;
        let mut res = 0;
        while full > 0 {
            res += full;
            empty += full;
            full = empty / num_exchange;
            empty %= num_exchange;
        }
        res
    }
}

#[test]
fn test() {
    let num_bottles = 9;
    let num_exchange = 3;
    let res = 13;
    assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
    let num_bottles = 15;
    let num_exchange = 4;
    let res = 19;
    assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
    let num_bottles = 15;
    let num_exchange = 4;
    let res = 19;
    assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
    let num_bottles = 5;
    let num_exchange = 5;
    let res = 6;
    assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
    let num_bottles = 2;
    let num_exchange = 3;
    let res = 2;
    assert_eq!(Solution::num_water_bottles(num_bottles, num_exchange), res);
}
