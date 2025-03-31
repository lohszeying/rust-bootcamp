use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc is a multi-producer, single-consumer channel
    // tx is transmitter, rx is receiver
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv();

    // unwrap -> if the value is Ok, it returns the value, if it is Err, it panics
    // let received = rx.recv().unwrap();
    // println!("Got: {received}");
    // unwrap is anti pattern, should not have unwrap in codebase. should have pattern matching, if have err should catch error, etc.
    match received {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error while reading data: {}", e),
    }

    // ============================

    println!("creating multiple threads to get sum...");
    create_multiple_threads()
}

fn create_multiple_threads() {
    let (tx, rx) = mpsc::channel();
    for i in 0..8 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000..(i + 1 * 10000) - 1 {
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }

    // Note: without, receiver doesnt know all the producer has finished

    // Drop the original transmitter after cloning. Without this, the channel never closes
    // and the `for val in rx` loop will never terminate, causing the program to hang.
    // After dropping this original tx, once all thread-specific transmitter clones are
    // dropped when threads complete, the receiver will know no more messages are coming.
    drop(tx);

    let mut final_sum: u64 = 0;
    for val in rx {
        println!("Received value from thread");
        final_sum += val;
    }
    println!("{}", final_sum);
}
