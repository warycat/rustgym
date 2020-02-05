struct Solution;

impl Solution {
    fn read_binary_watch(num: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for i in 0..11 {
            for j in 0..60 {
                if i32::count_ones(i) + i32::count_ones(j) == num as u32 {
                    res.push(format!("{}:{:02}", i, j));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let num = 1;
    let mut res: Vec<String> =
        vec_string!["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"];
    res.sort_unstable();
    assert_eq!(Solution::read_binary_watch(num), res);
}
