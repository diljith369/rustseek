use std::error::Error;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct OllamaRequest {
        model: String,
        prompt: String,
        stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
        response: String,
}

async fn ollama_query(prompt: &str) -> Result<String, Box<dyn Error>> {

    let client = Client::new();
    let request = OllamaRequest {
        model: "deepseek-r1".to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };


    let res: OllamaResponse = client.post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    Ok(res.response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "Once upon a time";
    let response = ollama_query(prompt).await?;
    println!("{}", response);
    Ok(())
}
