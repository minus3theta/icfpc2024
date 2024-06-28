use std::env;
use std::io;

use icfpc2024::token;
use itertools::Itertools;

const ENDPOINT: &str = "https://boundvariable.space/communicate";

#[tokio::main]
async fn main() -> anyhow::Result<()> {    
    let client = reqwest::Client::new();
    let problems = ["lambdaman", "spaceship"];

    // loop {
    //     input.clear();
    //     eprint!("❯ ");
    //     if io::stdin().read_line(&mut input)? == 0 {
    //         break;
    //     }
    //     let request = token::encode(&[token::Token::String(input.trim().to_owned())])?;
    //     let response = client
    //         .post(ENDPOINT)
    //         .bearer_auth(env::var("TOKEN")?)
    //         .body(request)
    //         .send()
    //         .await?;
    //     let text = response.text().await?;
    //     for token in &token::decode_token_stream(&text)? {
    //         println!("{}", token);
    //     }
    // }

    for problem in problems.iter() {        
        let problem_command = format!("get {}", problem);
        let request = token::encode(&[token::Token::String(problem_command)])?;
        let response = client
            .post(ENDPOINT)
            .bearer_auth(env::var("TOKEN")?)
            .body(request)
            .send()
            .await?;
        let text = response.text().await?;        
        let problem_statement = token::decode_token_stream(&text)?.into_iter().join("");
        for x in 1..{
            // e.g. lambdaman1
            let task = format!("{problem}{x}");
            if !problem_statement.contains(&task) {
                break;
            }
            eprintln!("Downloading task: {}", task);
            let task_command = format!("get {}", task);
            let request = token::encode(&[token::Token::String(task_command)])?;
            let response = client
                .post(ENDPOINT)
                .bearer_auth(env::var("TOKEN")?)
                .body(request)
                .send()
                .await?;
            let text = response.text().await?;
            let task_input = token::decode_token_stream(&text)?.into_iter().join("");
            
            println!("Task: {}", task_input);
        }
    }

    Ok(())
}