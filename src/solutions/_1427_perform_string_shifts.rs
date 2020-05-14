struct Solution;

impl Solution {
    fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let v: Vec<char> = s.chars().collect();
        let mut res = "".to_string();
        let n = s.len();
        let mut first = 0;
        for x in shift {
            let direction = x[0];
            let amount = x[1];
            if direction == 0 {
                first += amount;
            } else {
                first -= amount;
            }
            if first < 0 {
                first += n as i32;
            }
            first %= n as i32;
        }
        for i in 0..n {
            res.push(v[(first as usize + i) % n]);
        }
        res
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    let shift = vec_vec_i32![[0, 1], [1, 2]];
    let res = "cab".to_string();
    assert_eq!(Solution::string_shift(s, shift), res);
    let s = "abcdefg".to_string();
    let shift = vec_vec_i32![[1, 1], [1, 1], [0, 2], [1, 3]];
    let res = "efgabcd".to_string();
    assert_eq!(Solution::string_shift(s, shift), res);
}
