struct Solution;
use std::fmt;

struct Interval(i64, i64);

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}->{}", self.0, self.1)
        }
    }
}

impl Solution {
    fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let mut cur = Interval(lower as i64, upper as i64);
        let mut res: Vec<Interval> = vec![];
        for x in nums {
            let x = x as i64;
            if x > cur.0 {
                res.push(Interval(cur.0, x - 1));
            }
            cur.0 = x + 1;
        }
        if cur.0 <= cur.1 {
            res.push(cur);
        }
        res.iter().map(|x| x.to_string()).collect()
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 3, 50, 75];
    let lower = 0;
    let upper = 99;
    let res = vec_string!["2", "4->49", "51->74", "76->99"];
    assert_eq!(Solution::find_missing_ranges(nums, lower, upper), res);
    let nums = vec![2_147_483_647];
    let lower = 0;
    let upper = 2_147_483_647;
    let res = vec_string!["0->2147483646"];
    assert_eq!(Solution::find_missing_ranges(nums, lower, upper), res);
}
