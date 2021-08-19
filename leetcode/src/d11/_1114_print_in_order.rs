use std::sync::Condvar;
use std::sync::Mutex;

struct Foo {
    seq: (Mutex<i32>, Condvar),
}

impl Foo {
    fn new() -> Self {
        let seq = (Mutex::new(1), Condvar::new());
        Foo { seq }
    }

    fn first(&self, print_first: impl FnOnce()) {
        let (lock, cvar) = &self.seq;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |seq| *seq != 1)
            .unwrap();
        *g = 2;
        print_first();
        cvar.notify_all();
    }

    fn second(&self, print_second: impl FnOnce()) {
        let (lock, cvar) = &self.seq;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |seq| *seq != 2)
            .unwrap();
        *g = 3;
        print_second();
        cvar.notify_all();
    }

    fn third(&self, print_third: impl FnOnce()) {
        let (lock, cvar) = &self.seq;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |seq| *seq != 3)
            .unwrap();
        *g = 4;
        print_third();
        cvar.notify_all();
    }
}

#[test]
fn test() {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use threadpool::ThreadPool;

    let nums = vec![1, 2, 3];
    let fooo = Arc::new(Foo::new());
    let (tx, rx) = channel();

    let pool = ThreadPool::new(4);

    for x in nums {
        let tx = tx.clone();
        let fooo = fooo.clone();
        pool.execute(move || match x {
            1 => fooo.first(|| tx.send("first".to_string()).unwrap()),
            2 => fooo.second(|| tx.send("second".to_string()).unwrap()),
            3 => fooo.third(|| tx.send("third".to_string()).unwrap()),
            _ => {}
        })
    }
    pool.join();

    let ss: Vec<String> = rx.try_iter().collect();
    let res = ss.join("");
    let ans = "firstsecondthird".to_string();
    assert_eq!(ans, res);

    let nums = vec![1, 3, 2];
    let fooo = Arc::new(Foo::new());
    let (tx, rx) = channel();

    let pool = ThreadPool::new(4);

    for x in nums {
        let tx = tx.clone();
        let fooo = fooo.clone();
        pool.execute(move || match x {
            1 => fooo.first(|| tx.send("first".to_string()).unwrap()),
            2 => fooo.second(|| tx.send("second".to_string()).unwrap()),
            3 => fooo.third(|| tx.send("third".to_string()).unwrap()),
            _ => {}
        })
    }
    pool.join();

    let ss: Vec<String> = rx.try_iter().collect();
    let res = ss.join("");
    let ans = "firstsecondthird".to_string();
    assert_eq!(ans, res);
}
