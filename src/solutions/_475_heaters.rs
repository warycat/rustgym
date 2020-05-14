struct Solution;

impl Solution {
    fn distance(a: i32, b: i32) -> i32 {
        (a - b).abs()
    }
    fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort_unstable();
        heaters.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let n = houses.len();
        let m = heaters.len();
        let mut max = 0;
        while i < n {
            while j + 1 < m
                && Self::distance(heaters[j + 1], houses[i])
                    <= Self::distance(heaters[j], houses[i])
            {
                j += 1;
            }
            max = i32::max(Self::distance(houses[i], heaters[j]), max);
            i += 1;
        }
        max
    }
}

#[test]
fn test() {
    let houses = vec![1, 2, 3];
    let heaters = vec![2];
    assert_eq!(Solution::find_radius(houses, heaters), 1);
    let houses = vec![1, 2, 3, 4];
    let heaters = vec![1, 4];
    assert_eq!(Solution::find_radius(houses, heaters), 1);
}
