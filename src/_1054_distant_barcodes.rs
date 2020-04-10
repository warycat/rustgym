struct Solution;
use std::collections::HashMap;

impl Solution {
    fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let n = barcodes.len();
        if n == 1 {
            return barcodes;
        }
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut max: (usize, i32) = (0, 0);
        for barcode in barcodes {
            let count = hm.entry(barcode).or_default();
            *count += 1;
            if *count > max.0 {
                max = (*count, barcode);
            }
        }
        let mut stack = vec![];
        for (k, v) in hm {
            if k != max.1 {
                for _ in 0..v {
                    stack.push(k);
                }
            }
        }
        for _ in 0..max.0 {
            stack.push(max.1);
        }
        let mut res = vec![0; n];
        let m = if n % 2 == 0 { n / 2 } else { (n + 1) / 2 };
        for i in 0..m {
            res[i * 2] = stack.pop().unwrap();
        }
        let mut i = 1;
        while let Some(top) = stack.pop() {
            res[i] = top;
            i += 2;
        }
        res
    }
}

#[test]
fn test() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let res = vec![1, 2, 1, 2, 1, 2];
    assert_eq!(Solution::rearrange_barcodes(barcodes), res);
    let barcodes = vec![1, 1, 2];
    let res = vec![1, 2, 1];
    assert_eq!(Solution::rearrange_barcodes(barcodes), res);
}
