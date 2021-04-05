struct Solution;

impl Solution {
    fn square_is_white(coordinates: String) -> bool {
        let mut arr: Vec<u8> = coordinates.bytes().collect();
        arr[0] -= b'a';
        arr[1] -= b'1';
        (arr[0] % 2 == 0) ^ (arr[1] % 2 == 0)
    }
}

#[test]
fn test() {
    let coordinates = "a1".to_string();
    let res = false;
    assert_eq!(Solution::square_is_white(coordinates), res);
    let coordinates = "h3".to_string();
    let res = true;
    assert_eq!(Solution::square_is_white(coordinates), res);
    let coordinates = "c7".to_string();
    let res = false;
    assert_eq!(Solution::square_is_white(coordinates), res);
}
