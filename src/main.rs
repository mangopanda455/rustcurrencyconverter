use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;
use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    amnt: u32,
    origin: String,
    fin: String,
}
#[derive(Debug, Deserialize)]
struct quer {
    from: String,
    to: String,
    amount: u32,
}
#[derive(Debug, Deserialize)]
struct inf {
    timestamp: u64,
    rate: f64,
}
#[derive(Debug, Deserialize)]
struct help {
    success: bool,
    symbols: HashMap<String, String>
}
#[derive(Debug, Deserialize)]
struct FixerResponse {
    success: bool,
    query: quer,
    info: inf,
    date: String,
    result: f64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amnt = Cli::parse().amnt;
    let origin = Cli::parse().origin;
    let fin = Cli::parse().fin;
    let url = format!("https://api.apilayer.com/fixer/convert?from={}&to={}&amount={}", origin, fin, amnt);
    let client = reqwest::Client::new();
    if origin == "help" {
        let res = client
            .get("https://api.apilayer.com/fixer/symbols")
            .header("apikey", "xptI7I3A2fLNbX4RyEIs6ZMxKo52cn1X")
            .send()
            .await?
            .json::<help>()
            .await?;
        println!("{:#?}", res.symbols);
    }
    let res = client
        .get(&url)
        .header("apikey", "xptI7I3A2fLNbX4RyEIs6ZMxKo52cn1X")
        .send()
        .await?
        .json::<FixerResponse>()
        .await?;

    let converted = res.result;
    println!("{}", converted);
    Ok(())
}

