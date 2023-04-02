use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const API_URL: &str = "https://api.openai.com/v1/engines/davinci-codex/completions";

#[derive(Serialize)]
struct Request {
    prompt: String,
    max_tokens: u32,
    n: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let prompt = input.trim_end();

    let client = Client::new();
    let request = Request {
        prompt: prompt.to_owned(),
        max_tokens: 50,
        n: 1,
        temperature: 0.5,
    };
    let response = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", env!("OPENAI_API_KEY")),
        )
        .json(&request)
        .send()?
        .json::<Response>()?;
    let text = response.choices[0].text.trim_end();
    println!("{}", text);

    Ok(())
}
