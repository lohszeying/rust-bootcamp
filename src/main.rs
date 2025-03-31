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
}

