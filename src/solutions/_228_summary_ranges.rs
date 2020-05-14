struct Solution;
use std::fmt;

struct Range {
    start: i32,
    end: i32,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}->{}", self.start, self.end)
        }
    }
}

impl Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut r: Option<Range> = None;
        let mut res: Vec<String> = vec![];
        for x in nums {
            if let Some(prev) = r {
                if prev.end + 1 == x {
                    r = Some(Range {
                        start: prev.start,
                        end: x,
                    });
                    continue;
                } else {
                    res.push(format!("{}", prev));
                    r = Some(Range { start: x, end: x });
                }
            } else {
                r = Some(Range { start: x, end: x })
            }
        }
        if let Some(last) = r {
            res.push(format!("{}", last));
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    let res: Vec<String> = vec_string!["0->2", "4->5", "7"];
    assert_eq!(Solution::summary_ranges(nums), res);
}
