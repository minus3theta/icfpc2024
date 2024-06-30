use std::path::PathBuf;

use anyhow::{Context, Ok};
use clap::{Parser, Subcommand, ValueEnum};
use icfpc2024::token::{self, encode_string};

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
        #[arg(short, long)]
        task: Task,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum Task {
    // lambdaman
    Lambdaman,
    // spaceship
    Spaceship,
    // 3d
    Threed,
    // efficieny
    Efficieny,
}

async fn solve_spaceship(output: PathBuf) -> anyhow::Result<String> {
    let text = std::fs::read_to_string(output.clone())?;

    let problem_file_name = output
        .file_name()
        .context("Expected file name")?
        .to_string_lossy();
    let problem_name = problem_file_name
        .split('.')
        .next()
        .context("Expected file name")?;
    let output_file_name = output.to_string_lossy().to_string();
    eprintln!("problem_name: {}", problem_name);
    let tokens = [
        token::Token::BinaryOp(token::BinaryOp::Concat),
        token::Token::String(format!("solve {problem_name} ")),
        // TODO(togatoga): Bigintを使ってうまく圧縮したい
        token::Token::String(text),
    ];

    let request = token::encode(&tokens)?;
    // println!("{}", request);
    eprintln!("Submitting '{output_file_name}' for '{problem_name}' to the server...");
    let tokens = icfpc2024::send(request).await?;
    let result = icfpc2024::eval_tokens(&tokens)?;
    Ok(result)
}

async fn solve_lambdaman(output: PathBuf) -> anyhow::Result<String> {
    let text = std::fs::read_to_string(output.clone())?;

    let problem_file_name = output
        .file_name()
        .context("Expected file name")?
        .to_string_lossy();
    let problem_name = problem_file_name
        .split('.')
        .next()
        .context("Expected file name")?;
    let output_file_name = output.to_string_lossy().to_string();
    let cmd = format!("solve {problem_name} {text}");
    eprintln!("problem_name: {}", problem_name);
    // lambdamanに高速なやつ
    let tokens = encode_string(&cmd)?;
    let request = token::encode(&tokens)?;
    eprintln!("Submitting '{output_file_name}' for '{problem_name}' to the server...");
    let tokens = icfpc2024::send(request).await?;
    let result = icfpc2024::eval_tokens(&tokens)?;
    Ok(result)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = IcfpcCli::parse();
    match cli.commands {
        Command::Submit { output, task } => match task {
            Task::Spaceship => {
                let result = solve_spaceship(output).await?;
                println!("{}", result);
            }
            Task::Lambdaman => {
                let result = solve_lambdaman(output).await?;
                println!("{}", result);
            }
            _ => {
                eprintln!("Not implemented yet");
            }
        },
    }
    Ok(())
}
