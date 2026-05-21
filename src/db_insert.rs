use reqwest::Client;
use serde::Serialize;
use crate::post_generate::Post;

#[derive(Serialize)]
struct PostPayload<'a> {
    message: &'a str,
    code: &'a str,
    creator_id: i32,
    pg_lang: &'a str,
}

pub async fn upload_post(post: &Post) -> Result<(), reqwest::Error> {
    let client = Client::new();
    
    client.post("http://127.0.0.1:5000/api/post")
        .json(&PostPayload {
            message: &post.message,
            code: &post.code,
            creator_id: post.creator_id,
            pg_lang: &post.pg_lang,
        })
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}