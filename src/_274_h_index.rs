struct Solution;

impl Solution {
    fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut count: Vec<usize> = vec![0; n + 1];
        for c in citations {
            let i = c as usize;
            if i < n {
                count[i] += 1;
            } else {
                count[n] += 1;
            }
        }
        let mut h = n;
        let mut sum = 0;
        while sum <= h {
            sum += count[h];
            if sum >= h {
                return h as i32;
            }
            h -= 1;
        }
        0
    }
}

#[test]
fn test() {
    let citations = vec![3, 0, 6, 1, 5];
    let res = 3;
    assert_eq!(Solution::h_index(citations), res);
}
