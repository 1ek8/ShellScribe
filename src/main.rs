use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};
use dotenvy::dotenv;

mod tools;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("CC_KEY").unwrap_or_else(|_| {
        eprintln!("CC_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let tools = tools::all_tools();

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
            "model": "z-ai/glm-4.5-air:free",
            "tools": tools
        }))
        .await?;

    eprintln!("Logs from program will appear here!");

    match response["choices"][0]["message"]["content"].as_str() {
        Some(content) => println!("{}", content),
        None => eprintln!("Warning: Received an empty message from the model."),
    }

    Ok(())
}
