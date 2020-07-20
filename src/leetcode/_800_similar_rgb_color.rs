struct Solution;

use std::i32;

impl Solution {
    fn most_similar(x: i32) -> i32 {
        let mut min = i32::MAX;
        let mut res = 0;
        for i in 0..16 {
            if (x - i * 17).abs() < min {
                min = (x - i * 17).abs();
                res = i * 17;
            }
        }
        res
    }
    fn similar_rgb(color: String) -> String {
        let r = i32::from_str_radix(&color[1..3], 16).unwrap();
        let g = i32::from_str_radix(&color[3..5], 16).unwrap();
        let b = i32::from_str_radix(&color[5..7], 16).unwrap();
        format!(
            "#{:02x}{:02x}{:02x}",
            Self::most_similar(r),
            Self::most_similar(g),
            Self::most_similar(b)
        )
    }
}

#[test]
fn test() {
    let color = "#09f166".to_string();
    let res = "#11ee66".to_string();
    assert_eq!(Solution::similar_rgb(color), res);
}
