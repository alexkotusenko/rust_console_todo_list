
use std::io::{self, Stdout, Write};
use std::process;
use std::task::Wake;

// file imports
use crate::todo_handler;
use crate::commands;

pub fn get_user_input(message: &str) -> String {
    sep();
    println!("{}", message);
    let mut user_input = String::new();
    print!(" > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input)
        .expect("Error while reading input!");
    return user_input;
}

pub fn run_number_command(input: Option<i32>) {
    match input {
        Some(num) => {
            match num {
                1 => {
                    commands::say_hi(); 
                } 
                _ => {
                    println!("ERROR: commanad not recognized");
                }
            }
        }
        None => {
            // end the function
            return;
        }
    }
}

fn ask_for_confirmation() -> bool {
    println!("Are you sure?");
    print!("\"y\" to confirm > ");
    let mut input: String = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("ERROR reading input");
    if (input.trim() == "y".to_string()) {
        return true;
    }
    return false;
}

pub fn run_char_command(input: Option<char>) {
    match input {
        Some(character) =>
        {
            match character {
                'h' => {
                    list_options();
                }
                'l' => {
                        commands::list_todos();
                }
                'a' => {
                        commands::add_todo();
                }
                'x' => {
                    if ask_for_confirmation() {
                        println!("BYE BYE");
                        process::exit(0);
                    }
                }
                _ => {
                    println!("ERROR: commanad not recognized");
                }
            }
        }
        None => {
            // end the function
            return;
        }
    }
}

pub fn run_string_command(input: &mut String) {
    println!("You are trying to run the following command: \n\n{}", input.trim());
}

// does not need to be public
fn sep() { 
    println!("---------------");
}

pub fn list_options() {
    sep();
    for option in OPTIONS.iter() {
        println!("{}", option);
    }
}

const OPTIONS: [&str; 5] = [
    "Exit - x",
    "Help - h",
    "Say Hello - 1",
    "List TODOs - l",
    "Add new TODO - a"
];
