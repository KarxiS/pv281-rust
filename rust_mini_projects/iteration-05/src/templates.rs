use askama::Template;
// 1. Import the Todo struct from the repository module
// Hint: use crate::repository::Todo;
use crate::repository::Todo;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    // 2. Add a field 'todos' that will hold a vector of Todo items
    // Hint: pub todos: Vec<Todo>,
    pub todos: Vec<Todo>,
}

// TODO: Feel free to delete this template. This is just an example.
#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    pub name: String,
}

#[derive(Template)]
#[template(path = "todo_list.html")]
pub struct TodoListTemplate {
    pub todos: Vec<Todo>,
}

// TODO: Add your templates here.
