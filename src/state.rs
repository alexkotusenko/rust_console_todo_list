
use std::any::Any;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::misc::sep;
use crate::todo_handler::{self, print_todos, Todo};

lazy_static! {
    pub static ref GLOBAL_TODO_LIST: Mutex<Vec<Todo>> = Mutex::new(Vec::new());
    pub static ref GLOBAL_TODO_COUNTER: Mutex<i32> = Mutex::new(0);
}

pub fn print_global_todo_list() {
    let mut global_todo_list = GLOBAL_TODO_LIST.lock();
    let length = global_todo_list.len();
    
    println!("Printing global todo list...");
    
    for i in 0..length {
        global_todo_list[i].print();
    }
}

pub fn initialize_empty_todo() {
    {
        let mut global_todo_list = GLOBAL_TODO_LIST.lock();
        let mut new_todo: Todo = Todo::new(None);
        let mut global_todo_counter = GLOBAL_TODO_COUNTER.lock();
        new_todo.id = *global_todo_counter; // we dereferenced it, because otherwise, global_todo_counter itself is a MutexGuard<i32>
        *global_todo_counter += 1;
        global_todo_list.push(new_todo);
    }
}