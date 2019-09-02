struct Solution;

impl Solution {
    fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut i = 0;
        let n = num_people as usize;
        let mut res: Vec<i32> = vec![0; n];
        let mut gift = 0;
        while candies > 0 {
            gift = i32::min(gift + 1, candies);
            res[i] += gift;
            candies -= gift;
            i = (i + 1) % n;
        }
        res
    }
}

#[test]
fn test() {
    let candies = 7;
    let num_people = 4;
    let res = vec![1, 2, 3, 1];
    assert_eq!(Solution::distribute_candies(candies, num_people), res);
    let candies = 10;
    let num_people = 3;
    let res = vec![5, 2, 3];
    assert_eq!(Solution::distribute_candies(candies, num_people), res);
}
