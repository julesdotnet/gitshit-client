use std::env;
use sqlx::{MySqlPool, Row, mysql::MySqlPoolOptions};
use std::fs;

pub async fn set_credentials(display_name: String, token: i32) -> Result<(), sqlx::Error> {
    let env_path = dirs::home_dir().unwrap().join(".env");
    dotenvy::from_path(env_path).ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    if verify_user(&pool, &display_name, token).await? {
        let config = format!("DISPLAY_NAME={}\nUSER_TOKEN={}", display_name, token);
        let config_path = dirs::home_dir().unwrap().join(".gsconfig");
        fs::write(config_path, config).expect("Failed to save credentials");
    } else {
        eprintln!("Invalid credentials");
    }

    Ok(())
}

pub async fn verify_user(pool: &MySqlPool, display_name: &str, token: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "SELECT COUNT(*) as count FROM users WHERE display_name = ? AND token = ?"
    )
    .bind(display_name)
    .bind(token)
    .fetch_one(pool)
    .await?;

    let count: i64 = result.try_get("count")?;
    Ok(count > 0)
}