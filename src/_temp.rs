use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    messages: Vec<Message>,
    model: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    id: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize, Debug)]
struct MessageContent {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let output_path = "./data/output".to_string();
    // Retrieve the API key from environment variables
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");

    // Prepare the request payload
    let request = ChatRequest {
        messages: vec![Message {
            role: "user".to_string(),
            content: "Explain the importance of fast language models".to_string(),
        }],
        model: "llama3-8b-8192".to_string(),
    };

    // Create a reqwest client
    let client = Client::new();

    // Make the POST request
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    // Check if the response is successful
    if response.status().is_success() {
        let result: ChatResponse = response.json().await?;
        // Save result to output.txt
        let output_file = format!("{}/output.txt", output_path);
        std::fs::write(output_file, format!("{:#?}", result)).expect("Unable to write file");
        println!("{:#?}", result);
    } else {
        eprintln!("Request failed: {}", response.status());
        let error_text = response.text().await?;
        eprintln!("Error details: {}", error_text);
    }

    Ok(())
}
