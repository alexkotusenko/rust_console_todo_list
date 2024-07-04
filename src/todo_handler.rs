#[derive(Debug, Default)]
pub struct Todo {
    pub id: i32,
    title: String,
    completed: bool
}

impl Todo {
    // Construction function with default values
    pub fn new(title: Option<&str>) -> Self {
        let mut title_to_pass: &str = "";
        match title {
            Some(title_provided) => title_to_pass = title_provided,
            None => title_to_pass = "No_title"
        }
        Todo {
            title: String::from(title_to_pass),
            id: -1,
            completed: false
        }
    }
    pub fn print(&self) {
        match self.completed {
            true => print!("[x] "),
            false => print!("[ ] ")
        }
        println!("{} - #{}", self.title, self.id);
    }
}

#[derive(Debug)]
pub struct TodoList {
    todos: Vec<Todo> 
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new()
        }
    }
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
    pub fn print(&self) {
        let size: usize = self.todos.len();
        println!("size: {}", size);
        for i in 0..size {
            println!("{}", self.todos[i].title);
        }
    }
}

pub fn print_todos(todo_list: Vec<Todo>) {
    let length = todo_list.len();
    println!("length of the todo list: {}", length);
}

pub fn say_hi() {
    println!("Hi from the todo_handler.rs file!");
}
