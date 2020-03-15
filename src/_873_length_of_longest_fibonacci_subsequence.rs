struct Solution;
use std::collections::HashSet;

impl Solution {
    fn len_longest_fib_subseq(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut hs: HashSet<i32> = HashSet::new();
        for &x in &a {
            hs.insert(x);
        }
        let mut max = 0;
        let last = a[n - 1];
        for i in 0..n {
            for j in i + 1..n {
                let mut small = a[i];
                let mut big = a[j];
                let mut sum = small + big;
                let mut size = 0;
                if big * max > last {
                    break;
                }
                while hs.contains(&sum) {
                    small = big;
                    big = sum;
                    sum = small + big;
                    size += 1;
                }
                max = max.max(size);
            }
        }
        if max > 0 {
            (max + 2) as i32
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let res = 5;
    assert_eq!(Solution::len_longest_fib_subseq(a), res);
    let a = vec![1, 3, 7, 11, 12, 14, 18];
    let res = 3;
    assert_eq!(Solution::len_longest_fib_subseq(a), res);
}
