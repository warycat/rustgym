use std::sync::Condvar;
use std::sync::Mutex;

struct H2O {
    balance: (Mutex<(i32, i32)>, Condvar),
}

impl H2O {
    fn new() -> Self {
        let balance = (Mutex::new((0, 0)), Condvar::new());
        H2O { balance }
    }

    fn hydrogen(&self, release_hydrogen: impl FnOnce()) {
        let (lock, cvar) = &self.balance;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |balance| balance.0 == 2)
            .unwrap();
        g.0 += 1;
        if *g == (2, 1) {
            *g = (0, 0);
        }
        release_hydrogen();
        cvar.notify_all();
    }

    fn oxygen(&self, release_oxygen: impl FnOnce()) {
        let (lock, cvar) = &self.balance;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |balance| balance.1 == 1)
            .unwrap();
        g.1 += 1;
        if *g == (2, 1) {
            *g = (0, 0);
        }
        release_oxygen();
        cvar.notify_all();
    }
}

#[test]
fn test() {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use threadpool::ThreadPool;

    let (tx, rx) = channel();
    let water = "OOHHHH".to_string();
    let h2o = Arc::new(H2O::new());
    let pool = ThreadPool::new(2);
    for m in water.chars() {
        let h2o = h2o.clone();
        let tx = tx.clone();
        pool.execute(move || match m {
            'H' => h2o.hydrogen(|| tx.send('H').unwrap()),
            'O' => h2o.oxygen(|| tx.send('O').unwrap()),
            _ => {}
        });
    }
    pool.join();
    let res: Vec<char> = rx.try_iter().collect();
    assert_eq!(res.len(), 6);
    let ohh = vec!['O', 'H', 'H'];
    let hoh = vec!['H', 'O', 'H'];
    let hho = vec!['H', 'H', 'O'];
    let all = vec![ohh, hoh, hho];
    for w in res.chunks(3) {
        assert!(all.iter().any(|x| x == w));
    }
}
