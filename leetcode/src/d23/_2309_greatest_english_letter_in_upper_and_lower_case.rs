struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut chars = vec![false; 58];

        s.bytes().for_each(|c| {
            chars[(c - b'A') as usize] = true;
        });

        for i in (0..26).rev() {
            if chars[i] && chars[i + 32] {
                return ((i as u8 + b'A') as char).to_string();
            }
        }

        "".to_string()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::greatest_letter("lEeTcOdE".to_string()), "E");
    assert_eq!(Solution::greatest_letter("arRAzFif".to_string()), "R");
    assert_eq!(Solution::greatest_letter("AbCdEfGhIjK".to_string()), "");
}
