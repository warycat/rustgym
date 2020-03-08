struct Solution;

impl Solution {
    fn construct_array(n: i32, mut k: i32) -> Vec<i32> {
        let mut res = vec![1];
        let mut l = 2;
        let mut r = n;
        let mut forward = true;
        for _ in 1..n {
            if k > 1 {
                if forward {
                    res.push(r);
                    r -= 1;
                } else {
                    res.push(l);
                    l += 1;
                }
                forward = !forward;
                k -= 1;
            } else {
                if forward {
                    res.push(l);
                    l += 1;
                } else {
                    res.push(r);
                    r -= 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 1;
    let res = vec![1, 2, 3];
    assert_eq!(Solution::construct_array(n, k), res);
    let n = 3;
    let k = 2;
    let res = vec![1, 3, 2];
    assert_eq!(Solution::construct_array(n, k), res);
    let n = 5;
    let k = 2;
    let res = vec![1, 5, 4, 3, 2];
    assert_eq!(Solution::construct_array(n, k), res);
}
