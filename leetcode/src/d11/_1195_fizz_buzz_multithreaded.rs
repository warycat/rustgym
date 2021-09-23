use std::sync::Condvar;
use std::sync::Mutex;
struct FizzBuzz {
    n: i32,
    seq: (Mutex<i32>, Condvar),
}

impl FizzBuzz {
    fn new(n: i32) -> Self {
        let seq = (Mutex::new(1), Condvar::new());
        FizzBuzz { n, seq }
    }

    fn fizz(&self, print_fizz: impl Fn()) {
        loop {
            let (lock, cvar) = &self.seq;
            let mut g = cvar
                .wait_while(lock.lock().unwrap(), |seq| {
                    !(*seq == self.n + 1 || (*seq % 3 == 0 && *seq % 5 != 0))
                })
                .unwrap();
            if *g == self.n + 1 {
                break;
            }
            print_fizz();
            *g += 1;
            cvar.notify_all();
        }
    }

    fn buzz(&self, print_buzz: impl Fn()) {
        loop {
            let (lock, cvar) = &self.seq;
            let mut g = cvar
                .wait_while(lock.lock().unwrap(), |seq| {
                    !(*seq == self.n + 1 || (*seq % 3 != 0 && *seq % 5 == 0))
                })
                .unwrap();
            if *g == self.n + 1 {
                break;
            }
            print_buzz();
            *g += 1;
            cvar.notify_all();
        }
    }

    fn fizzbuzz(&self, print_fizz_buzz: impl Fn()) {
        loop {
            let (lock, cvar) = &self.seq;
            let mut g = cvar
                .wait_while(lock.lock().unwrap(), |seq| {
                    !(*seq == self.n + 1 || (*seq % 3 == 0 && *seq % 5 == 0))
                })
                .unwrap();
            if *g == self.n + 1 {
                break;
            }
            print_fizz_buzz();
            *g += 1;
            cvar.notify_all();
        }
    }

    fn number(&self, print_number: impl Fn(i32)) {
        loop {
            let (lock, cvar) = &self.seq;
            let mut g = cvar
                .wait_while(lock.lock().unwrap(), |seq| {
                    !(*seq == self.n + 1 || (*seq % 3 != 0 && *seq % 5 != 0))
                })
                .unwrap();
            if *g == self.n + 1 {
                break;
            }
            print_number(*g);
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

    let (tx, rx) = channel();
    let n = 15;
    let fizzbuzz = Arc::new(FizzBuzz::new(n));
    let pool = ThreadPool::new(4);
    {
        let fizzbuzz = fizzbuzz.clone();
        let tx = tx.clone();
        pool.execute(move || fizzbuzz.fizz(|| tx.send("fizz".to_string()).unwrap()));
    }
    {
        let fizzbuzz = fizzbuzz.clone();
        let tx = tx.clone();
        pool.execute(move || fizzbuzz.buzz(|| tx.send("buzz".to_string()).unwrap()));
    }
    {
        let fizzbuzz = fizzbuzz.clone();
        let tx = tx.clone();
        pool.execute(move || fizzbuzz.fizzbuzz(|| tx.send("fizzbuzz".to_string()).unwrap()));
    }
    {
        let fizzbuzz = fizzbuzz;
        let tx = tx;
        pool.execute(move || fizzbuzz.number(|x| tx.send(format!("{}", x)).unwrap()));
    }
    pool.join();
    let res: Vec<String> = rx.try_iter().collect();
    let ans = vec_string![
        "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13", "14",
        "fizzbuzz"
    ];
    assert_eq!(res, ans);
}
