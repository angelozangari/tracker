use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use tokio_postgres::{Client, Error, NoTls};

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

async fn init_db(client: &Client) -> Result<(), Error> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id          SERIAL PRIMARY KEY,
            description TEXT NOT NULL,
            created_at  TEXT NOT NULL,
            done        BOOLEAN NOT NULL DEFAULT FALSE
        )",
        &[],
    ).await?;
    Ok(())
}

async fn add_task(client: &Client, description: String) -> Result<(), Error> {
    let now = Utc::now().to_rfc3339();
    client.execute(
        "INSERT INTO tasks (description, created_at, done) VALUES ($1, $2, $3)", 
        &[&description, &now, &false],
    ).await?;
    Ok(())
}

async fn list_tasks(client: &Client) -> Result<Vec<Task>, Error> {
    let rows = client.query("SELECT id, description, created_at, done FROM tasks", &[]).await?;
    let mut tasks = Vec::new();
    
    for row in rows {
        let created_at_str: String = row.get(2);
        let task = Task {
            id: row.get::<_, i32>(0) as usize,
            description: row.get(1),
            created_at: DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap()
                .with_timezone(&Utc),
            done: row.get(3),
        };
        tasks.push(task);
    }
    
    Ok(tasks)
}

async fn mark_task_done(client: &Client, id: usize) -> Result<(), Error> {
    client.execute(
        "UPDATE tasks SET done = TRUE WHERE id = $1", 
        &[&(id as i32)],
    ).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 user=postgres password=password dbname=tracker",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    init_db(&client).await.expect("failed to initialize db");

    match cli.command {
        Commands::Add {description} => {
            add_task(&client, description).await.expect("failed to insert");
        }
        Commands::List => {
            let tasks = list_tasks(&client).await.expect("failed to load");
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
            mark_task_done(&client, id).await.expect("failed to update");
            println!("Task {} marked as done!", id);
        }
    }
    
    Ok(())
}

