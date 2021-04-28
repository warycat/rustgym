struct Solution;
struct ArrayReader {
    nums: Vec<i32>,
}

impl ArrayReader {
    fn new(nums: Vec<i32>) -> Self {
        ArrayReader { nums }
    }
    fn query(&self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        let a = a as usize;
        let b = b as usize;
        let c = c as usize;
        let d = d as usize;
        let mut diff: i32 = 0;
        for &i in &[a, b, c, d] {
            if self.nums[i] == 1 {
                diff += 1;
            } else {
                diff -= 1;
            }
        }
        diff.abs()
    }
    fn length(&self) -> i32 {
        self.nums.len() as i32
    }
}

impl Solution {
    fn get_majority(reader: &ArrayReader) -> i32 {
        let n = reader.length() as usize;
        let mut zero = 1;
        let mut one = 0;
        for i in 1..n {
            let mut other = vec![];
            for j in 1..n {
                if j != i {
                    other.push(j);
                }
                if other.len() == 3 {
                    break;
                }
            }
            let mut q1 = vec![0];
            let mut q2 = vec![i as i32];
            for j in other {
                q1.push(j as i32);
                q2.push(j as i32);
            }
            q2.sort_unstable();
            if reader.query(q1[0], q1[1], q1[2], q1[3]) != reader.query(q2[0], q2[1], q2[2], q2[3])
            {
                one += 1;
            } else {
                zero += 1;
            }
            if one > n / 2 || zero > n / 2 {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![0, 0, 1, 0, 1, 1, 1, 1];
    let reader = ArrayReader::new(nums);
    let res = 7;
    assert_eq!(Solution::get_majority(&reader), res);
}
