use icfpc2024::{eval_tokens, send, token};
use itertools::Itertools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let problems = ["lambdaman", "spaceship", "3d"];
    for problem in problems.iter() {
        eprintln!("Downloading problem: {}", problem);
        let problem_command = format!("get {}", problem);
        let request = token::encode(&[token::Token::String(problem_command)])?;
        let tokens = send(request).await?;
        let text = eval_tokens(&tokens)?;
        let problem_statement = token::decode_token_stream(&text)?.into_iter().join("");
        for x in 1.. {
            // e.g. lambdaman1
            let task = format!("{problem}{x}");
            if !problem_statement.contains(&task) {
                break;
            }
            eprintln!("Downloading task: {}", task);
            let task_command = format!("get {}", task);
            let request = token::encode(&[token::Token::String(task_command)])?;
            let tokens = send(request).await?;
            let text = eval_tokens(&tokens)?;
            // data/{problem}/{task}.raw
            // data/{problem}/{task}.in
            let task_raw_file = format!("data/{problem}/{task}.raw");
            std::fs::write(&task_raw_file, text.clone())?;
            eprintln!("Task Raw data saved to: {}", task_raw_file);
            let task_file = format!("data/{problem}/{task}.in");
            if let Ok(text) =
                token::decode_token_stream(&text).map(|tokens| tokens.into_iter().join(""))
            {
                std::fs::write(&task_file, text)?;
                eprintln!("Task data saved to: {}", task_file);
            } else {
                eprintln!("Failed to save task data to: {}", task_file);
            }
            // sleep 4 second
            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
        }
    }

    Ok(())
}
