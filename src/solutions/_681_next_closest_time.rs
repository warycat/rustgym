struct Solution;

impl Solution {
    fn next_closest_time(time: String) -> String {
        let h = time[..2].parse::<usize>().unwrap();
        let m = time[3..].parse::<usize>().unwrap();
        let a = h / 10;
        let b = h % 10;
        let c = m / 10;
        let d = m % 10;
        let set = 1 << a | 1 << b | 1 << c | 1 << d;
        let mut found = false;
        for _ in 0..2 {
            for i in 0..24 {
                for j in 0..60 {
                    if !found {
                        if i == h && j == m {
                            found = true;
                        }
                    } else {
                        let a = i / 10;
                        let b = i % 10;
                        let c = j / 10;
                        let d = j % 10;
                        let next_set = 1 << a | 1 << b | 1 << c | 1 << d;
                        if (next_set | set) == set {
                            return format!("{}{}:{}{}", a, b, c, d);
                        }
                    }
                }
            }
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let time = "19:34".to_string();
    let res = "19:39".to_string();
    assert_eq!(Solution::next_closest_time(time), res);
    let time = "23:59".to_string();
    let res = "22:22".to_string();
    assert_eq!(Solution::next_closest_time(time), res);
}
