#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct AccountInfo {
    id: String,
    name: String,
    income: f64,
    expense: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct StatementRec {
    acc_id: String,
    time: String,
    amount: f64,
    is_income: bool,
    s_type: StatementType,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum StatementType {
    Food,
    Needed,
    Subscription,
    SpecialItem,
    Transportation,
    Etcetera,
}

// read account data
fn read_data(username: &str) {
    if username == "admin" {
        println!("   Account list: ");

        if let Ok(paths) = fs::read_dir("./account") {
            for entry in paths.flatten() {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(name) = path.file_stem() {
                        println!("     => {}", name.to_str().unwrap());
                    }
                }
            }
        }
    }
}

// add statement record
fn save_data(username: String, statement: &StatementRec) -> bool {
    let json_statement = serde_json::to_string(statement).unwrap();
    fs::write(username, json_statement).unwrap();

    true
}

// create new account and return account id
fn create_acc() -> Option<(String, String)> {
    // Get client username
    print!("   Type your username! <username>\n> ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    let file: String;
    loop {
        username.clear();
        io::stdin().read_line(&mut username).unwrap();
        let username_trim = username.trim();
        if username_trim == "q" { return None; }
        let check_file = format!("./account/{}.json", username_trim);
        if Path::new(&check_file).exists() {
            print!("   This username already exist please try again!\n> ");
            io::stdout().flush().unwrap();
        } else {
            file = check_file;
            break;
        }
    }
    
    // Get client name
    print!("   Type your name! <Name Surname>\n> ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    if name == "q" { return None; }

    let id = Uuid::new_v4().to_string();
    let acc_json = serde_json::to_string(
        &AccountInfo {id: id.clone(), name: name.to_string(), income: 0.00, expense: 0.00}
    ).unwrap();
    fs::write(&file, acc_json).unwrap();

    Some((name.to_string(), username.to_string()))
}

fn edit_acc(_username: String) -> bool {
    todo!();
}

fn delete_acc() -> bool {
    todo!();
}

fn main() {
    let welcome_prompt = "Welcome, to our Income/Expense calculation program. What service would you like to use?";
    let help_prompt = 
    "Command for service:
    > Create new account  --- C
    > Edit account        --- E <account id>
    > Remove account      --- R <account id>
    > Add transaction     --- AT
    > Read data           --- RD
    > help                --- H
    > Exit program        --- q";
    
    println!("{}", welcome_prompt);
    println!("{help_prompt}");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        
        match command {
            "C" => {
                match create_acc() {
                    Some((name, username)) =>
                    print!(
                        "   Thanks for create account with us {}!\n   Here's your username for future Transaction: {}"
                    , name, username),
                    None => println!("   Account creation cancelled"),
                }
            }
            "RD" => { 
                print!("   Type your username!\n> ");
                io::stdout().flush().unwrap();

                let mut read = String::new();
                io::stdin().read_line(&mut read).unwrap();
                let read = read.trim();
                read_data(read);
            }
            "H" => {
                println!("{}", help_prompt);
            }
            "q" => {
                println!("Thank you for using our service! see you next time :)");
                break;
            }
            _ => {
                println!("Unknown command. Type 'H' for help.");
            }
        }
    }
}

