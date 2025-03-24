use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod file_handler;
mod get_user_input;

pub struct Context {
    pub name: String,
    pub username: String,
    pub password: String,
    pub account_data: AccountData,
    pub history: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData {
    pub balance: String,
    pub on_hold: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = get_user_input::get_username_input();
    let response: Context = file_handler::read_bank_account(&username);

    println!("{:?}", response);

    // println!("Name: {}", response.name);
    // println!("Username: {}", response.username);
    // println!("Password: {:?}", response.password);
    // println!("account_data: {:?}", bank_data.account_data);

    Ok(())
}
