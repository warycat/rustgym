extern crate rand;
use rand::prelude::*;

struct Solution {
    indexes: Vec<usize>,
    m: usize,
    rng: ThreadRng,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut indexes = vec![];
        let mut m: usize = 0;
        for n in w {
            m += n as usize;
            indexes.push(m);
        }
        let rng = rand::thread_rng();
        Solution { indexes, m, rng }
    }

    fn pick_index(&mut self) -> i32 {
        let r = self.rng.gen_range(1, self.m + 1) as usize;
        let index = match self.indexes.binary_search(&r) {
            Ok(x) => x,
            Err(x) => x,
        };
        index as i32
    }
}
