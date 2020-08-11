use rand::prelude::*;
use rustgym_util::*;

struct Solution {
    head: ListLink,
    rng: ThreadRng,
}

impl Solution {
    fn new(head: ListLink) -> Self {
        let rng = thread_rng();
        Solution { head, rng }
    }

    fn get_random(&mut self) -> i32 {
        let mut cur = &self.head;
        let mut res = 0;
        let mut count = 0;
        while let Some(node) = cur {
            let val = node.val;
            count += 1;
            if self.rng.gen_range(0, count) == 0 {
                res = val;
            }
            cur = &node.next;
        }
        res
    }
}
