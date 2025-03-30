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

    // ================================================

    consuming_adapter();

    // ================================================

    iterator_adapter();
}

fn consuming_adapter() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let sum: i32 = v1_iter.sum();
    // Note: v1_iter is consumed here in v1_iter.sum(), so I cannot use v1_iter anymore

    println!("Sum: {:?}", sum);
}

// This function demonstrates the use of iterator adapters like .map() and .filter()
// Iterator adapters are methods defined on iterator types that allow you to change iterators into different kinds of iterators.
// Iterator adapters returns another iterator. A function on an original iterator that returns another iterator.
fn iterator_adapter() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    // .map and .filter works
    let v1_iter2 = v1_iter.map(|x| x + 1);

    for i in v1_iter2 {
        println!("v1_iter2: {}", i);
    }

    let new_iter = v1.iter().filter(|x| *x % 2 == 1).map(|x| x + 1);
}