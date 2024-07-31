#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Debug)] // Derive Clone to enable cloning
struct Todo {
    index: i32,
    name: String,
    description: String,
    completed: bool,
}

// Create a new todo item with default values
impl Todo {
    fn new() -> Self {
        Self {
            index: -1,
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
            background: "aliceblue",
            b { "{todo.index}. {todo.name}" }
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
        index: todos.len() as i32 + 1,
        name: todo_name,
        description: description,
        completed: false,
    };

    todos.push(new_todo);
}

// Fetch a todo by its index
fn fetch_todo_by_index(todos: &Signal<Vec<Todo>>, index: i32) -> Todo {
    let mut selected_todo: Todo = Todo::new();

    for todo in todos.iter() {
        if todo.index == index {
            selected_todo = todo.clone();
        }
    }

    selected_todo
}

// Reassign indexes 
fn reassign_indexes(todos: &mut Vec<Todo>) {
    for (index, todo) in todos.iter_mut().enumerate() {
        todo.index = index as i32 + 1;
    }
}

#[component]
fn AddTodoForm(todos: Signal<Vec<Todo>>, todo_name: Signal<String>, todo_description: Signal<String>) -> Element {
    rsx! {
        div {
            border: "1px solid black",
            padding: "10px",
            margin: "5px",
            b { "Add a new todo:" }
            br {}
            { "Name: " }
            br {}
            input {
                value: "{todo_name}",
                oninput: move |event| todo_name.set(event.value())
            }
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
}

#[component]
fn TodoList(todos: Signal<Vec<Todo>>, todo_index: Signal<i32>) -> Element {
    rsx! {
        div {
            if todos.len() == 0 {
                h1 { margin: "5px", "No todos yet" }
            } else {
                h1 { margin: "5px", "Todos: {todos.len()}" }
                b { margin: "5px", "Click on a todo to view details and edit" }
                for (i, todo) in todos.iter().enumerate() {
                    div {
                        onclick: {
                            move |_| {
                                println!("clicked todo: {}", i + 1);
                                todo_index.set((i + 1) as i32);
                            }
                        },
                        { show_todo(&todo) } // Render the todo
                    }
                }
            }
        }
    }
}

#[component]
fn EditTodo(todos: Signal<Vec<Todo>>, todo_index: Signal<i32>) -> Element {
    let mut selected_todo = fetch_todo_by_index(&todos, *todo_index.read());
    let mut todo_name = use_signal(|| selected_todo.name.clone());
    let mut todo_description = use_signal(|| selected_todo.description.clone());

    rsx! {
        div {
            { show_todo(&mut selected_todo) } 
        }
        div {
            button {
                margin: "5px",
                onclick: {
                    let todo_index = todo_index.clone();
                    move |_| {
                        let mut todos_vec = todos.write();
                        if let Some(todo) = todos_vec.iter_mut().find(|todo| todo.index == *todo_index.read()) {
                            todo.completed = !todo.completed;
                        }
                    }
                },
                "Toggle completed"
            }
        }
        br {}
        br {}
        div {
            border: "1px solid black",
            padding: "10px",
            margin: "5px",
            b { "Edit todo" }
            br {}
            { "Name: " }
            input {
                value: "{todo_name}",
                oninput: move |event| todo_name.set(event.value())
            }
            br {}
            { "Description: " }
            input {
                value: "{todo_description}",
                oninput: move |event| todo_description.set(event.value())
            }
            br {}
            button {
                margin_right: "10px",
                onclick: {
                    move |_| {
                        let mut todos_vec = todos.write();
                        if let Some(todo) = todos_vec.iter_mut().find(|todo| todo.index == *todo_index.read()) {
                            selected_todo.name = todo_name.read().to_string();
                            selected_todo.description = todo_description.read().to_string();
                            *todo = selected_todo.clone();
                        }
                        todo_index.set(-1); // Go back to the main view
                    }
                },
                "Save"
            }
            button {
                onclick: {
                    let mut todo_index = todo_index.clone();
                    move |_| {
                        let mut todos = todos.write();
                        todos.retain(|todo| todo.index != *todo_index.read());
                        reassign_indexes(&mut todos);
                        todo_index.set(-1);
                    }
                },
                "Delete"
            }
        }
    }
}

fn main() {    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let todos: Signal<Vec<Todo>> = use_signal(|| Vec::new());
    let todo_name: Signal<String> = use_signal(|| "".to_string());
    let todo_description: Signal<String> = use_signal(|| "".to_string());
    let todo_index: Signal<i32> = use_signal(|| -1);

    rsx! {
        link { rel: "stylesheet", href: "../assets/main.css" }
        div {
            class: "app-container",
            if *todo_index.read() == -1 {
                TodoList { todos, todo_index }
                br {}
                AddTodoForm { todos, todo_name, todo_description } 
            } else {
                EditTodo { todos, todo_index }
            }
        }
    }
}
 
