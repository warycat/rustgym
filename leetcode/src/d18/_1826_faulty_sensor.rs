struct Solution;

impl Solution {
    fn bad_sensor(sensor1: Vec<i32>, sensor2: Vec<i32>) -> i32 {
        let p1 = bad(&sensor1, &sensor2);
        let p2 = bad(&sensor2, &sensor1);
        if p1 && !p2 {
            return 1;
        }
        if !p1 && p2 {
            return 2;
        }
        -1
    }
}

fn bad(s1: &[i32], s2: &[i32]) -> bool {
    let n1 = s1.len();
    let n2 = s2.len();
    if n1 != n2 {
        return false;
    }
    let mut i = 0;
    while i < n1 {
        if s1[i] != s2[i] {
            break;
        }
        i += 1;
    }
    if i == n1 {
        return false;
    }
    if s1[i..n1 - 1] != s2[i + 1..n2] || s1[n1 - 1] == s2[i] {
        return false;
    }

    true
}

#[test]
fn test() {
    let sensor1 = vec![2, 3, 4, 5];
    let sensor2 = vec![2, 1, 3, 4];
    let res = 1;
    assert_eq!(Solution::bad_sensor(sensor1, sensor2), res);
    let sensor1 = vec![2, 2, 2, 2, 2];
    let sensor2 = vec![2, 2, 2, 2, 5];
    let res = -1;
    assert_eq!(Solution::bad_sensor(sensor1, sensor2), res);
    let sensor1 = vec![2, 3, 2, 2, 3, 2];
    let sensor2 = vec![2, 3, 2, 3, 2, 7];
    let res = 2;
    assert_eq!(Solution::bad_sensor(sensor1, sensor2), res);
}
