#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[derive(Clone, Debug)] // Derive Clone to enable cloning
struct Todo {
    id: i32,
    name: String,
    description: String,
    completed: bool,
}

// Create a new todo item with default values
impl Todo {
    fn new() -> Self {
        Self {
            id: -1,
            name: String::from(""),
            description: String::from(""),
            completed: false,
        }
    }
}

// Method to render a todo item
fn show_todo(todo: &Todo) -> Element {
    rsx! {
        div {
            border: "1px solid black",
            padding: "10px",
            margin: "5px",
            b { "{todo.id}. {todo.name}" }
            br {}
            "Description: {todo.description}"
            br {}
            if todo.completed {
                i { "completed" }
            } else {
                i { "not completed" }
            }
        }
    }
}

// Add a new todo to the list
fn add_todo(todos: &mut Signal<Vec<Todo>>, todo_name: &Signal<String>, todo_description: &Signal<String>) {
    info!("add todo");

    let todo_name: String = todo_name.to_string();
    let description: String = todo_description.to_string();
    let new_todo: Todo = Todo {
        id: todos.len() as i32 + 1,
        name: todo_name,
        description: description,
        completed: false,
    };

    todos.push(new_todo);
}

// Fetch a todo by its id
fn fetch_todo_by_id(todos: &Signal<Vec<Todo>>, id: i32) -> Todo {
    let mut selected_todo: Todo = Todo::new();

    for todo in todos.iter() {
        if todo.id == id {
            selected_todo = todo.clone();
        }
    }

    selected_todo
}

#[component]
fn App() -> Element {
    let mut todos: Signal<Vec<Todo>> = use_signal(|| Vec::new());
    let mut todo_name: Signal<String> = use_signal(|| "".to_string());
    let mut todo_description: Signal<String> = use_signal(|| "".to_string());
    let mut todo_id: Signal<i32> = use_signal(|| -1);

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        if *todo_id.read() == -1 {
            div {
                h1 { "Todos: {todos.len()}" }
                for (i, todo) in todos.iter().enumerate() {
                    div {
                        onclick: {
                            move |_| {
                                println!("clicked todo: {}", i + 1);
                                todo_id.set((i + 1) as i32);
                            }
                        },
                        { show_todo(&todo) } // Render the todo
                    }
                }
            }
        }
        else {
            div {
                { 
                    let mut selected_todo = fetch_todo_by_id(&todos, *todo_id.read());
                    show_todo(&mut selected_todo)
                }
                button {
                    onclick: move |_| {
                        todo_id.set(-1);
                    },
                    "Back"
                }
            }
        }

        h3 { "Add a new todo:" }
        { "Name: " }
        br {}
        input {
            value: "{todo_name}",
            oninput: move |event| todo_name.set(event.value())
        }
        br {}
        br {}
        { "Description: " }
        br {}
        input {
            value: "{todo_description}",
            oninput: move |event| todo_description.set(event.value())
        }
        div {
            br {}
            button {
                onclick: move |_| {
                    add_todo(&mut todos, &todo_name, &todo_description);
                    todo_name.set(String::from("")); // Clear input
                    todo_description.set(String::from("")); // Clear input
                },
                "Add todo"
            }
        }
    }
    
}
 
