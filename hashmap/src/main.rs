use std::collections::HashMap;

fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("me"), 21);
    users.insert(String::from("aaaa"), 33);

    let user1 = users.get("me");

    println!("{}", user1.unwrap());

    match users.get("lalala not found") {
        Some(age) => println!("Age: {}", age),
        None => println!("no user"),
    }

    // Convert from tuple to hashmap
    let my_tuple = vec![(String::from("a"), 21), (String::from("aa"), 33)];
    println!("my values: {:?}", group_values_by_key(my_tuple));
}

fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    hm
}

