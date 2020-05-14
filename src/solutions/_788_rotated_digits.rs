struct Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum D {
    Invalid,
    Same,
    Different,
}

impl D {
    fn new(d: usize) -> Self {
        match d {
            0 | 1 | 8 => D::Same,
            2 | 5 | 6 | 9 => D::Different,
            _ => D::Invalid,
        }
    }
}

impl Solution {
    fn rotated_digits(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut a: Vec<D> = vec![D::Invalid; (n + 1) as usize];
        for i in 0..=n {
            if i < 10 {
                a[i] = D::new(i);
            } else {
                let j = i / 10;
                let k = i % 10;
                a[i] = match (a[j], a[k]) {
                    (D::Invalid, _) => D::Invalid,
                    (_, D::Invalid) => D::Invalid,
                    (D::Different, _) => D::Different,
                    (_, D::Different) => D::Different,
                    (D::Same, D::Same) => D::Same,
                }
            }
        }
        a.iter()
            .fold(0, |sum, &d| if d == D::Different { sum + 1 } else { sum })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::rotated_digits(10), 4);
}
