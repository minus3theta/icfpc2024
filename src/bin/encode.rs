use std::io;

use icfpc2024::token;

/// Usage: `cargo run --bin encode <<<'12345'`
fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("{:?}", token::encode(&[token::Token::String(input.trim().to_owned())])?);

    Ok(())
}
