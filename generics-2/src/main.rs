fn main() {
    let bigger = largest(1,2);
    let bigger_char = largest('a', 'b');
    println!("The largest number is {}", bigger);
    println!("The largest char is {}", bigger_char);
}

// The `largest` function is generic over any type `T`.
// Here, `<T>` allows the function to accept arguments of various types.
// The trait bound `T: std::cmp::PartialOrd` ensures that values of type `T` can be compared
// using the `>` operator, enabling the function to determine the larger of the two.
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

