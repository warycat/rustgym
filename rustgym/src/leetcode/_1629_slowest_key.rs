struct Solution;

impl Solution {
    fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut res = (0, ' ');
        let mut prev = 0;
        for (i, c) in keys_pressed.char_indices() {
            res = res.max((release_times[i] - prev, c));
            prev = release_times[i];
        }
        res.1
    }
}

#[test]
fn test() {
    let release_times = vec![9, 29, 49, 50];
    let keys_pressed = "cbcd".to_string();
    let res = 'c';
    assert_eq!(Solution::slowest_key(release_times, keys_pressed), res);
    let release_times = vec![12, 23, 36, 46, 62];
    let keys_pressed = "spuda".to_string();
    let res = 'a';
    assert_eq!(Solution::slowest_key(release_times, keys_pressed), res);
}
