use std::fs;

fn main() -> () {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    let res2 = read_from_file_own_err_implementation("example.txt".to_string());
    match res2 {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn read_from_file_own_err_implementation(file_content: String) -> Result<String, String> {
    println!("reading from own fn...");
    let res = fs::read_to_string(file_content);
    match res {
        Ok(content) => Ok(content),
        Err(_) => Err("Error reading file".to_string()),
    }
}