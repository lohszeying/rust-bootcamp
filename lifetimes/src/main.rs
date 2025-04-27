// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }
//
// fn main() {
//     let ans;
//     let str1 = String::from("small");
//     {
//         let str2 = String::from("longer");
//         ans = longest(&str1, &str2);
//     }
//     // Rust compiler is complaining, because there is change that ans will be dangling pointer.
//     // How to fix this? With generic lifetime annotation.
//     println!("longest: {}", ans);
// }

// ====================================================

fn main() {
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        ans = longest(&str1, &str2);
    }
    println!("longest: {}", ans);
}

// <'a> is generic lifetime annotation
// currently it looks like str1 has lifetime 'a, str2 has lifetime 'a and return type has lifetime 'a
// but actually it means all 3 are related (str1: &'a str, str2: &'a str) -> &'a str,
// whatever that is intersection of these lifetime is what is the lifetime of return type
// NOTE: Having generic lifetime annotation is just a way to tell compiler about the lifetime. So when I try to use cargo build, currently
// build will still fail because str2 is out of scope and hence unable to print ans in line 30.
// If I remove line 30 or shift it to within scope then compiler can compile
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
