fn main() {
    let word = String::from("Hello world");
    // word2 is a string slice
    // let word2 = &word[0..5];

    let word2 = find_first_word(&word);

    println!("{}", word);
    println!("{}", word2);
}

// word is borrowed rather than transferred ownership
fn find_first_word(str: &String) -> &str {
    let mut index = 0;
    for (_, i) in str.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index += 1;
    }
    return &str[0..index];
}