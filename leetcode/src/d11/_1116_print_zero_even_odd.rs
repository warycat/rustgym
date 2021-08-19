use std::sync::Condvar;
use std::sync::Mutex;

struct ZeroEvenOdd {
    state: (Mutex<i32>, Condvar),
    n: i32,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        let state = (Mutex::new(0), Condvar::new());
        ZeroEvenOdd { state, n }
    }

    fn zero(&self, print_number: impl Fn(i32)) {
        self.job(true, print_number, |state| {
            !(*state == 2 * self.n || *state % 2 == 0)
        });
    }

    fn odd(&self, print_number: impl Fn(i32)) {
        self.job(false, print_number, |state| {
            !(*state == 2 * self.n || *state % 2 == 1 && *state % 4 == 1)
        });
    }

    fn even(&self, print_number: impl Fn(i32)) {
        self.job(false, print_number, |state| {
            !(*state == 2 * self.n || *state % 2 == 1 && *state % 4 == 3)
        });
    }

    fn job(
        &self,
        zero: bool,
        print_number: impl Fn(i32),
        condition: impl FnMut(&mut i32) -> bool + Copy,
    ) {
        loop {
            let (lock, cvar) = &self.state;
            let mut g = cvar.wait_while(lock.lock().unwrap(), condition).unwrap();
            if *g == 2 * self.n {
                break;
            }
            print_number(if zero { 0 } else { (*g + 1) / 2 });
            *g += 1;
            cvar.notify_all();
        }
    }
}

#[test]
fn test() {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use threadpool::ThreadPool;

    let zero_even_odd = Arc::new(ZeroEvenOdd::new(5));
    let (tx, rx) = channel();
    let pool = ThreadPool::new(4);

    for i in 0..3 {
        let zero_even_odd = zero_even_odd.clone();
        let tx = tx.clone();
        pool.execute(move || match i {
            0 => zero_even_odd.zero(|x| tx.send(x.to_string()).unwrap()),
            1 => zero_even_odd.even(|x| tx.send(x.to_string()).unwrap()),
            2 => zero_even_odd.odd(|x| tx.send(x.to_string()).unwrap()),
            _ => panic!(),
        });
    }
    pool.join();
    let nums: Vec<String> = rx.try_iter().collect();
    let res = nums.join("");
    let ans = "0102030405".to_string();
    assert_eq!(res, ans);
}
