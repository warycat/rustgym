struct Solution;

impl Solution {
    fn reformat_number(number: String) -> String {
        let nums: Vec<char> = number
            .chars()
            .filter(|&c| ('0'..='9').contains(&c))
            .collect();
        let mut res = "".to_string();
        let n = nums.len();
        let mut i = 0;
        while n - i > 4 {
            res.push(nums[i]);
            i += 1;
            res.push(nums[i]);
            i += 1;
            res.push(nums[i]);
            i += 1;
            res.push('-');
        }
        if n - i == 4 {
            res.push(nums[i]);
            i += 1;
            res.push(nums[i]);
            i += 1;
            res.push('-');
            res.push(nums[i]);
            i += 1;
            res.push(nums[i]);
        } else {
            while n - i > 0 {
                res.push(nums[i]);
                i += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let number = "1-23-45 6".to_string();
    let res = "123-456".to_string();
    assert_eq!(Solution::reformat_number(number), res);
    let number = "123 4-567".to_string();
    let res = "123-45-67".to_string();
    assert_eq!(Solution::reformat_number(number), res);
    let number = "123 4-5678".to_string();
    let res = "123-456-78".to_string();
    assert_eq!(Solution::reformat_number(number), res);
    let number = "12".to_string();
    let res = "12".to_string();
    assert_eq!(Solution::reformat_number(number), res);
    let number = "--17-5 229 35-39475 ".to_string();
    let res = "175-229-353-94-75".to_string();
    assert_eq!(Solution::reformat_number(number), res);
}
