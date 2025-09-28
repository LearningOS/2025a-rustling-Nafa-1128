// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 启动线程发送数据，返回线程句柄
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Vec<thread::JoinHandle<()>> {
    let mut handles = vec![];

    // 线程 1 发送 first_half
    let tx1 = tx.clone();
    let first_half = q.first_half;
    handles.push(thread::spawn(move || {
        for val in first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }));

    // 线程 2 发送 second_half
    let tx2 = tx.clone();
    let second_half = q.second_half;
    handles.push(thread::spawn(move || {
        for val in second_half {
            println!("sending {:?}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }));

    handles
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 启动发送线程并获取句柄
    let handles = send_tx(queue, tx);

    // 等待发送线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 收取所有数字
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
