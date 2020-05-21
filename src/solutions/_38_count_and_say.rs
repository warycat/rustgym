struct Solution;

struct Pair {
    digit: char,
    count: usize,
}

impl Solution {
    fn next(nums: String) -> String {
        let mut prev: Option<Pair> = None;
        let mut s = String::from("");
        for c in nums.chars() {
            if let Some(prev_pair) = prev {
                if prev_pair.digit == c {
                    prev = Some(Pair {
                        digit: c,
                        count: prev_pair.count + 1,
                    });
                } else {
                    s.push_str(&prev_pair.count.to_string());
                    s.push_str(&prev_pair.digit.to_string());
                    prev = Some(Pair { digit: c, count: 1 });
                }
            } else {
                prev = Some(Pair { digit: c, count: 1 });
            }
        }
        if let Some(prev_pair) = prev {
            s.push_str(&prev_pair.count.to_string());
            s.push_str(&prev_pair.digit.to_string());
        }
        s
    }

    fn count_and_say(n: i32) -> String {
        match n {
            1 => String::from("1"),
            2..=30 => Self::next(Solution::count_and_say(n - 1)),
            _ => String::from(""),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_and_say(4), String::from("1211"));
    assert_eq!(Solution::count_and_say(1), String::from("1"));
}
