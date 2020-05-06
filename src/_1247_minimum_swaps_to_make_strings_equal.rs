struct Solution;

impl Solution {
    fn minimum_swap(s1: String, s2: String) -> i32 {
        let n = s1.len();
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut x = 0;
        let mut y = 0;
        for i in 0..n {
            if s1[i] == 'x' && s2[i] == 'y' {
                x += 1;
            }
            if s1[i] == 'y' && s2[i] == 'x' {
                y += 1;
            }
        }
        if (x + y) % 2 != 0 {
            return -1;
        }
        x / 2 + y / 2 + x % 2 * 2
    }
}

#[test]
fn test() {
    let s1 = "xx".to_string();
    let s2 = "yy".to_string();
    let res = 1;
    assert_eq!(Solution::minimum_swap(s1, s2), res);
    let s1 = "xy".to_string();
    let s2 = "yx".to_string();
    let res = 2;
    assert_eq!(Solution::minimum_swap(s1, s2), res);
    let s1 = "xx".to_string();
    let s2 = "xy".to_string();
    let res = -1;
    assert_eq!(Solution::minimum_swap(s1, s2), res);
    let s1 = "xxyyxyxyxx".to_string();
    let s2 = "xyyxyxxxyx".to_string();
    let res = 4;
    assert_eq!(Solution::minimum_swap(s1, s2), res);
}
