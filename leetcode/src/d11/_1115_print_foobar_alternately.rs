use std::sync::Condvar;
use std::sync::Mutex;

struct FooBar {
    flag: (Mutex<bool>, Condvar),
    n: usize,
}

impl FooBar {
    fn new(n: usize) -> Self {
        let flag = (Mutex::new(true), Condvar::new());
        FooBar { flag, n }
    }

    fn foo(&self, print_foo: impl Fn()) {
        for _ in 0..self.n {
            let (lock, cvar) = &self.flag;
            let mut g = cvar
                .wait_while(lock.lock().unwrap(), |flag| !*flag)
                .unwrap();
            *g = false;
            print_foo();
            cvar.notify_all();
        }
    }

    fn bar(&self, print_bar: impl Fn()) {
        for _ in 0..self.n {
            let (lock, cvar) = &self.flag;
            let mut g = cvar.wait_while(lock.lock().unwrap(), |flag| *flag).unwrap();
            *g = true;
            print_bar();
            cvar.notify_all();
        }
    }
}

#[test]
fn test() {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use threadpool::ThreadPool;

    let foo_bar = Arc::new(FooBar::new(4));
    let (tx, rx) = channel();
    let pool = ThreadPool::new(4);
    {
        let tx = tx.clone();
        let foo_bar = foo_bar.clone();
        pool.execute(move || {
            foo_bar.foo(|| tx.send("foo".to_string()).unwrap());
        });
    }
    {
        let tx = tx;
        let foo_bar = foo_bar;
        pool.execute(move || {
            foo_bar.bar(|| tx.send("bar".to_string()).unwrap());
        });
    }
    pool.join();
    let ss: Vec<String> = rx.try_iter().collect();
    let res = ss.join("");
    let ans = "foobarfoobarfoobarfoobar".to_string();
    assert_eq!(ans, res);
}
