use std::io;

use icfpc2024::token;

/// Usage: `cargo run --bin decode <<<'SB%,,/}Q/2,$_'`
fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("{:?}", token::decode_token_stream(input.trim())?);

    Ok(())
}
