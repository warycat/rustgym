struct Solution;

impl Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut slice = vec![];
        let mut res = 0;
        for x in a {
            if slice.len() < 2 {
                slice.push(x);
            } else {
                let y = slice.pop().unwrap();
                let z = slice.pop().unwrap();
                if y - z == x - y {
                    slice.push(z);
                    slice.push(y);
                    slice.push(x);
                    res += slice.len() - 2;
                } else {
                    slice.clear();
                    slice.push(y);
                    slice.push(x);
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 4];
    let res = 3;
    assert_eq!(Solution::number_of_arithmetic_slices(a), res);
}
