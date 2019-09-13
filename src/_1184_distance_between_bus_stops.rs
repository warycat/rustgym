struct Solution;

impl Solution {
    fn distance_between_bus_stops(distance: Vec<i32>, mut start: i32, mut destination: i32) -> i32 {
        let n: usize = distance.len();
        let mut left = 0;
        let mut right = 0;
        if destination < start {
            std::mem::swap(&mut start, &mut destination);
        }
        for i in 0..n {
            let j = start as usize + i;
            if j < destination as usize {
                left += distance[j % n];
            } else {
                right += distance[j % n];
            }
        }
        i32::min(left, right)
    }
}

#[test]
fn test() {
    let distance = vec![1, 2, 3, 4];
    let start = 0;
    let destination = 1;
    let res = 1;
    assert_eq!(
        Solution::distance_between_bus_stops(distance, start, destination),
        res
    );
    let distance = vec![1, 2, 3, 4];
    let start = 0;
    let destination = 1;
    let res = 1;
    assert_eq!(
        Solution::distance_between_bus_stops(distance, start, destination),
        res
    );
    let distance = vec![1, 2, 3, 4];
    let start = 0;
    let destination = 2;
    let res = 3;
    assert_eq!(
        Solution::distance_between_bus_stops(distance, start, destination),
        res
    );
    let distance = vec![1, 2, 3, 4];
    let start = 0;
    let destination = 3;
    let res = 4;
    assert_eq!(
        Solution::distance_between_bus_stops(distance, start, destination),
        res
    );
    let distance = vec![3, 6, 7, 2, 9, 10, 7, 16, 11];
    let start = 6;
    let destination = 2;
    let res = 28;
    assert_eq!(
        Solution::distance_between_bus_stops(distance, start, destination),
        res
    );
}
