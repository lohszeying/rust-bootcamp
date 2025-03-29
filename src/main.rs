fn main() {
    let v1 = vec![1,2,3,4];
    // under the hood, v1 is using v1.iter()
    for val in v1 {
        println!("{}", val);
    }
    // Note: v1 doesnt work anymore because v1 is using v1.into_iter() under the hood,unless i choose to use v1.iter() in the for loop
    // println!("v1 is {:?}", v1);

    // ================================================

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

    // ================================================

    // The real way an iterator expects you to iterate an array (for loop hides the complexity but this is what's happening under the hood):
    let nums = vec![1,2,3,4];
    let mut iter = nums.iter();
    // while there is still value in the iter.next()
    while let Some(val) = iter.next() {
        println!("{}", val); }

    // ================================================

    // into_iter (transfer ownership to v3_iter!)
    // Can use this if i am sure i wont use v3 anymore. Performance issue! cuz if just use iter() which just borrows, it will use pointer to reference!
    // if I just write for val in v3, then it actually converts into iter under the hood, and I cant use v3 anymore.
    let v3 = vec![1,2,3];
    let v3_iter = v3.into_iter();
    for val in v3_iter {
        println!("into_iter val: {}", val);
    }
}



