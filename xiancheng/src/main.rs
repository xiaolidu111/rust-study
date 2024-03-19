use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });
    // println!("{:?}", v);

    // handle.join().unwrap();
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     // let v = String::from("hi");
    //     let v = vec![
    //         String::from("ha"),
    //         String::from("ha2"),
    //         String::from("ha2"),
    //         String::from("ha3"),
    //     ];
    //     for val in v {
    //         tx.send(val).unwrap();
    //         // println!("val is {}", v);
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // // let res = rx.recv().unwrap();
    // // println!("{:?},nihao", res);
    // for val in rx {
    //     println!("{}", val);
    // }
    // let (tx, rx) = mpsc::channel();

    // let tx1 = mpsc::Sender::clone(&tx);
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("{:?}", m);
    let counter = Arc::new(Mutex::new(0));

    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    println!("{:?}", counter);
}
