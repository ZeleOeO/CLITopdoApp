use clap::{Parser, Subcommand};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, ContentArrangement, Table};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Parser)]
#[command(name = "todo_app")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String, description: String },
    List,
    Close,
    Delete { delete_index: i32 },
}

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    name: String,
    description: String,
}

fn main() {
    let args: Args = Args::parse();

    match &args.command {
        Commands::Add { title, description } => {
            println!("Adding Item To Todo");
            println!("Title: {} Description: {}", title, description);
            let todo = Todo {
                name: title.to_string(),
                description: description.to_string(),
            };
            save_to_file(&todo);
        }
        Commands::List => {
            println!("Printing Todo List");
            read_from_file("todo.json");
        }
        Commands::Close => {
            println!("Ending Session");
        }
        Commands::Delete { delete_index } => {
            delete_todo("todo.json", *delete_index);
        }
    }
}

fn save_to_file(t: &Todo) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.json")
        .unwrap();

    let json = serde_json::to_string(t).unwrap();

    match writeln!(file, "{}", json) {
        Ok(()) => println!("Save successful"),
        Err(e) => println!("Failed to process: {}", e),
    }
}

fn read_from_file(file_name: &str) {
    let f = File::open(file_name).expect("Error opening File");
    let reader = BufReader::new(f);
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40)
        .set_header(vec!["S/N", "Name", "Description"]);

    for (index, line) in reader.lines().into_iter().enumerate() {
        let line = line.expect("Failed to read line");
        let todo: Todo = match serde_json::from_str(&line) {
            Ok(todo) => todo,
            Err(e) => {
                println!("Failed to parse line {}. Error is {} ", line, e);
                continue;
            }
        };

        table.add_row(vec![
            Cell::new(index + 1),
            Cell::new(todo.name),
            Cell::new(todo.description),
        ]);
    }
    println!("{table}");
}

fn delete_todo(file_name: &str, delete_index: i32) {
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let lines: Vec<String> = reader
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.ok()?;
            if (i as i32) + 1 != delete_index {
                Some(line)
            } else {
                None
            }
        })
        .collect();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .unwrap();

    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

    println!("Deleted todo at line {} ", delete_index);
}
