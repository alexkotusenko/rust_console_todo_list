mod io_handler;
mod misc;
mod todo_handler;
mod parser;
mod commands;
mod state;

use todo_handler::TodoList;
use todo_handler::Todo;

fn main() {
    let mut todos: Vec<todo_handler::Todo> = Vec::new();
    
    misc::clear_console();

    misc::print_header();
    
   io_handler::list_options();
    let mut input: String = String::new();
    loop {
        input = io_handler::get_user_input(None);
        if let Some(value) = parser::parse_int(&mut input) {
            io_handler::run_number_command(Some(value));
        }
        else if let Some(value) = parser::parse_char(&mut input) {
            io_handler::run_char_command(Some(value));
        }
        else {
            io_handler::run_string_command(&mut input);
        }
    }
}
