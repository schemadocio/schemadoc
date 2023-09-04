pub mod alerts;
pub mod app_state;
pub mod branches;
pub mod constants;
pub mod datasources;
pub mod dependencies;
pub mod models;
pub mod persistence;
pub mod scheduler;
pub mod settings;
pub mod storage;
pub mod versions;
pub mod web;

use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(disable_help_flag = true)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run server
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(short, long, default_value = "0.0.0.0")]
        host: String,
        #[arg(short, long, default_value = "9753")]
        port: u16,
        #[arg(short, long, action)]
        schedule: bool,
    },

    Schedule {
        #[arg(short, long, default_value = "0.0.0.0")]
        host: String,
        #[arg(short, long, default_value = "9753")]
        port: u16,
        #[arg(short, long, action)]
        force: bool,
        #[arg(short, long, default_value = "1")]
        interval: u64,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap() {
        Commands::Serve {
            host,
            port,
            schedule,
        } => {
            tokio::select! {
                r = web::serve(&host, port) => {
                    println!("Server exited: {:?}", r)
                }
                r = scheduler::schedule("localhost", port, 1, false), if schedule => {
                    println!("Scheduler exited: {:?}", r)
                }
            }
            Ok(())
        }
        Commands::Schedule {
            host,
            port,
            interval,
            force,
        } => scheduler::schedule(&host, port, interval, force).await,
    }
}
