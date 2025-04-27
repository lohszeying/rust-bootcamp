fn find_first_a(s: &str) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    // Note: can either use return None; or just None (is a return statement if there is no ;)
    None
}

fn main() {
    let my_string = String::from("hello world a");
    match find_first_a(&my_string) {
        Some(index) => println!("Found letter a in index {}", index),
        None => println!("Couldn't find letter a within the string {}", my_string),
    }
}