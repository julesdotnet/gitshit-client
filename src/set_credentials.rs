use std::fs;
use serde_json::json;

pub async fn set_credentials(display_name: String, token: String) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://gitshit.onrender.com/api/login")
        .json(&json!({
            "display_name": display_name,
            "token": token
        }))
        .send()
        .await?;

    if res.status().is_success() {
        let config = format!("DISPLAY_NAME={}\nUSER_TOKEN={}", display_name, token);
        let config_path = dirs::home_dir().unwrap().join(".gsconfig");
        fs::write(config_path, config).expect("Failed to save credentials");
        println!("Logged in as {}", display_name);
    } else {
        eprintln!("Invalid credentials");
    }

    Ok(())
}