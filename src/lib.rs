use std::env;

use anyhow::bail;
use ast::{Expr, Value};
use dotenv::dotenv;
use token::Token;

pub mod assemble;
pub mod ast;
pub mod token;

pub const ENDPOINT: &str = "https://boundvariable.space/communicate";

pub async fn send(encoded: String) -> anyhow::Result<Vec<Token>> {
    dotenv().ok();
    let response = reqwest::Client::new()
        .post(ENDPOINT)
        .bearer_auth(env::var("TOKEN")?)
        .body(encoded)
        .send()
        .await?;
    let result = response.error_for_status()?.text().await?;
    if result.is_empty() {
        bail!(
            "You might have reached the rate limit 20 requests per minute. Please try again later."
        )
    }
    let tokens = token::decode_token_stream(&result)?;
    Ok(tokens)
}

pub fn eval_tokens(tokens: &[Token]) -> anyhow::Result<String> {
    let result = match Expr::from_tokens(tokens)?.eval()? {
        Value::String(s) => s,
        value => value.to_string(),
    };
    Ok(result)
}
