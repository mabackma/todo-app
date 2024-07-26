#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

struct Todo {
    id: i32,
    text: String,
    description: String,
    completed: bool,
}

// Add a new todo to the list
fn add_todo(todos: &mut Signal<Vec<Todo>>, todo_name: &Signal<String>, todo_description: &Signal<String>) {
    info!("add todo");

    let todo_name: String = todo_name.to_string();
    let description: String = todo_description.to_string();
    let todo_struct: Todo = Todo {
        id: todos.len() as i32 + 1,
        text: todo_name.clone(),
        description: description.clone(),
        completed: false,
    };

    todos.push(todo_struct);
}

#[component]
fn App() -> Element {
    let mut todos: Signal<Vec<Todo>> = use_signal(|| Vec::new());
    let mut todo_name: Signal<String> = use_signal(|| String::from(""));
    let mut todo_description: Signal<String> = use_signal(|| String::from(""));

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div {
            h1 { "Todos: {todos.len()}" }
            ul {
                for todo in todos.iter() {
                    if todo.completed {
                        p {
                            b { "{todo.id}. {todo.text}:" } 
                            br {}
                            " {todo.description}"
                            br {}
                            i { "completed" }
                        }
                    } else {
                        p {
                            b { "{todo.id}. {todo.text}:" } 
                            br {}
                            " {todo.description}"
                            br {}
                            i { "not completed" }
                        }
                    }
                }
            }
            h3 { "Add a new todo:" }
            input {
                value: "{todo_name}",
                oninput: move |event| todo_name.set(event.value())
            }
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
