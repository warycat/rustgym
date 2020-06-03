struct Solution;

impl Solution {
    fn verify_preorder(preorder: Vec<i32>) -> bool {
        let mut low = std::i32::MIN;
        let mut stack: Vec<i32> = vec![];
        for x in preorder {
            if x < low {
                return false;
            }
            while let Some(&top) = stack.last() {
                if top < x {
                    stack.pop();
                    low = top;
                } else {
                    break;
                }
            }
            stack.push(x);
        }
        true
    }
}

#[test]
fn test() {
    let preorder = vec![5, 2, 6, 1, 3];
    let res = false;
    assert_eq!(Solution::verify_preorder(preorder), res);
    let preorder = vec![5, 2, 1, 3, 6];
    let res = true;
    assert_eq!(Solution::verify_preorder(preorder), res);
}
