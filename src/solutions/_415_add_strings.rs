struct Solution;

impl Solution {
    fn add_strings(nums1: String, nums2: String) -> String {
        let s1: Vec<i32> = nums1.bytes().map(|x| (x - b'0') as i32).rev().collect();
        let s2: Vec<i32> = nums2.bytes().map(|x| (x - b'0') as i32).rev().collect();
        let mut carry = 0;
        let mut i = 0;
        let mut s3: Vec<char> = vec![];
        while i < s1.len() || i < s2.len() || carry > 0 {
            let mut v = 0;
            if i < s1.len() {
                v += s1[i];
            }
            if i < s2.len() {
                v += s2[i];
            }
            v += carry;
            carry = v / 10;
            s3.push(((v % 10) as u8 + b'0') as char);
            i += 1;
        }
        let res: String = s3.iter().rev().collect();
        res
    }
}

#[test]
fn test() {
    let nums1 = "0".to_string();
    let nums2 = "0".to_string();
    let sum = "0".to_string();
    assert_eq!(Solution::add_strings(nums1, nums2), sum);
}
