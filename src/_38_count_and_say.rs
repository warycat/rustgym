struct Solution;

struct Pair {
    pub digit: char,
    pub count: usize,
}

impl Solution {
    fn count_and_say(n: i32) -> String {
        fn next(nums: String) -> String {
            let mut pairs: Vec<Pair> = vec![];
            for c in nums.chars() {
                if let Some(p) = pairs.last_mut() {
                    if p.digit == c {
                        p.count = p.count + 1;
                    } else {
                        pairs.push(Pair { digit: c, count: 1 });
                    }
                } else {
                    pairs.push(Pair { digit: c, count: 1 });
                }
            }
            let mut s = String::from("");
            for p in pairs {
                s.push_str(&p.count.to_string());
                s.push_str(&p.digit.to_string());
            }
            s
        }
        match n {
            1 => String::from("1"),
            2...30 => next(Solution::count_and_say(n - 1)),
            _ => String::from(""),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_and_say(4), String::from("1211"));
    assert_eq!(Solution::count_and_say(1), String::from("1"));
}
