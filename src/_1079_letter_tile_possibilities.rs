struct Solution;

impl Solution {
    fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts: Vec<usize> = vec![0; 26];
        for c in tiles.chars() {
            counts[(c as u8 - b'A') as usize] += 1;
        }
        let mut res = 0;
        Self::dfs(&mut res, &mut counts);
        res
    }

    fn dfs(sum: &mut i32, counts: &mut Vec<usize>) {
        for i in 0..26 {
            if counts[i] > 0 {
                *sum += 1;
                counts[i] -= 1;
                Self::dfs(sum, counts);
                counts[i] += 1;
            }
        }
    }
}

#[test]
fn test() {
    let tiles = "AAB".to_string();
    let res = 8;
    assert_eq!(Solution::num_tile_possibilities(tiles), res);
    let tiles = "AAABBC".to_string();
    let res = 188;
    assert_eq!(Solution::num_tile_possibilities(tiles), res);
}
