use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use clap::Parser;
use csv::Writer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Validator vote pubkey
    #[clap(short, long)]
    vote_pubkey: String,
    start_date: String,
}

fn main() {
    let args = Args::parse();
    let cluster_url = "https://api.mainnet-beta.solana.com/".to_string();
    let rpc = RpcClient::new(cluster_url);
    let mut wtr = Writer::from_path("validator_payments.csv").unwrap();

    let mut next_epoch: Option<u64> = None;

    loop {
        let inflation_reward = rpc
            .get_inflation_reward(
                &[Pubkey::from_str(args.vote_pubkey.as_ref()).unwrap()],
                next_epoch,
            )
            .unwrap();
        let inflation_data = inflation_reward.first().unwrap().clone().unwrap();
        next_epoch = Some(inflation_data.epoch - 1);

        let slot_time = rpc.get_block_time(inflation_data.effective_slot).unwrap();

        let naive_datetime = NaiveDateTime::from_timestamp(slot_time, 0);
        if naive_datetime.date() < NaiveDate::parse_from_str(&args.start_date, "%Y-%m-%d").unwrap()
        {
            break;
        }
        let amount_in_sol = inflation_data.amount as f64 / 1e9 as f64;

        println!(
            "{}",
            format!("Received {} SOL at {}", amount_in_sol, naive_datetime)
        );

        // TokenTax format
        wtr.write_record(&[
            "Mining",
            &amount_in_sol.to_string(),
            "SOL",
            "0",
            "",
            "0",
            "",
            "",
            "",
            "Validator fees",
            &naive_datetime.format("%Y-%m-%d %H:%M").to_string(),
        ])
        .unwrap();
        wtr.flush().unwrap();
    }
}
