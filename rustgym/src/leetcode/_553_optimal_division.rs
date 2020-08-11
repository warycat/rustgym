struct Solution;

impl Solution {
    fn optimal_division(nums: Vec<i32>) -> String {
        let n = nums.len();
        if n == 1 {
            format!("{}", nums[0])
        } else if n == 2 {
            format!("{}/{}", nums[0], nums[1])
        } else {
            let mid: String = (1..n)
                .map(|i| nums[i].to_string())
                .collect::<Vec<String>>()
                .join("/");
            format!("{}/({})", nums[0], mid)
        }
    }
}

#[test]
fn test() {
    let nums = vec![1000, 100, 10, 2];
    let res = "1000/(100/10/2)".to_string();
    assert_eq!(Solution::optimal_division(nums), res);
}
