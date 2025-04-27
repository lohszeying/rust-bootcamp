// Usually the convention for generics is T, U, W, X, etc
struct Point<T> {
    x: T,
    y: T,
}

// Can also do it like this, but means the type of y and z must be the same now.
// struct Point<A, B> {
//     x: A,
//     y: B,
//     z: B,
// }

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("float point: ({}, {})", float_point.x, float_point.y);
}