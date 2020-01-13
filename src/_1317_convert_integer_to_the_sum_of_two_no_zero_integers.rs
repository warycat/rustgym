struct Solution;

impl Solution {
    fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut v: Vec<bool> = vec![true; 10000];
        v[0] = false;
        for i in 0..10 {
            for j in 0..10 {
                for k in 0..10 {
                    for l in 0..10 {
                        let x = i * 1000 + j * 100 + k * 10 + l;
                        if i != 0 && (j == 0 || k == 0 || l == 0) {
                            v[x] = false;
                        }
                        if j != 0 && (k == 0 || l == 0) {
                            v[x] = false;
                        }
                        if k != 0 && l == 0 {
                            v[x] = false;
                        }
                    }
                }
            }
        }
        for a in 1..n {
            let b = n - a;
            if v[a as usize] && v[b as usize] {
                return vec![a, b];
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    let n = 2;
    let res = vec![1, 1];
    assert_eq!(Solution::get_no_zero_integers(n), res);
    let n = 11;
    let res = vec![2, 9];
    assert_eq!(Solution::get_no_zero_integers(n), res);
    let n = 10000;
    let res = vec![1, 9999];
    assert_eq!(Solution::get_no_zero_integers(n), res);
    let n = 69;
    let res = vec![1, 68];
    assert_eq!(Solution::get_no_zero_integers(n), res);
    let n = 1010;
    let res = vec![11, 999];
    assert_eq!(Solution::get_no_zero_integers(n), res);
    let n = 4102;
    let res = vec![111, 3991];
    assert_eq!(Solution::get_no_zero_integers(n), res);
}
