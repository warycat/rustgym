struct Solution;

impl Solution {
    fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let n = people.len();
        people.sort_unstable();
        people.reverse();
        let mut i = 0;
        let mut j = n - 1;
        let mut res = 0;
        while i <= j {
            res += 1;
            if people[i] + people[j] <= limit {
                j -= 1;
            }
            i += 1;
        }
        res
    }
}

#[test]
fn test() {
    let people = vec![1, 2];
    let limit = 3;
    let res = 1;
    assert_eq!(Solution::num_rescue_boats(people, limit), res);
    let people = vec![3, 2, 2, 1];
    let limit = 3;
    let res = 3;
    assert_eq!(Solution::num_rescue_boats(people, limit), res);
    let people = vec![3, 5, 3, 4];
    let limit = 5;
    let res = 4;
    assert_eq!(Solution::num_rescue_boats(people, limit), res);
}
