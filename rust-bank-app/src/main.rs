use colored::Colorize;

mod file_handler;
mod get_user_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = get_user_input::get_username_input();

    match file_handler::read_bank_account(&username) {
        Ok(response) => println!("{:?}", response),
        Err(e) => println!("{}", format!("Error: {}", e).red()),
    }

    Ok(())
}
