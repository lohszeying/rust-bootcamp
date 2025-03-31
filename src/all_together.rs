use std::fmt::Display;

// ann is announcement which should implement the Display trait
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let announcement = String::from("Today is someone's birthday!");

    let result = longest_with_an_announcement(
        &string1,
        &string2,
        announcement,
    );

    println!("The longest string is: {}", result);
}