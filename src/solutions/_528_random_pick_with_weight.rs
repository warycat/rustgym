use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Solution {
    dist: WeightedIndex<i32>,
    rng: ThreadRng,
}

impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        let rng = rand::thread_rng();
        let dist = WeightedIndex::new(w).unwrap();
        Solution { dist, rng }
    }

    pub fn pick_index(&mut self) -> i32 {
        self.rng.sample(&self.dist) as i32
    }
}
