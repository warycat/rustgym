struct Solution;

impl Solution {
    fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for s in ops {
            match s.as_ref() {
                "C" => {
                    stack.pop();
                }
                "D" => {
                    let top = stack.pop().unwrap();
                    let double = top * 2;
                    stack.push(top);
                    stack.push(double);
                }
                "+" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let plus = a + b;
                    stack.push(a);
                    stack.push(b);
                    stack.push(plus);
                }
                _ => {
                    stack.push(s.parse::<i32>().unwrap());
                }
            }
        }
        stack.iter().sum()
    }
}

#[test]
fn test() {
    let ops: Vec<String> = vec_string!["5", "2", "C", "D", "+"];
    assert_eq!(Solution::cal_points(ops), 30);
    let ops: Vec<String> = vec_string!["5", "-2", "4", "C", "D", "9", "+", "+"];
    assert_eq!(Solution::cal_points(ops), 27);
}
