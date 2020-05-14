struct Solution;

impl Solution {
    fn max_number_of_apples(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let n = arr.len();
        let mut sum = 0;
        for i in 0..n {
            sum += arr[i];
            if sum > 5000 {
                return i as i32;
            }
        }
        n as i32
    }
}

#[test]
fn test() {
    let arr = vec![100, 200, 150, 1000];
    assert_eq!(Solution::max_number_of_apples(arr), 4);
    let arr = vec![900, 950, 800, 1000, 700, 800];
    assert_eq!(Solution::max_number_of_apples(arr), 5);
}
