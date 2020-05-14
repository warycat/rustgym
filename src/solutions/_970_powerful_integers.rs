struct Solution;

impl Solution {
    fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set: Vec<bool> = vec![false; bound as usize + 1];
        let mut i = 0;
        while x.pow(i) < bound {
            let mut j = 0;
            while y.pow(j) < bound {
                let sum = x.pow(i) + y.pow(j);
                if sum <= bound {
                    set[sum as usize] = true;
                }
                j += 1;
                if y == 1 {
                    break;
                }
            }
            i += 1;
            if x == 1 {
                break;
            }
        }
        let mut res = vec![];
        for i in 0..=bound {
            if set[i as usize] {
                res.push(i);
            }
        }
        res
    }
}

#[test]
fn test() {
    let x = 2;
    let y = 3;
    let bound = 10;
    let res = vec![2, 3, 4, 5, 7, 9, 10];
    assert_eq!(Solution::powerful_integers(x, y, bound), res);
}
