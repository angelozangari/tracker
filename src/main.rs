use chrono::{Utc, DateTime};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;

const DB_FILE: &str = "tasks.json";

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done { id: usize },
}
#[derive(Parser)]
#[command(name = "tracker")]
#[command(about = "A simple task tracker in Rust", long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task{
    id: usize,
    description: String,
    created_at: DateTime<Utc>,
    done: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add {description} => {
            let task = Task {
                id: tasks.len() + 1,
                description,
                created_at: Utc::now(),
                done: false,
            };
            tasks.push(task);
            save_tasks(&tasks);
        }
        Commands::List => {
            for task in &tasks {
                println!(
                    "{}. [{}] {} (created {})",
                    task.id,
                    if task.done {"x"} else {" "},
                    task.description,
                    task.created_at
                );
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id){
                task.done = true;
                save_tasks(&tasks);
                println!("Task {} marked as done!", id);
            } else {
                println!("Task not found");
            }
        }
    }
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(DB_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &[Task]) {
    let content = serde_json::to_string_pretty(tasks).expect("Failed to serialize");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(DB_FILE)
        .expect("Failed to open file");
    file.write_all(content.as_bytes()).expect("Failed to write file");
}