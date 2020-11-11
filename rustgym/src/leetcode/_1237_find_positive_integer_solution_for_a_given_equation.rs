struct Solution;

struct CustomFunction {
    fp: fn(i32, i32) -> i32,
}

impl CustomFunction {
    fn f(&self, x: i32, y: i32) -> i32 {
        (self.fp)(x, y)
    }
}

impl Solution {
    fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..1000 {
            for j in 0..1000 {
                let x = (i + 1) as i32;
                let y = (j + 1) as i32;
                if customfunction.f(x, y) == z {
                    res.push(vec![x, y]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let cf = CustomFunction { fp: |x, y| x + y };
    let z = 5;
    let res = vec_vec_i32![[1, 4], [2, 3], [3, 2], [4, 1]];
    assert_eq!(Solution::find_solution(&cf, z), res);
    let cf = CustomFunction { fp: |x, y| x * y };
    let z = 5;
    let res = vec_vec_i32![[1, 5], [5, 1]];
    assert_eq!(Solution::find_solution(&cf, z), res);
}
