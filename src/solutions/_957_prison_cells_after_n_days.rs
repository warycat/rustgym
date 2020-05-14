struct Solution;

use std::collections::HashMap;

impl Solution {
    fn prison_after_n_days(mut cells: Vec<i32>, mut n: i32) -> Vec<i32> {
        let mut hm: HashMap<Vec<i32>, i32> = HashMap::new();
        while n > 0 {
            hm.insert(cells.to_vec(), n);
            let mut next = vec![0; 8];
            for i in 1..7 {
                next[i] = 1 - (cells[i - 1] ^ cells[i + 1]);
            }
            cells = next;
            n -= 1;
            if let Some(m) = hm.get(&cells) {
                n %= m - n;
            }
        }
        cells
    }
}

#[test]
fn test() {
    let cells = vec![0, 1, 0, 1, 1, 0, 0, 1];
    let n = 7;
    let res = vec![0, 0, 1, 1, 0, 0, 0, 0];
    assert_eq!(Solution::prison_after_n_days(cells, n), res);
    let cells = vec![1, 0, 0, 1, 0, 0, 1, 0];
    let n = 1_000_000_000;
    let res = vec![0, 0, 1, 1, 1, 1, 1, 0];
    assert_eq!(Solution::prison_after_n_days(cells, n), res);
}
