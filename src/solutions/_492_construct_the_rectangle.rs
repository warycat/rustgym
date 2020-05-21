struct Solution;

impl Solution {
    fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut max = (area as f64).sqrt().floor() as i32;
        while area % max != 0 {
            max -= 1;
        }
        vec![area / max, max]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
}
