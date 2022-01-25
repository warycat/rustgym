struct Solution;

impl Solution {
    fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let n = tickets.len();
        let mut arr = vec![0; n];
        let mut i = 0;
        let k = k as usize;
        while arr[k] != tickets[k] {
            if arr[i] < tickets[i] {
                arr[i] += 1;
                res += 1;
            }

            i += 1;
            i %= n;
        }
        res
    }
}

#[test]
fn test() {
    let tickets = vec![2, 3, 2];
    let k = 2;
    let res = 6;
    assert_eq!(Solution::time_required_to_buy(tickets, k), res);
    let tickets = vec![5, 1, 1, 1];
    let k = 0;
    let res = 8;
    assert_eq!(Solution::time_required_to_buy(tickets, k), res);
}
