struct Solution;

impl Solution {
    fn minimum_one_bit_operations(n: i32) -> i32 {
        if n <= 1 {
            n
        } else {
            let mut bit = 0;
            while 1 << bit <= n {
                bit += 1;
            }
            let m = n ^ (1 << (bit - 1));
            let k = (1 << bit) - 1;
            k - Solution::minimum_one_bit_operations(m)
        }
    }
}

#[test]
fn test() {
    let n = 0;
    let res = 0;
    assert_eq!(Solution::minimum_one_bit_operations(n), res);
    let n = 3;
    let res = 2;
    assert_eq!(Solution::minimum_one_bit_operations(n), res);
    let n = 6;
    let res = 4;
    assert_eq!(Solution::minimum_one_bit_operations(n), res);
}
