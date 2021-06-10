use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {
    rng: ThreadRng,
    map: HashMap<usize, usize>,
    m: usize,
    n: usize,
}

impl Solution {
    fn new(n: i32, mut blacklist: Vec<i32>) -> Self {
        let n = n as usize;
        let rng = thread_rng();
        blacklist.sort_unstable();
        let m = blacklist.len();
        let set: HashSet<i32> = blacklist.iter().copied().collect();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut j = 0;
        for i in n - m..n {
            if !set.contains(&(i as i32)) {
                map.insert(blacklist[j] as usize, i);
                j += 1;
            }
        }
        Solution { rng, map, m, n }
    }

    fn pick(&mut self) -> i32 {
        let x = self.rng.gen_range(0, self.n - self.m);
        if let Some(&v) = self.map.get(&x) {
            v as i32
        } else {
            x as i32
        }
    }
}
