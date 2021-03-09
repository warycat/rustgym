struct Solution;

impl Solution {
    fn maximum_time(time: String) -> String {
        let time: Vec<char> = time.chars().collect();
        let mut max = 0;
        for i in 0..24 {
            'outer: for j in 0..60 {
                let t = i * 60 + j;
                let s = format!("{:02}:{:02}", i, j);
                let s: Vec<char> = s.chars().collect();
                for i in 0..5 {
                    if time[i] != '?' && time[i] != s[i] {
                        continue 'outer;
                    }
                }
                max = max.max(t);
            }
        }
        format!("{:02}:{:02}", max / 60, max % 60)
    }
}

#[test]
fn test() {
    let time = "2?:?0".to_string();
    let res = "23:50".to_string();
    assert_eq!(Solution::maximum_time(time), res);
    let time = "0?:3?".to_string();
    let res = "09:39".to_string();
    assert_eq!(Solution::maximum_time(time), res);
    let time = "1?:22".to_string();
    let res = "19:22".to_string();
    assert_eq!(Solution::maximum_time(time), res);
}
