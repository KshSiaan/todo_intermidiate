use std::{fs, path};

use chrono::prelude::*;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

const GREEN: &str = "\x1b[32m";
const DIM: &str = "\x1b[2m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const STRIKE: &str = "\x1b[9m";
const RESET: &str = "\x1b[0m";

#[derive(Serialize, Deserialize, Debug)]
struct TODO {
    id: String,
    title: String,
    done: bool,
    updated_at: String,
    created_at: String,
}

fn get_input() -> TODO {
    let mut input_data = TODO {
        id: nanoid!(),
        title: String::new(),
        done: false,
        updated_at: Utc::now().to_string(),
        created_at: Utc::now().to_string(),
    };

    println!("Enter TODO title: ");
    std::io::stdin()
        .read_line(&mut input_data.title)
        .expect("Something went wrong while taking Name");

    input_data.title = input_data.title.trim().to_string();

    input_data
}

fn get_todos_from_file() -> Vec<TODO> {
    let file = fs::read_to_string("data.json").unwrap_or_default();

    let todos: Vec<TODO> = serde_json::from_str(&file).unwrap_or_default();

    todos
}

fn print_todos(todos: &[TODO]) {
    println!();
    println!("{BOLD}{CYAN}📋 Your Todos{RESET}");
    println!("{DIM}{}{RESET}", "─".repeat(40));

    if todos.is_empty() {
        println!();
        println!("  ✨ All clear! Nothing here yet.");
        println!("{DIM}  Add a task to get started.{RESET}");
        println!();
        return;
    }

    let width = todos.len().to_string().len();

    for (i, todo) in todos.iter().enumerate() {
        if todo.done {
            println!(
                "  {DIM}{:>width$}.{RESET} {GREEN}✔{RESET} {DIM}{STRIKE}{}{RESET}",
                i + 1,
                todo.title
            );
        } else {
            println!("  {:>width$}. ☐ {}", i + 1, todo.title);
        }
    }

    let done = todos.iter().filter(|t| t.done).count();
    let total = todos.len();
    let bar_len = 20;
    let filled = done * bar_len / total;

    println!("{DIM}{}{RESET}", "─".repeat(40));
    println!(
        "  [{GREEN}{}{RESET}{DIM}{}{RESET}] {done}/{total} done",
        "█".repeat(filled),
        "░".repeat(bar_len - filled),
    );
    println!();
}

fn main() {
    if !path::Path::new("data.json").exists() {
        match fs::write("data.json", "[]") {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to create data file: {}", e);
                return;
            }
        }
    }

    let mut todos: Vec<TODO> = get_todos_from_file();

    loop {
        println!("1. Add TODO");
        println!("2. List TODOs");
        println!("3. Mark TODO as done");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let todo = get_input();
                todos.push(todo);
                let json = serde_json::to_string(&todos).expect("Failed to serialize todos");
                match fs::write("data.json", json) {
                    Ok(_) => println!("TODO added successfully!"),
                    Err(e) => println!("Failed to write to file: {}", e),
                }
            }
            "2" => {
                print_todos(&todos);
            }
            "3" => {
                let json = serde_json::to_string(&todos).expect("Failed to serialize todos");

                let mut todos: Vec<TODO> =
                    serde_json::from_str(&json).expect("Failed to parse todos");

                println!("Enter the id of the TODO to mark as done: ");

                let mut id = String::new();
                std::io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read input");

                let index = todos
                    .iter()
                    .position(|todo| todo.id == id.trim())
                    .expect("SOmething went wrong while taking input");

                todos[index].done = true;

                let json = serde_json::to_string(&todos).expect("Failed to serialize todos");
                match fs::write("data.json", json) {
                    Ok(_) => println!("TODO marked as done successfully!"),
                    Err(e) => println!("Failed to write to file: {}", e),
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
