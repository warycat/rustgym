struct Solution;

impl Solution {
    fn count_quadruples(first_string: String, second_string: String) -> i32 {
        let first_string: Vec<u8> = first_string.bytes().collect();
        let second_string: Vec<u8> = second_string.bytes().collect();
        let n = first_string.len();
        let m = second_string.len();
        let mut min = std::i32::MAX;
        for j in 0..26 {
            let left = first_string
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| {
                    if b == b'a' + j as u8 {
                        Some(i as i32)
                    } else {
                        None
                    }
                })
                .min();
            let right = second_string
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| {
                    if b == b'a' + j as u8 {
                        Some(i as i32)
                    } else {
                        None
                    }
                })
                .max();
            if let (Some(l), Some(r)) = (left, right) {
                min = min.min(l - r);
            }
        }
        let mut res = 0;
        for i in 0..n {
            let a = first_string[i];
            let j = i as i32 - min;
            if j < m as i32 && j >= 0 {
                let b = second_string[j as usize];
                if a == b {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let first_string = "abcd".to_string();
    let second_string = "bccda".to_string();
    let res = 1;
    assert_eq!(Solution::count_quadruples(first_string, second_string), res);
    let first_string = "ab".to_string();
    let second_string = "cd".to_string();
    let res = 0;
    assert_eq!(Solution::count_quadruples(first_string, second_string), res);
    let first_string = "xznhzmyk".to_string();
    let second_string = "mxznzn".to_string();
    let res = 2;
    assert_eq!(Solution::count_quadruples(first_string, second_string), res);
}
