struct Solution;

impl Solution {
    fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 0;
        let mut start = 0;
        for b in s.bytes() {
            let w = widths[(b - b'a') as usize];
            if start + w > 100 {
                lines += 1;
                start = w;
            } else {
                start += w;
            }
        }
        vec![lines + 1, start]
    }
}

#[test]
fn test() {
    let widths = vec![
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10,
    ];
    let s = "abcdefghijklmnopqrstuvwxyz".to_string();
    let res = vec![3, 60];
    assert_eq!(Solution::number_of_lines(widths, s), res);
    let widths = vec![
        4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10,
    ];
    let s = "bbbcccdddaaa".to_string();
    let res = vec![2, 4];
    assert_eq!(Solution::number_of_lines(widths, s), res);
}
