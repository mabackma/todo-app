#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

fn add_todo(todos: &mut Signal<Vec<String>>, new_todo: &Signal<String>) {
    info!("add todo");
    let todo: String = new_todo.to_string();
    todos.push(todo);
}

#[component]
fn App() -> Element {
    let mut count: Signal<i32> = use_signal(|| 0);
    let mut todos: Signal<Vec<String>> = use_signal(|| Vec::new());
    let mut new_todo: Signal<String> = use_signal(|| String::from(""));

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div {
            h1 { "Todos: {count}" }
            ul {
                for todo in todos.iter() {
                    li { "{todo}" }
                }
            }
            h3 { "Add a new todo:" }
            input {
                value: "{new_todo}",
                oninput: move |event| new_todo.set(event.value())
            }
            div {
                br {}
                button {
                    onclick: move |_| {
                        add_todo(&mut todos, &new_todo);
                        count += 1; // Increment count
                        new_todo.set(String::from("")); // Clear input
                    },
                    "Add todo"
                }
            }
        }
    }
    
}
