use icfpc2024::ast::{Expr, Value};
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, DefaultEditor, EditMode};
use std::env;

use icfpc2024::token;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::with_config(
        Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build(),
    )?;
    let client = reqwest::Client::new();
    loop {
        let input = match rl.readline("❯ ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                line
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                return Err(err.into());
            }
        };

        let request = token::encode(&[token::Token::String(input.trim().to_owned())])?;
        let response = client
            .post(icfpc2024::ENDPOINT)
            .bearer_auth(env::var("TOKEN")?)
            .body(request)
            .send()
            .await?;
        let text = response.text().await?;
        let tokens = token::decode_token_stream(&text)?;
        match Expr::from_tokens(&tokens)?.eval()? {
            Value::String(s) => println!("{s}"),
            value => println!("{value}"),
        }
    }

    Ok(())
}
