
use crate::{state, todo_handler};
use crate::state::print_global_todo_list;

pub fn say_hi() {
    println!("helli");
}
pub fn list_todos() {
    print_global_todo_list();
}
pub fn add_todo() {
    println!("ADDING A TODO...");
    state::initialize_empty_todo();
}

pub fn list_unchecked_todos() {
    println!("Listing unchecked todos...");
}

pub fn list_checked_todos() {
    println!("listing checked todos!");
}
