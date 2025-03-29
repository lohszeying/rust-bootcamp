fn main() {
    let v1 = vec![1,2,3,4];
    // under the hood, v1 is using v1.iter()
    for val in v1 {
        println!("{}", val);
    }

    // Mutating the value in iterator
    // Ownership still belongs to v2, v2_iter is just borrowing it
    let mut v2 = vec![1,2,3,4];
    let v2_iter = v2.iter_mut();
    for val in v2_iter {
        *val = *val + 1;
    }
    println!("{:?}", v2);

    // I can borrow here again cuz in rust, i just cannot have multiple borrow at a time
    // If I shift line 19 to line 12, then this wont work
    let v2_iter_again = v2.iter_mut();
    for val in v2_iter_again {
        *val = *val + 1;
    }
    println!("{:?}", v2);
}

