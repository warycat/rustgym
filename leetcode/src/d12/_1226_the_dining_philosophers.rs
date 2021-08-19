use std::sync::Condvar;
use std::sync::Mutex;

struct DiningPhilosophers {
    eating: (Mutex<i32>, Condvar),
}

impl DiningPhilosophers {
    fn new() -> Self {
        let eating = (Mutex::new(-1), Condvar::new());
        DiningPhilosophers { eating }
    }

    fn wants_to_eat(
        &self,
        philosopher: i32,
        pick_left_fork: impl Fn(),
        pick_right_fork: impl Fn(),
        eat: impl Fn(),
        put_left_fork: impl Fn(),
        put_right_fork: impl Fn(),
    ) {
        let (lock, cvar) = &self.eating;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |eating| *eating != -1)
            .unwrap();
        *g = philosopher;
        pick_left_fork();
        pick_right_fork();
        eat();
        put_left_fork();
        put_right_fork();
        *g = -1;
        cvar.notify_all();
    }
}

#[test]
fn test() {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use threadpool::ThreadPool;

    let pool = ThreadPool::new(5);
    let dining_philosophers = Arc::new(DiningPhilosophers::new());
    let n = 1;
    let (tx, rx) = channel();
    for _ in 0..n {
        for i in 0..5 {
            let dining_philosophers = dining_philosophers.clone();
            let tx = tx.clone();
            pool.execute(move || {
                dining_philosophers.wants_to_eat(
                    i,
                    || tx.send(vec![i, 1, 1]).unwrap(),
                    || tx.send(vec![i, 2, 1]).unwrap(),
                    || tx.send(vec![i, 0, 3]).unwrap(),
                    || tx.send(vec![i, 1, 2]).unwrap(),
                    || tx.send(vec![i, 2, 2]).unwrap(),
                )
            });
        }
    }
    pool.join();
    let res: Vec<Vec<i32>> = rx.try_iter().collect();
    let ans = vec_vec_i32![
        [4, 2, 1],
        [4, 1, 1],
        [0, 1, 1],
        [2, 2, 1],
        [2, 1, 1],
        [2, 0, 3],
        [2, 1, 2],
        [2, 2, 2],
        [4, 0, 3],
        [4, 1, 2],
        [0, 2, 1],
        [4, 2, 2],
        [3, 2, 1],
        [3, 1, 1],
        [0, 0, 3],
        [0, 1, 2],
        [0, 2, 2],
        [1, 2, 1],
        [1, 1, 1],
        [3, 0, 3],
        [3, 1, 2],
        [3, 2, 2],
        [1, 0, 3],
        [1, 1, 2],
        [1, 2, 2]
    ];
    assert_eq!(res.len(), ans.len());
}
