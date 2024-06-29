use std::io;

use icfpc2024::{
    ast::{Expr, Value},
    token,
};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let tokens = token::decode_token_stream(input.trim())?;
    let ast = Expr::from_tokens(&tokens)?;
    eprintln!("{}", &ast);
    match ast.eval()? {
        Value::String(s) => println!("{s}"),
        value => println!("{value}"),
    }

    Ok(())
}
