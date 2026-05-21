use clap::Parser;
use post_generate::Post;
use command::{Cli, Commands};

mod post_generate;
mod db_insert;
mod command;
mod set_credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir().unwrap().join(".gsconfig");
    dotenvy::from_path(config_path).ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Login { display_name, token } => {
            set_credentials::set_credentials(display_name, token).await?;
        }
        Commands::Post { message, filepath, line_start, line_end } => {
            let creator_id: i32 = std::env::var("USER_TOKEN")
                .expect("Not logged in")
                .parse()
                .expect("USER_TOKEN is not a valid number");

            let post = Post::from(message, filepath, line_start - 1, line_end - 1, creator_id)?;
            db_insert::upload_post(&post).await?;
        }
    }

    Ok(())
}