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
    text: String,
    description: String,
    completed: bool,
}

impl Todo {
    fn mark_completed(&mut self) {
        self.completed = true;
    }

    // Method to render a todo item
    fn show_todo(&self) -> Element {
        rsx! {
            div {
                
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
    let mut new_todo: Todo = Todo {
        id: todos.len() as i32 + 1,
        text: todo_name.clone(),
        description: description.clone(),
        completed: false,
    };

    todos.push(new_todo);
}

#[component]
fn App() -> Element {
    let mut todos: Signal<Vec<Todo>> = use_signal(|| Vec::new());
    let mut todo_name: Signal<String> = use_signal(|| "".to_string());
    let mut todo_description: Signal<String> = use_signal(|| "".to_string());

    // Function to toggle completed state of a todo
    let mut toggle_todo = move |id: i32| {
        println!("clicked todo: {}", id);
        let mut todos = todos.get_mut(id as usize);
        for todo in todos.iter_mut() {
            if todo.id == id {
                todo.completed = !todo.completed;
            }
        }
    };

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div {
            h1 { "Todos: {todos.len()}" }

            div {
                for todo in todos.iter() {
                    div {
                        onclick: move |_| toggle_todo(21341234), // toggle_todo(todo.id), FIND A WAY TO PASS THE ID
                        { todo.show_todo() } // Render the todo
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
    
}
