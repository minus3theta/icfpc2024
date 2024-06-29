use std::env;
use std::io;

use icfpc2024::token;

const ENDPOINT: &str = "https://boundvariable.space/communicate";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    let client = reqwest::Client::new();
    loop {
        input.clear();
        eprint!("❯ ");
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        let request = token::encode_string(input.trim())?;
        let response = client
            .post(ENDPOINT)
            .bearer_auth(env::var("TOKEN")?)
            .body(request)
            .send()
            .await?;
        let text = response.text().await?;
        for token in &token::decode_token_stream(&text)? {
            println!("{}", token);
        }
    }

    Ok(())
}
