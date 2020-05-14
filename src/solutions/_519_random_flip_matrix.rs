use rand::prelude::*;
use std::collections::HashMap;

struct Solution {
    size: usize,
    indexes: HashMap<usize, usize>,
    rng: ThreadRng,
    rows: usize,
    cols: usize,
}

impl Solution {
    fn new(n_rows: i32, n_cols: i32) -> Self {
        let rows = n_rows as usize;
        let cols = n_cols as usize;
        let size = rows * cols;
        let indexes = HashMap::new();
        let rng = thread_rng();
        Solution {
            size,
            indexes,
            rng,
            rows,
            cols,
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let r = self.rng.gen_range(0, self.size);
        let x = *self.indexes.entry(r).or_insert(r);
        self.size -= 1;
        let y = *self.indexes.entry(self.size).or_insert(self.size);
        *self.indexes.entry(r).or_default() = y;
        let col = x % self.cols;
        let row = x / self.cols;
        vec![row as i32, col as i32]
    }

    fn reset(&mut self) {
        self.size = self.rows * self.cols;
        self.indexes = HashMap::new();
    }
}
