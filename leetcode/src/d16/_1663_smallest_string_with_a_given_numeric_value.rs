struct Solution;

impl Solution {
    fn get_smallest_string(n: i32, k: i32) -> String {
        let mut k = k - n;
        let n = n as usize;
        let mut arr: Vec<u8> = vec![b'a'; n];
        for i in (0..n).rev() {
            if k > 25 {
                k -= 25;
                arr[i] = b'z';
            } else if k > 0 {
                arr[i] += k as u8;
                k -= k;
            } else {
                break;
            }
        }
        arr.into_iter().map(|b| b as char).collect()
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 27;
    let res = "aay".to_string();
    assert_eq!(Solution::get_smallest_string(n, k), res);
    let n = 5;
    let k = 73;
    let res = "aaszz".to_string();
    assert_eq!(Solution::get_smallest_string(n, k), res);
}
