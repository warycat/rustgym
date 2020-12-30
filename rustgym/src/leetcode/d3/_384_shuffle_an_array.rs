use rand::prelude::*;

struct Solution {
    rng: ThreadRng,
    nums: Vec<i32>,
    n: usize,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let rng = thread_rng();
        Solution { rng, nums, n }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.to_vec()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut v = self.nums.to_vec();
        let n = self.n;
        for i in 0..n {
            let j = self.rng.gen_range(i, n);
            v.swap(i, j);
        }
        v
    }
}
