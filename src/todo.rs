#[derive(Clone)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub due_date: String,
}

impl TodoItem {
    pub fn new(title: String, description: String, priority: String, status: String, due_date: String) -> TodoItem {
        TodoItem { title, description, priority, status, due_date }
    }
}
