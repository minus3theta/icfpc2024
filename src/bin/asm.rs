use std::io::{self, Read};

use icfpc2024::assemble::assemble;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens = assemble(&input)?;
    print!("{}", tokens.join(" "));

    Ok(())
}
