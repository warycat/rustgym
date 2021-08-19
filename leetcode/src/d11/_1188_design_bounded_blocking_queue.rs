use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;

struct BoundedBlockingQueue {
    queue: (Mutex<VecDeque<i32>>, Condvar),
    capacity: usize,
}

impl BoundedBlockingQueue {
    fn new(capacity: usize) -> Self {
        let queue = (Mutex::new(VecDeque::new()), Condvar::new());
        BoundedBlockingQueue { queue, capacity }
    }

    fn enqueue(&self, element: i32) {
        let (lock, cvar) = &self.queue;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |queue| queue.len() == self.capacity)
            .unwrap();
        g.push_back(element);
        cvar.notify_all();
    }

    fn dequeue(&self) -> i32 {
        let (lock, cvar) = &self.queue;
        let mut g = cvar
            .wait_while(lock.lock().unwrap(), |queue| queue.is_empty())
            .unwrap();
        let res = g.pop_front().unwrap();
        cvar.notify_all();
        res
    }

    fn size(&self) -> i32 {
        let (lock, _cvar) = &self.queue;
        let g = lock.lock().unwrap();
        g.len() as i32
    }
}

#[derive(Debug)]
enum TestCommand {
    Enqueue(i32),
    Dequeue,
}

impl TestCommand {
    fn is_enqueue(&self) -> bool {
        use TestCommand::*;
        match self {
            Enqueue(_) => true,
            Dequeue => false,
        }
    }
    fn is_dequeue(&self) -> bool {
        use TestCommand::*;
        match self {
            Enqueue(_) => false,
            Dequeue => true,
        }
    }
}

fn test_case(
    n_producer: usize,
    n_consumer: usize,
    capacity: usize,
    test_cmds: VecDeque<TestCommand>,
) -> Vec<i32> {
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use threadpool::ThreadPool;
    use TestCommand::*;

    let (tx, rx) = channel();
    let bbq: Arc<BoundedBlockingQueue> = Arc::new(BoundedBlockingQueue::new(capacity));
    let producer_pool = ThreadPool::new(n_producer);
    let consumer_pool = ThreadPool::new(n_consumer);

    for test_cmd in test_cmds {
        let bbq = bbq.clone();
        let tx = tx.clone();
        match test_cmd {
            Enqueue(x) => producer_pool.execute(move || {
                bbq.enqueue(x);
            }),
            Dequeue => consumer_pool.execute(move || {
                let y = bbq.dequeue();
                tx.send(y).unwrap();
            }),
        }
    }
    producer_pool.join();
    consumer_pool.join();
    tx.send(bbq.size()).unwrap();
    let res: Vec<i32> = rx.try_iter().collect();
    res
}

#[test]
fn test() {
    use TestCommand::*;

    let n_producer = 1;
    let n_consumer = 1;
    let cmds = vec_string![
        "BoundedBlockingQueue",
        "enqueue",
        "dequeue",
        "dequeue",
        "enqueue",
        "enqueue",
        "enqueue",
        "enqueue",
        "dequeue"
    ];
    let args = vec_vec_i32![[2], [1], [], [], [0], [2], [3], [4], []];
    let ans = vec![1, 0, 2, 2];
    let capacity = args[0][0] as usize;
    let mut test_cmds: VecDeque<TestCommand> = VecDeque::new();
    for i in 1..cmds.len() {
        if cmds[i] == "enqueue" {
            test_cmds.push_back(Enqueue(args[i][0]));
        }
        if cmds[i] == "dequeue" {
            test_cmds.push_back(Dequeue);
        }
    }
    let res = test_case(n_producer, n_consumer, capacity, test_cmds);
    assert_eq!(res, ans);

    let n_producer = 3;
    let n_consumer = 4;
    let cmds = vec_string![
        "BoundedBlockingQueue",
        "enqueue",
        "enqueue",
        "enqueue",
        "dequeue",
        "dequeue",
        "dequeue",
        "enqueue"
    ];
    let args = vec_vec_i32![[3], [1], [0], [2], [], [], [], [3]];
    let capacity = args[0][0] as usize;
    let mut test_cmds: VecDeque<TestCommand> = VecDeque::new();
    for i in 1..cmds.len() {
        if cmds[i] == "enqueue" {
            test_cmds.push_back(Enqueue(args[i][0]));
        }
        if cmds[i] == "dequeue" {
            test_cmds.push_back(Dequeue);
        }
    }
    let res = test_case(n_producer, n_consumer, capacity, test_cmds);
    assert_eq!(res.len(), 4);
    assert_eq!(res[3], 1);
}
