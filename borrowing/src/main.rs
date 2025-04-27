fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    print_str(s2);
    println!("s1 is {}", s1.to_string())
}

fn print_str(s: &String) {
    println!("string is: {}", s);
}