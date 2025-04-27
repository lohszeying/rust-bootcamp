trait Summary {
    // Using default implementation, if there is no implementation eg. when struct implements trait Summary, then this will be used.
    fn summarize(&self) -> String {
        return String::from("Summarise!")
    }

    // Without default implementation:
    // fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

// For the user struct, I have implemented summary trait
// User struct implements the summary trait
impl Summary for User {
    fn summarize(&self) -> String {
        format!("User: {} is {} years old", self.name, self.age)
    }
}

struct RandomStruct {
    name: String,
}

impl Summary for RandomStruct {
    fn summarize(&self) -> String {
        format!("RandomStruct: {}", self.name)
    }
}
impl Summary for String {}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{}", user.summarize());

    let random_struct = RandomStruct{
        name: String::from("Lala"),
    };
    println!("{}", random_struct.summarize());


    notify(String::from("test"));
    notify(random_struct);
}


// Traits as parameters
fn notify(u: impl Summary) {
    println!("{}", u.summarize());
}
