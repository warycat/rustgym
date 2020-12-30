struct Solution;

impl Solution {
    fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;
        for i in 0..32 {
            let bit = x & 1 << i;
            let travel = (31 - i as i32) - i as i32;
            if travel > 0 {
                res |= bit << travel;
            } else {
                res |= bit >> -travel;
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 0b00000010100101000001111010011100;
    let res = 964176192;
    assert_eq!(Solution::reverse_bits(n), res);
    let n = 0b11111111111111111111111111111101;
    let res = 3221225471;
    assert_eq!(Solution::reverse_bits(n), res);
}
