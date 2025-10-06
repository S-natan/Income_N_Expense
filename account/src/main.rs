#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

#[derive(Serialize, Deserialize, Debug)]
struct AccountData {
    info: AccountInfo,
    transaction: Vec<StatementRec>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountInfo {
    id: String,
    name: String,
    income: f64,
    expense: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct StatementRec {
    acc_name: String,
    time: String,
    amount: f64,
    is_income: bool,
    tag: StatementType,
    description: String,
}

impl StatementRec {
    fn new(
        acc_name: String,
        time: String,
        amount: f64,
        is_income: bool,
        tag: StatementType,
        description: String
    ) -> Self {
        Self {
            acc_name,
            time,
            amount,
            is_income,
            tag,
            description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum StatementType {
    Food,
    Needed,
    Subscription,
    SpecialItem,
    Transportation,
    Rent,
    
    Wages,
    PocketMoney,
    ExtraIncome,

    Etcetera,
}

fn load_acc_list() -> Vec<String> {
    let acc_list_path = "./account_list.json";

    if Path::new(acc_list_path).exists() {
        match fs::read_to_string(acc_list_path) {
            Ok(data) => {
                serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    }
}

fn save_acc_list(account_list: &Vec<String>) {
    let acc_list_path = "./account_list.json";
    let json = serde_json::to_string_pretty(account_list).unwrap();
    fs::write(acc_list_path, json).expect("Failed to save account list");
} 

// read account data
fn read_data(account_list: &mut Vec<String>) {
    println!("   Account list: ");
    for acc in &mut *account_list {
        println!("     => {}", acc.to_string());
    }

    loop {
        print!("   Type your account name for further detail\n> ");
        io::stdout().flush().unwrap();

        let mut read = String::new();
        io::stdin().read_line(&mut read).unwrap();
        let read_trim = read.trim().to_string();
        if read_trim == "q" { 
            println!("Reading account command is cancel");
            return; 
        }
        if !account_list.contains(&read_trim) {
            println!("   The account doesn't exist please try again");
            continue;
        }
        println!("   Haven't implement this fn can't read {read_trim}");
        read.clear();
        break;
    }
}

fn load_acc_data(file_path: &str) -> Result<AccountData, String> {
    // Need to load data to change info inside
    let data = fs::read_to_string(file_path).map_err(|_| "Couldn't read account file".to_string())?;

    if let Ok(acc_data) = serde_json::from_str::<AccountData>(&data) {
        return Ok(acc_data);
    } else {
        return Err("Failed to parse account data".to_string());
    }
}

// add statement record
fn save_data(account_list: &mut Vec<String>) -> bool {
    let transaction = get_statement(account_list);
    let acc_file_path = format!("./account/{}.json", transaction.acc_name);
    let mut acc_data: AccountData = match load_acc_data(&acc_file_path) {
        Ok(data) => data,
        Err(e) => {
            println!("   Error: {e}");
            return false;
        }
    };

    if transaction.is_income {
        acc_data.info.income += transaction.amount;
    } else {
        acc_data.info.income -= transaction.amount;
    }
    acc_data.transaction.push(transaction);

    let acc_json = serde_json::to_string_pretty(&acc_data).expect("Failed to serialize account data");
    fs::write(&acc_file_path, acc_json).expect("Failed to write updated account data");

    true
}

fn get_statement(account_list: &mut Vec<String>) -> StatementRec {
    let acc_name: String;
    let time: String = Local::now().to_string();
    let amount: f64;
    let is_income: bool;
    let tag: StatementType;
    let mut description: String = Default::default();
    loop {
        let mut input = String::new();
        print!("   Enter account name: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Read account name failed");
        if account_list.contains(&input.trim().to_string()) {
            acc_name = input.trim().to_string();
            break;
        }
        println!("   This account doesn't exist please try again");
    }

    loop {
        let mut input = String::new();
        print!("   Enter amount: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Read amount failed");
        match input.trim().parse::<f64>() {
        Ok(value) => {
            if value >= 0.0 {
                amount = value;
                break;
            } else {
                println!("   The amount can't be negative");
            }
        }
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
        }
    }
    }

    loop {
        let mut input = String::new();
        print!("   Enter income or expense(I/E): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Read boolean failed");
        let input_trim = input.trim().to_uppercase();
        if input_trim == "I" {
            is_income = true;
            break;
        } else if input_trim == "E" {
            is_income = false;
            break;
        } else {
            println!("   You must enter either 'I' for income and 'E' for expense");
        }
    }

    println!(
        "   Statement type list
        - Food
        - Needed
        - Subscription
        - SpecialItem
        - Transportation
        - Rent
        
        - Wages
        - PocketMoney
        - ExtraIncome

        - Etcetera"
    );
    print!("   Enter statement type: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    tag = get_statement_type(input.trim());

    print!("   Enter description: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut description).expect("Read description failed");

    return StatementRec::new(acc_name, time, amount, is_income, tag, description);
}

fn get_statement_type(input: &str) -> StatementType {
    match input.to_lowercase().as_str() {
        "food" => StatementType::Food,
        "needed" => StatementType::Needed,
        "subscription" => StatementType::Subscription,
        "specialitem" | "special item" => StatementType::SpecialItem,
        "transportation" => StatementType::Transportation,
        "rent" => StatementType::Rent,
        "wages" => StatementType::Wages,
        "pocketmoney" | "pocket money" => StatementType::PocketMoney,
        "extraincome" | "extra income" => StatementType::ExtraIncome,
        "etcetera" | "etc" => StatementType::Etcetera,
        _ => StatementType::Etcetera,
    }
}

// create new account and return account id
fn create_acc(account_list: &mut Vec<String>) -> Option<String> {
    // Get account name
    print!("   Type your account name! <name>\n> ");
    io::stdout().flush().expect("stdout failed in create_acc");
    let name: String;
    let file: String;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read name fail");
        let name_trim = input.trim().to_string();
        if name_trim == "q" { return None; }
        if account_list.contains(&name_trim) {
            print!("   This name already exist please try again!\n> ");
            io::stdout().flush().unwrap();
        } else {
            name = name_trim.clone();
            file = format!("./account/{}.json", name_trim);
            break;
        }
    }
    
    let id = Uuid::new_v4().to_string();
    let acc_json = serde_json::to_string(
        &AccountData {
            info: AccountInfo {
                id: id.clone(),
                name: name.clone(),
                income: 0.00,
                expense: 0.00
            },
            transaction: Vec::new(),
        }
    ).unwrap();
    fs::write(&file, acc_json).unwrap();
    account_list.push(name.clone());

    Some(name)
}

fn edit_acc(_name: String) -> bool {
    todo!();
}

fn delete_acc() -> bool {
    todo!();
}

fn main() {
    let mut account_list: Vec<String> = load_acc_list();

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
                match create_acc(&mut account_list) {
                    Some(acc_name) => {
                        println!(
                            "   Your account successfully create!\n   Here's your account name for transaction record: {acc_name}"
                        );
                        save_acc_list(&account_list);
                    }
                    None => println!("   Account creation cancelled"),
                }
            }
            "AT" => { save_data(&mut account_list); }
            "RD" => { read_data(&mut account_list); }
            "H" => { println!("{help_prompt}"); }
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

