use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, DefaultEditor, EditMode};

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
        // TODO: 動くようになったら token::encode_string を使いたい
        let request = token::encode(&[token::Token::String(input.trim().to_owned())])?;
        println!("Sending: {request}");
        let tokens = icfpc2024::send(request).await?;
        let result = icfpc2024::eval_tokens(&tokens)?;
        println!("{}", result);
    }

    Ok(())
}
