use std::io::{self, Write};

pub fn get_username_input() -> String {
    print!("Enter username: ");
    let _ = io::stdout().flush();

    let mut username_input = String::new();

    let _ = io::stdin()
        .read_line(&mut username_input)
        .expect("Failed to read line");

    username_input = username_input.trim().to_string();

    return username_input;
}
