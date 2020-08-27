struct Solution;

impl Solution {
    fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let mut pigs = 0;
        let t = minutes_to_test / minutes_to_die + 1;
        while t.pow(pigs) < buckets {
            pigs += 1;
        }
        pigs as i32
    }
}

#[test]
fn test() {
    let buckets = 1000;
    let minutes_to_test = 60;
    let minutes_to_die = 15;
    let res = 5;
    assert_eq!(
        Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
        res
    );
}
