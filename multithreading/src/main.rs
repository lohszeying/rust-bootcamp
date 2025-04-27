use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("hi from spawned thread {}", i);
        }
    });

    for i in 0..5 {
        println!("hi from main thread {}", i);
    }

    // wait for the spawned thread to finish
    handle.join();

    print_vector();
}

fn print_vector() {
    let x = 1;
    {
        let v = vec![1, 2, 3];

        // move ownership of v into the thread
        // Without 'move', v would be borrowed and dropped when this scope ends,
        // but the thread could still be running - creating a dangling pointer
        thread::spawn(move || {
            println!("v: {:?}", v);
        });
    }
    println!("{}", x);
}