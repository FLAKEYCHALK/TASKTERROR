use crate::todo::TodoItem;
use crate::io::read_input;

pub fn show_menu() {
    println!("=================================");
    println!("           TASKTERROR");
    println!("=================================");
    println!("  [1] Add a new task");
    println!("  [2] List all tasks");
    println!("  [3] Display a task's details");
    println!("  [4] Delete a task");
    println!("  [5] Exit");
    println!("=================================");
    println!();
}

pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    read_input()
}

pub fn display_todo_titles(todo_list: &Vec<TodoItem>) {
    println!("Tasks ???:");
    for (index, todo) in todo_list.iter().enumerate() {
        println!("  [{}] {}", index + 1, todo.title);
    }
}

pub fn display_todo_details(todo: &TodoItem) {
    println!("Details:");
    println!("  Title: {}", todo.title);
    println!("  Description: {}", todo.description);
    println!("  Priority: {}", todo.priority);
    println!("  Status: {}", todo.status);
    println!("  Due Date: {}", todo.due_date);
}
