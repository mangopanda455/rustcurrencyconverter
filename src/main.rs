use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use clap::Parser;
use colored::*;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;
#[derive(Parser)]
struct Cli {
    amnt: u32,
    origin: Option<String>,
    fin: Option<String>,
    hlp: Option<String>,
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
    if Cli::parse().amnt == 0 {
        // let helpval = Cli::parse().hlp.unwrap();
        let url = "https://api.apilayer.com/fixer/symbols";
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("apikey", "xptI7I3A2fLNbX4RyEIs6ZMxKo52cn1X")
            .send()
            .await?
            .json::<help>()
            .await?;
        let sym = res.symbols;
        println!("{:#?}", sym);
        Ok::<(), ()>(()).expect("ERROR");
    }
    // if Cli::parse().hlp.is_some() {
    //     // let helpval = Cli::parse().hlp.unwrap();
    //     let url = "https://api.apilayer.com/fixer/symbols";
    //     let client = reqwest::Client::new();
    //     let res = client
    //         .get(url)
    //         .header("apikey", "xptI7I3A2fLNbX4RyEIs6ZMxKo52cn1X")
    //         .send()
    //         .await?
    //         .json::<help>()
    //         .await?;
    //     let sym = res.symbols;
    //     println!("{:#?}", sym);
    //     Ok::<(), ()>(()).expect("ERROR");
    // }
    if Cli::parse().amnt > 0 {
        let amnt = Cli::parse().amnt;
        let origin = Cli::parse().origin.unwrap();
        let fin = Cli::parse().fin.unwrap();
        let mut sp = Spinner::new(Spinners::Dots9, "Working!".red().to_string().into());
        let url = format!("https://api.apilayer.com/fixer/convert?from={}&to={}&amount={}", origin, fin, amnt);
        let client = reqwest::Client::new(); 
        let res = client
            .get(&url)
            .header("apikey", "xptI7I3A2fLNbX4RyEIs6ZMxKo52cn1X")
            .send()
            .await?
            .json::<FixerResponse>()
            .await?;
        let converted = res.result.round();
        sp.stop();
        // println!("\n{amnt} {origin} = {} {fin}", converted);
        println!("\n\n{:^32} {:^32}\n", converted.to_string().green(), fin.to_string().green());
        println!("{} {}", "Converted from".truecolor(219, 219, 219).underline(), fin.to_string().truecolor(219, 219, 219).underline());
        Ok(())
    }
    else {
        Ok(())
    }
}

