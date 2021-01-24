struct Solution;

const M: usize = 1440;

impl Solution {
    fn find_min_difference(time_points: Vec<String>) -> i32 {
        let n = time_points.len();
        if n > M {
            return 0;
        }
        let mut a = [false; M];
        for time_point in time_points {
            let t = Self::clock_to_minute(time_point);
            if a[t as usize] {
                return 0;
            } else {
                a[t as usize] = true;
            }
        }
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;
        let mut prev: Option<usize> = None;
        let mut min = M;
        for i in 0..M {
            if a[i] {
                if first == None {
                    prev = Some(i);
                    first = Some(i);
                } else {
                    min = usize::min(min, i - prev.unwrap());
                    prev = Some(i);
                    last = Some(i);
                }
            }
        }
        min = usize::min(first.unwrap() + M - last.unwrap() as usize, min);
        min as i32
    }

    fn clock_to_minute(s: String) -> usize {
        let parts: Vec<usize> = s.split(':').map(|s| s.parse::<usize>().unwrap()).collect();
        parts[0] * 60 + parts[1]
    }
}

#[test]
fn test() {
    let time_points = vec_string!["23:59", "00:00"];
    assert_eq!(Solution::find_min_difference(time_points), 1);
}
