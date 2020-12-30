struct Solution;

use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;

impl Solution {
    fn rand10() -> i32 {
        let distribution: Uniform<i32> = Uniform::new(0, 10);
        let mut rng = thread_rng();
        rng.sample(distribution) + 1
    }
}
