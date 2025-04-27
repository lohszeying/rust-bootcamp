fn main() {
    let mut s1 = String::from("Hello");
    print_str(&mut s1);
    println!("s1 is {}", s1.to_string())
}

fn print_str(s: &mut String) {
    s.push_str(" world!")
}