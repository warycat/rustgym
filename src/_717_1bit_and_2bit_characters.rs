struct Solution;

impl Solution {
    fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        let n = bits.len();
        let mut one_bit: Option<bool> = None;
        while i < n {
            if bits[i] == 1 {
                i += 2;
                one_bit = Some(false);
            } else {
                i += 1;
                one_bit = Some(true);
            }
        }
        if let Some(res) = one_bit {
            res
        } else {
            false
        }
    }
}

#[test]
fn test() {
    let bits = vec![1, 0, 0];
    assert_eq!(Solution::is_one_bit_character(bits), true);
    let bits = vec![1, 1, 0, 0];
    assert_eq!(Solution::is_one_bit_character(bits), true);
}
