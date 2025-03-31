// If the lifetime of name is going to be 'a, then lifetime of User is going to be 'a
// To let rust compiler know: Whenever name goes out of scope, User goes out of scope
struct User<'a> {
    name: &'a str,
    age: u32,
}

fn main() {
    let user;
    {
        let first_name = String::from("Jack");
        user = User {
            name: &first_name,
            age: 32,
        };
    }
    println!("First name: {}, age: {}", user.name, user.age);
}
