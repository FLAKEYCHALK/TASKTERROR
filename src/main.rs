#![allow(non_snake_case)]
mod io;
mod todo;
mod user_interface;
use todo::TodoItem;
use user_interface::{display_todo_details, display_todo_titles, get_user_input, show_menu};

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        show_menu();

        let choice: i32 = get_user_input("Now what? Ready to GTFO?").parse().expect("Nuh Uh!");

        match choice {
            1 => add_todo_item(&mut todo_list),
            2 => display_todo_titles(&todo_list),
            3 => display_todo_details(&select_todo_item(&todo_list)),
            4 => delete_todo_item(&mut todo_list),
            5 => {
                println!("get rekt! bye!");
                break;
            }
            _ => println!("Nuh Uh! Please enter a number between 1 and 5."),
        }
    }
}

fn add_todo_item(todo_list: &mut Vec<TodoItem>) {
    let title = get_user_input("Enter title:");
    let description = get_user_input("Enter description:");
    let priority = get_user_input("Enter priority:");
    let status = get_user_input("Enter status:");
    let due_date = get_user_input("Enter due date:");

    let todo_item = TodoItem::new(title, description, priority, status, due_date);
    todo_list.push(todo_item);
}

fn select_todo_item(todo_list: &Vec<TodoItem>) -> TodoItem {
    display_todo_titles(todo_list);
    let index: usize = get_user_input("Enter the index of the task you want to display:")
        .parse()
        .expect("Invalid index!");

    if index > 0 && index <= todo_list.len() {
        todo_list[index - 1].clone()
    } else {
        println!("Nuh uh! Please select a valid index.");
        select_todo_item(todo_list)
    }
}

fn delete_todo_item(todo_list: &mut Vec<TodoItem>) {
    display_todo_titles(todo_list);
    let index: usize = get_user_input("Enter the index of the todo you want to delete:")
        .parse()
        .expect("Invalid index!");

    if index > 0 && index <= todo_list.len() {
        todo_list.remove(index - 1);
        println!("TODO deleted successfully!");
    } else {
        println!("Nuh Uh! Please select a valid index.");
    }
}
