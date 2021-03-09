struct Solution;

impl Solution {
    fn nearest_palindromic(n: String) -> String {
        let size = n.len();
        let half = size / 2;
        let value = n.parse::<i64>().unwrap();
        let a = 10i64.pow(size as u32);
        let b = 10i64.pow(size as u32 - 1);
        let mut candidates = vec![a - 1, a + 1, b - 1, b + 1];
        if size % 2 == 0 {
            let left = n[..half].parse::<i64>().unwrap();
            candidates.push(Self::combine(left - 1, false));
            candidates.push(Self::combine(left, false));
            candidates.push(Self::combine(left + 1, false));
        } else {
            let left = n[..half + 1].parse::<i64>().unwrap();
            candidates.push(Self::combine(left - 1, true));
            candidates.push(Self::combine(left, true));
            candidates.push(Self::combine(left + 1, true));
        }
        let mut res = (std::i64::MAX, 0);
        for x in candidates {
            let diff = (value - x).abs();
            if diff == 0 {
                continue;
            }
            if (diff, x) < res {
                res = (diff, x);
            }
        }
        (res.1).to_string()
    }

    fn combine(left: i64, odd: bool) -> i64 {
        let l = left.to_string();
        l.chars()
            .chain(l.chars().rev().skip(if odd { 1 } else { 0 }))
            .collect::<String>()
            .parse::<i64>()
            .unwrap()
    }
}

#[test]
fn test() {
    let n = "123".to_string();
    let res = "121".to_string();
    assert_eq!(Solution::nearest_palindromic(n), res);
    let n = "8".to_string();
    let res = "7".to_string();
    assert_eq!(Solution::nearest_palindromic(n), res);
    let n = "10".to_string();
    let res = "9".to_string();
    assert_eq!(Solution::nearest_palindromic(n), res);
    let n = "11".to_string();
    let res = "9".to_string();
    assert_eq!(Solution::nearest_palindromic(n), res);
}
