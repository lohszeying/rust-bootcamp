// If the lifetime of name is going to be 'a, then lifetime of User is going to be 'a
// To let rust compiler know: Whenever name goes out of scope, User goes out of scope
// struct User<'a> {
//     name: &'a str,
//     age: u32,
// }
//
// fn main() {
//     let user;
//     {
//         let first_name = String::from("Jack");
//         user = User {
//             name: &first_name,
//             age: 32,
//         };
//     }
//     println!("First name: {}, age: {}", user.name, user.age);
// }

// ===============================

struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}

fn main() {
    let user: User;
    let first_name = String::from("Jack");
    {
        let last_name = String::from("Smith");
        user = User {
            first_name: &first_name,
            last_name: &last_name,
        };
    }
    // Code doesn't compile because last_name doesn't live long enough
    println!("First name: {}", user.first_name);
}
