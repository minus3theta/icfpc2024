use std::path::PathBuf;

use anyhow::{Context, Ok};
use clap::{Parser, Subcommand};
use icfpc2024::token;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct IcfpcCli {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Subcommand)]
enum Command {
    Submit {
        /// Path to the output file
        /// e.g. data/spaceship/spaceship1.kaku.out
        #[arg(short, long)]
        output: PathBuf,
        /// Read raw tokens
        #[arg(short, long)]
        raw: bool,
        /// Save post payload to file
        #[arg(short, long)]
        save: Option<PathBuf>,
    },
}

async fn submit_solution(
    output: PathBuf,
    raw: bool,
    save: Option<PathBuf>,
) -> anyhow::Result<String> {
    let text = std::fs::read_to_string(&output)?;

    let problem_file_name = output
        .file_name()
        .context("Expected file name")?
        .to_string_lossy();
    let problem_name = problem_file_name
        .split('.')
        .next()
        .context("Expected file name")?;
    let output_file_name = output.to_string_lossy().to_string();
    println!("problem_name: {}", problem_name);

    let request = if raw {
        text
    } else {
        let mut tokens = vec![
            token::Token::BinaryOp(token::BinaryOp::Concat),
            token::Token::String(format!("solve {problem_name} ")),
        ];
        tokens.extend(token::encode_string(&text)?);

        token::encode(&tokens)?
    };

    if let Some(save) = save {
        std::fs::write(save, &request)?;
    }
    // println!("{}", request);
    eprintln!("Submitting '{output_file_name}' for '{problem_name}' to the server...");
    let tokens = icfpc2024::send(request).await?;
    let result = icfpc2024::eval_tokens(&tokens)?;
    Ok(result)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = IcfpcCli::parse();
    match cli.commands {
        Command::Submit { output, raw, save } => {
            let result = submit_solution(output, raw, save).await?;
            println!("{}", result);
        }
    }
    Ok(())
}
