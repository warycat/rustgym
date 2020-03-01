use rand::prelude::*;

struct Solution {
    rng: ThreadRng,
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let rng = thread_rng();
        Solution { rng, nums }
    }
    fn pick(&mut self, target: i32) -> i32 {
        let n = self.nums.len();
        let mut count = 0;
        let mut res = 0;
        for i in 0..n {
            if self.nums[i] == target {
                count += 1;
                if self.rng.gen_range(0, count) == 0 {
                    res = i;
                }
            }
        }
        res as i32
    }
}
