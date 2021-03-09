struct Solution;

impl Solution {
    fn min_available_duration(
        mut slots1: Vec<Vec<i32>>,
        mut slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        slots1.sort_unstable_by_key(|v| v[0]);
        slots2.sort_unstable_by_key(|v| v[0]);
        let n = slots1.len();
        let m = slots2.len();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            let s1 = &slots1[i];
            let s2 = &slots2[j];
            if s1[1] < s2[0] {
                i += 1;
                continue;
            }
            if s2[1] < s1[0] {
                j += 1;
                continue;
            }
            let start = s1[0].max(s2[0]);
            let end = s1[1].min(s2[1]);
            if end - start >= duration {
                return vec![start, start + duration];
            } else {
                if s1[0] < s2[0] {
                    i += 1;
                } else {
                    j += 1;
                }
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    let slots1 = vec_vec_i32![[10, 50], [60, 120], [140, 210]];
    let slots2 = vec_vec_i32![[0, 15], [60, 70]];
    let duration = 8;
    let res = vec![60, 68];
    assert_eq!(
        Solution::min_available_duration(slots1, slots2, duration),
        res
    );
    let slots1 = vec_vec_i32![[10, 50], [60, 120], [140, 210]];
    let slots2 = vec_vec_i32![[0, 15], [60, 70]];
    let duration = 12;
    let res: Vec<i32> = vec![];
    assert_eq!(
        Solution::min_available_duration(slots1, slots2, duration),
        res
    );
}
