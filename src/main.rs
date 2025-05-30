use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use rusqlite::{params, Connection, Result};

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

#[derive(Debug)]
struct Task{
    id: usize,
    description: String,
    created_at: DateTime<Utc>,
    done: bool,
}

fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            created_at  TEXT NOT NULL,
            done        INTEGER NOT NULL
        )",
         [],
    )?;
    Ok(())
}

fn add_task(conn: &Connection, description: String) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO tasks (description, created_at, done) VALUES (?1, ?2, ?3)", 
        params![description, now, 0],
    )?;
    Ok(())
}

fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, created_at, done FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                .unwrap()
                .with_timezone(&Utc),
            done: row.get::<_, i32>(3)? != 0,
        })
    })?;
    Ok(task_iter.map(|r| r.unwrap()).collect())
}

fn mark_task_done(conn: &Connection, id: usize) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET done = 1 WHERE id =?1", 
        params![id],
    )?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    
    let conn = Connection::open("tracker.db").expect("failed to open db");
    init_db(&conn).expect("failed to initialize db");

    match cli.command {
        Commands::Add {description} => {
            add_task(&conn, description).expect("failed to insert");
        }
        Commands::List => {
            let tasks = list_tasks(&conn).expect("failed to load");
            for task in tasks {
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
            mark_task_done(&conn, id).expect("failed to update");
            println!("Task {} marked as done!", id);
        }
    }
}

