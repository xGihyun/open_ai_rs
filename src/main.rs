use reqwest::Error;
use serde_json::json;
use serde::Serialize;
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use std::io::{stdin, stdout, Write};
use spinners::{Spinner, Spinners};

#[derive(Debug, Deserialize, Serialize)]
struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: i32
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();

    let client = reqwest::Client::new();
    let rapid_api_key = env::var("RAPID_API_KEY").expect("Missing Rapid API key");
    
    loop{
        print!("> ");
        stdout().flush().unwrap();
        
        let mut user_input = String::new();
        
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        println!();

        let mut sp = Spinner::new(Spinners::Dots12, "\t Generating response...".into());
        let res = client
            .post("https://openai80.p.rapidapi.com/chat/completions")
            .header("Content-Type", "application/json")
            .header("X-RapidAPI-Key", &rapid_api_key)
            .header("X-RapidAPI-Host", "openai80.p.rapidapi.com")
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [
                    {
                        "role": "user",
                        "content": user_input
                    }
                ]
            }))
            .send()
            .await?
            .text()
            .await?;
    
        let chat_completion: ChatCompletion = serde_json::from_str(&res).unwrap();

        sp.stop();
    
        let ai_response = &chat_completion.choices[0].message.content;
        
        println!("\n");
        println!("{}", ai_response);
        println!();

        // break;
    }

    // Unreachable since loop isn't broken in code
    Ok(())

}