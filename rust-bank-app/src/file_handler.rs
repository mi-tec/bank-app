use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Write},
};

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
    let parent_dir = current_dir.parent().ok_or("No parent directory found")?;
    let bank_accounts_dir = parent_dir.join("bank-accounts");
    let file_path = bank_accounts_dir.join(&username_file);

    if !file_path.is_file() {
        println!("{}", format!("Account {} doesn't exist", username).yellow());

        print!(
            "{} {}{}{}{}",
            "Do you want to create a new account ".bold(),
            "[Y]".green().bold(),
            "es/".bold(),
            "(n)".red().bold(),
            "0: ".bold()
        );
        io::stdout().flush()?;
        io::stdout().flush()?;

        let mut create_user = String::new();
        io::stdin().read_line(&mut create_user)?;

        let create_user_response = create_user.trim().to_lowercase();
        let create_user_choice = match create_user_response.as_str() {
            "yes" | "y" => true,
            "no" | "n" => false,
            _ => {
                println!("Invalid input. Assuming 'no'.");
                false
            }
        };

        if !create_user_choice {
            return Err("Account creation canceled. Exiting the program".into());
        }

        print!("Enter Fullname: ");
        io::stdout().flush()?;

        let mut new_user_name = String::new();
        io::stdin().read_line(&mut new_user_name)?;
        let new_user_name = new_user_name.trim().to_string();

        let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
        let mut new_username = String::new();
        loop {
            print!("{}", "Enter username: ".green().bold());
            io::stdout().flush()?;
            new_username.clear();
            io::stdin().read_line(&mut new_username)?;
            new_username = new_username.trim().to_string();

            let sanitized_username = re.replace_all(&new_username, "").to_string();

            let check_for_existing_user =
                bank_accounts_dir.join(format!("{}.json", &sanitized_username));
            if check_for_existing_user.is_file() {
                println!(
                    "{}",
                    "Username already exists. Please choose a different one.".red()
                );
            } else {
                new_username = sanitized_username;
                break;
            }
        }

        print!("Enter password: ");
        io::stdout().flush()?;

        let mut new_password = String::new();
        io::stdin().read_line(&mut new_password)?;
        let new_password = new_password.trim().to_string();

        println!("Please wait, account is being created...");

        fs::create_dir_all(&bank_accounts_dir)?;

        let new_file_path = bank_accounts_dir.join(format!("{}.json", new_username));

        let new_account = Context {
            name: new_user_name,
            username: new_username.clone(),
            password: new_password,
            account_data: AccountData {
                balance: "0.00".to_string(),
                on_hold: "0.00".to_string(),
            },
            history: HashMap::new(),
        };

        let mut file = File::create(&new_file_path)?;
        let json_data = serde_json::to_string_pretty(&new_account)?;
        file.write_all(json_data.as_bytes())?;

        println!(
            "{} : {}",
            "Account created successfully for".green().bold(),
            &new_username
        );
        return Ok(new_account);
    }

    let json_data = fs::read_to_string(&file_path)?;
    let bank_data: Context = serde_json::from_str(&json_data)?;

    Ok(bank_data)
}
