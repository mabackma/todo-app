#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_elements::br;
use dioxus_logger::tracing::{info, Level};

fn main() {    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[derive(Clone)] // Derive Clone to enable cloning
struct Todo {
    id: i32,
    text: String,
    description: String,
    completed: bool,
}

impl Todo {
    // Method to toggle the completed status of a todo
    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    // Method to render a todo item
    fn show_todo(&self) -> Element {
        rsx! {
            div {
                //onclick: move |_| self.toggle_completed(),
                border: "1px solid black",
                padding: "10px",
                margin: "5px",
                b { "{self.id}. {self.text}" }
                br {}
                "Description: {self.description}"
                br {}
                if self.completed {
                    i { "completed" }
                } else {
                    i { "not completed" }
                }
            }
        }
    }
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
            div {
                for todo in todos.iter() {
                    { todo.show_todo() }
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
    
}
