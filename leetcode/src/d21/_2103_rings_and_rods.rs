struct Solution;

impl Solution {
    fn count_points(rings: String) -> i32 {
        let mut rgb = vec![vec![false; 3]; 10];
        let rings: Vec<u8> = rings.bytes().collect();
        let n = rings.len() / 2;
        for i in 0..n {
            let color = match rings[2 * i] {
                b'R' => 0,
                b'G' => 1,
                b'B' => 2,
                _ => panic!(),
            };
            let rod = (rings[2 * i + 1] - b'0') as usize;
            rgb[rod][color] = true;
        }
        let mut res = 0;
        for i in 0..10 {
            if rgb[i][0] && rgb[i][1] && rgb[i][2] {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let rings = "B0B6G0R6R0R6G9".to_string();
    let res = 1;
    assert_eq!(Solution::count_points(rings), res);
    let rings = "B0R0G0R9R0B0G0".to_string();
    let res = 1;
    assert_eq!(Solution::count_points(rings), res);
    let rings = "G4".to_string();
    let res = 0;
    assert_eq!(Solution::count_points(rings), res);
}
