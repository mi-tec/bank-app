use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, env, fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
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

pub fn read_bank_account(username: &str) -> Result<Context, Box<dyn std::error::Error>> {
    println!("Attempting to read bank account: {:?}", username);
    let username_file = format!("{}.json", username);

    let current_dir = env::current_dir()?;
    let parent_dir = current_dir.parent().unwrap();

    let file_path = Path::new(&parent_dir)
        .join("bank-accounts")
        .join(username_file);

    let json_data = fs::read_to_string(file_path).expect("No Account available");

    let bank_data: Context = serde_json::from_str(&json_data)?;

    Ok(bank_data)
}
