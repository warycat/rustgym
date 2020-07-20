struct Solution;

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    fn new(hour: i32, minute: i32) -> Self {
        Time { hour, minute }
    }
    fn is_valid(&self) -> bool {
        self.hour < 24 && self.minute < 60
    }
    fn from_digits(a: &[i32]) -> Self {
        Self::new(a[0] * 10 + a[1], a[2] * 10 + a[3])
    }
    fn to_minutes(&self) -> i32 {
        self.hour * 60 + self.minute
    }
    fn to_digits(&self) -> Vec<i32> {
        vec![
            self.hour / 10,
            self.hour % 10,
            self.minute / 10,
            self.minute % 10,
        ]
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.to_digits();
        write!(f, "{}{}:{}{}", a[0], a[1], a[2], a[3])
    }
}

use std::cmp::Ordering;

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_minutes().cmp(&other.to_minutes()))
    }
}

impl Solution {
    fn backtrack(a: &mut Vec<i32>, index: usize, max: &mut Option<Time>) {
        let n = a.len();
        if index == n {
            let time = Time::from_digits(a);
            if time.is_valid() {
                if let Some(max_time) = max {
                    if time > *max_time {
                        *max = Some(time)
                    }
                } else {
                    *max = Some(time)
                }
            }
        } else {
            for i in index..n {
                a.swap(index, i);
                Self::backtrack(a, index + 1, max);
                a.swap(index, i);
            }
        }
    }
    fn largest_time_from_digits(mut a: Vec<i32>) -> String {
        let mut max: Option<Time> = None;
        Self::backtrack(&mut a, 0, &mut max);
        if let Some(max_time) = max {
            max_time.to_string()
        } else {
            "".to_string()
        }
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 4];
    let res = "23:41".to_string();
    assert_eq!(Solution::largest_time_from_digits(a), res);
    let a = vec![5, 5, 5, 5];
    let res = "".to_string();
    assert_eq!(Solution::largest_time_from_digits(a), res);
    let a = vec![0, 0, 0, 0];
    let res = "00:00".to_string();
    assert_eq!(Solution::largest_time_from_digits(a), res);
}
