use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
}

fn main() {
    let matches = App::new("Simple Task Manager")
        .version("0.1")
        .author("Your Name <your.email@example.com>")
        .about("Manages a list of tasks")
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a task")
                .arg(
                    Arg::with_name("description")
                        .help("The task description")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Lists all tasks"))
        .subcommand(
            SubCommand::with_name("remove")
                .about("Removes a task by ID")
                .arg(
                    Arg::with_name("id")
                        .help("The task ID")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let mut tasks = read_tasks();

    if let Some(matches) = matches.subcommand_matches("add") {
        let description = matches.value_of("description").unwrap();
        let task = Task {
            id: tasks.len() + 1,
            description: description.to_string(),
        };
        tasks.push(task);
        save_tasks(&tasks);
    } else if let Some(_) = matches.subcommand_matches("list") {
        for task in tasks {
            println!("{}: {}", task.id, task.description);
        }
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let id = matches.value_of("id").unwrap().parse::<usize>().unwrap();
        tasks.retain(|task| task.id != id);
        save_tasks(&tasks);
    }
}

fn read_tasks() -> Vec<Task> {
    let path = Path::new("tasks.json");
    if !path.exists() {
        return vec![];
    }

    let mut file = File::open(path).expect("Unable to open tasks file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read tasks file");

    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string(tasks).expect("Unable to serialize tasks");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("tasks.json")
        .expect("Unable to open tasks file");
    file.write_all(data.as_bytes()).expect("Unable to write tasks file");
}
