
use std::any::Any;
use std::sync::RwLock;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::misc::sep;
use crate::todo_handler::{self, print_todos, Todo, TodoList};

lazy_static! {
    pub static ref GLOBAL_TODO_LIST: Mutex<TodoList> = Mutex::new(TodoList::new());
    pub static ref GLOBAL_TODO_COUNTER: Mutex<i32> = Mutex::new(1);
}

pub fn print_global_todo_list() {
    let mut global_todo_list = GLOBAL_TODO_LIST.lock();
    let length = global_todo_list.todos.len();
    
    sep();
    println!();
    match length as i32 {
        0 => println!("No todos yet."),
        _ => {}
    }
    for i in 0..length {
        global_todo_list.todos[i].print();
    }
    println!();
}

// forcibly empty
pub fn initialize_empty_todo() {
    {
        let mut global_todo_list = GLOBAL_TODO_LIST.lock();
        let mut new_todo: Todo = Todo::new(None);
        let mut global_todo_counter = GLOBAL_TODO_COUNTER.lock();
        new_todo.id = *global_todo_counter; // we dereferenced it, because otherwise, global_todo_counter itself is a MutexGuard<i32>
        *global_todo_counter += 1;
        global_todo_list.todos.push(new_todo);
    }
}

// may or may not have a name
pub fn add_new_todo(title: Option<&str>) {
    let mut global_todo_list = GLOBAL_TODO_LIST.lock();
    let mut new_todo: Todo = Todo::new(title);
    let mut global_todo_counter = GLOBAL_TODO_COUNTER.lock();
    new_todo.id = *global_todo_counter;
    *global_todo_counter+=1;
    global_todo_list.todos.push(new_todo);
}

pub fn toggle_completed(id: i32) {
    let mut global_todo_list = GLOBAL_TODO_LIST.lock();
    let length = global_todo_list.todos.len();
    for i in 0..length {
        // println!("i: {}", i);
        if global_todo_list.todos[i].id == id {
            // println!("Todo with specified id FOUND!"); // works
            global_todo_list.todos[i].completed = !global_todo_list.todos[i].completed;
            return;
        }
    }
}

pub fn rename(id: i32, new_name: String) {
    let mut global_todo_list = GLOBAL_TODO_LIST.lock();
    let length = global_todo_list.todos.len();
    for i in 0..length {
        if global_todo_list.todos[i].id == id {
            global_todo_list.todos[i].title = new_name;
            break;
        }
    }
}
