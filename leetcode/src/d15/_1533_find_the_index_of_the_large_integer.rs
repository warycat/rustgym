struct Solution;

use std::cmp::Ordering::*;

struct ArrayReader {
    n: usize,
    small: i32,
    diff: i32,
    index: usize,
}

impl ArrayReader {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let small = arr[0].min(arr[1]);
        let mut diff = 0;
        let mut index = 0;
        for i in 0..n {
            if arr[i] > small {
                diff = arr[i] - small;
                index = i;
                break;
            }
        }
        ArrayReader {
            n,
            small,
            diff,
            index,
        }
    }

    #[allow(non_snake_case)]
    fn compareSub(&self, l: i32, r: i32, x: i32, y: i32) -> i32 {
        let i = self.index as i32;
        let s1 = (r - l + 1) * self.small + if l <= i && i <= r { self.diff } else { 0 };
        let s2 = (y - x + 1) * self.small + if x <= i && i <= y { self.diff } else { 0 };
        match s1.cmp(&s2) {
            Equal => 0,
            Greater => 1,
            Less => -1,
        }
    }

    fn length(&self) -> i32 {
        self.n as i32
    }
}

impl Solution {
    fn get_index(reader: &ArrayReader) -> i32 {
        let n = reader.length();
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            let w = r - l + 1;
            let m = l + w / 2;
            if w % 2 == 0 {
                let cmp = reader.compareSub(l, m - 1, m, r);
                if cmp == 1 {
                    r = m - 1;
                } else {
                    l = m;
                }
            } else {
                let cmp = reader.compareSub(l, m - 1, m + 1, r);
                if cmp == 0 {
                    return m;
                }
                if cmp == 1 {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            }
        }
        l
    }
}

#[test]
fn test() {
    // let arr = vec![7, 7, 7, 7, 10, 7, 7, 7];
    // let reader = ArrayReader::new(arr);
    // let res = 4;
    // assert_eq!(Solution::get_index(&reader), res);
    // let arr = vec![6, 6, 12];
    // let reader = ArrayReader::new(arr);
    // let res = 2;
    // assert_eq!(Solution::get_index(&reader), res);
    let arr = vec![1, 1, 1, 1, 1, 1, 2, 1, 1];
    let reader = ArrayReader::new(arr);
    let res = 6;
    assert_eq!(Solution::get_index(&reader), res);
}
