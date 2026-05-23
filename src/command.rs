use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Login {
        #[arg(short, long)]
        display_name: String,
        #[arg(short, long)]
        token: String,
    },
    Post {
        #[arg(short, long)]
        message: String,
        #[arg(short, long)]
        filepath: String,
        #[arg(short, long)]
        line_start: i32,
        #[arg(short, long)]
        line_end: i32,
    },
}