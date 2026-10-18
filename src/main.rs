use clap::{Parser, Subcommand};
use rusqlite::{Connection, params};
use std::error::Error;

#[derive(Parser)]
#[command(name = "todo", about = "A simple Todo List CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: i64 },
    Delete { id: i64 },
}

fn main() -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    let data_dir = format!("{home}/.local/share/logbook");

    std::fs::create_dir_all(&data_dir)?;

    let db_path = format!("{data_dir}/todo.db");
    let db = Connection::open(db_path)?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            db.execute("INSERT INTO todos (title) VALUES (?1)", params![title])?;

            println!("Todo added!");
        }

        Commands::List => {
            let mut stmt = db.prepare("SELECT id, title, completed FROM todos ORDER BY id")?;

            let todos: Vec<_> = stmt
                .query_map([], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, bool>(2)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            if todos.is_empty() {
                println!("Your todo list is empty.");
            } else {
                for (id, title, completed) in todos {
                    let status = if completed { "Done" } else { "Pending" };
                    println!("{id}. [{status}] {title}");
                }
            }
        }

        Commands::Done { id } => {
            let changed =
                db.execute("UPDATE todos SET completed = 1 WHERE id = ?1", params![id])?;

            if changed > 0 {
                println!("Todo #{id} completed!");
            } else {
                println!("Todo #{id} not found.");
            }
        }

        Commands::Delete { id } => {
            let changed = db.execute("DELETE FROM todos WHERE id = ?1", params![id])?;

            if changed > 0 {
                println!("Todo #{id} deleted!");
            } else {
                println!("Todo #{id} not found.");
            }
        }
    }

    Ok(())
}
