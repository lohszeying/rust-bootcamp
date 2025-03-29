fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ans = even_filter(&vec);

    println!("even filter {:?}", ans);
    println!("original vec {:?}", vec);

    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);

    even_filter_in_place(&mut vec2);
    println!("even filter in place: {:?}", vec2);
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            // dereferencing a reference to a value
            // The asterisk (`*`) is the dereference operator in Rust. This suggests that `val` is a reference to some value (like `&T`), and `*val` accesses the actual value that the reference points to.
            new_vec.push(*val);
        }
    }
    return new_vec;
}

fn even_filter_in_place(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
