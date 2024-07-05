
use std::io::{self, Stdout, Write};
use std::process;
use std::task::Wake;

// file imports
use crate::{state, todo_handler};
use crate::commands;

pub fn get_user_input(message: Option<&str>) -> String {
    sep();
    match message {
        Some(value) => println!("{}", value),
        None => {}
    }
    let mut user_input = String::new();
    print!(" > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input)
        .expect("Error while reading input!");
    return user_input;
}

pub fn get_user_input_inline(message: &str) -> String {
    sep();
    print!("{}", message);
    let mut user_input = String::new();
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
    let pieces: Vec<&str> = input.trim().split_whitespace().collect();
    // println!("{:?}", pieces);
    //println!("pieces: {:?}", pieces);
    match pieces[0] {
        "ls" => {
            //println!("length of pieces: {}", pieces.len());
            //println!("length of pieces > 0: {}", pieces.len() > 0);
            //println!("length of pieces > 1: {}", pieces.len() > 1);
           
            if pieces.len() < 2 {
                commands::list_todos();
            }
            else if pieces[1] == "-u" {
               commands::list_unchecked_todos(); 
            } 
            else if pieces[1] == "-c" {
                commands::list_checked_todos();
            }
        }

        "add" => {
            if pieces.len() == 1 {
                state::initialize_empty_todo();
                return;
            }
            let pieces_except_first = &pieces[1..];
            let name_provided = pieces_except_first.join(" ");
            state::add_new_todo(Some(name_provided.as_str()));
        }
        "toggle" | "tog" => {
            // check if the second parameter, the integer is valid
            if pieces.len() < 2 {
                println!("Invalid amount of parameters.");
                return;
            }

            let id_provided = pieces[1].parse::<i32>();
            match id_provided { 
                Ok(value) => {
                    // println!("The value you entered ({}) is valid", value);
                    // println!("{:?}", value);
                    let my_int: i32 = value;
                    state::toggle_completed(my_int);
                }
                Err(e) => {
                    println!("Invalid Input. No TODOs have been affected.");
                    return;
                }
            }
            // the following code will only execute if no errors were found in the "c"
            // part of the match
        }
        "rename" => {
            if pieces.len() < 3 {
                println!("Invalid amount of parameters.");
                return;
            }
            else {
                // check if the second argument is an integer
                let parsed_id: Result<i32, _> = pieces[1].parse::<i32>();
                match parsed_id {
                    Ok(_) => {
                        //println!("The id provided is a valid int.")
                    }
                    Err(e) => {
                        println!("Invalid id provided");
                        return;
                    }
                }
                // this code will run if everything before this was inputted
                // correctly
                let name_pieces = &pieces[2..];
                let name_compiled: String = name_pieces.join(" ");
                // println!("name compiled: \"{}\"", name_compiled);
                state::rename(parsed_id.unwrap(), name_compiled);
            }
        }
        _ => {
            println!("Command not found.");
        }
    }
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

const OPTIONS: [&str; 9] = [
    "Exit - x",
    "Help - h",
    "Say Hello - 1",
    "List TODOs - ls",
    "List unchecked TODOs - ls -u",
    "Add new nameless TODO - add",
    "Add new TODO with a name - add <name>",
    "Toggle todo - tog <id> / toggle <id>",
    "Rename todo - rename <id> <new name>"
];
