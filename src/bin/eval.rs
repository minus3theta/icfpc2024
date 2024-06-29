use std::io;

use icfpc2024::{ast::Expr, token};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let tokens = token::decode_token_stream(input.trim())?;
    let ast = Expr::from_tokens(&tokens)?;
    eprintln!("{}", &ast);
    println!("{}", ast.eval()?);

    Ok(())
}
