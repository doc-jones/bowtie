use std::io;

use clap::{Parser, Subcommand};
use serde_json::Result;

mod lib;
use crate::lib::Case;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run bowtie on some provided test cases
    Run {
        /// Run all known implementations
        #[clap(short, long, action)]
        all: bool,
    },

    /// Dump the schema used for communicating with bowtie
    ShowSchema {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ShowSchema {} => show_schema(),
        Commands::Run { all: _ } => run().expect("Failed to run!"),
    };
}

fn show_schema() -> () {
    let schema = include_str!("../io-schema.json");
    print!("{schema}");
}

#[tokio::main]
async fn run() -> Result<()> {
    let lines = io::stdin().lines();
    for line in lines {
        let case: Case = serde_json::from_str(&line.unwrap())?;
        run_case(case);
    }
    Ok(())
}

fn run_case(case: Case) {
    for test in case.tests {
        let valid = match test.valid {
            Some(true) => format!(" (valid)"),
            Some(false) => format!(" (invalid)"),
            None => format!(""),
        };
        println!(
            "{} > {}: {} / {}{}",
            case.description, test.description, case.schema, test.instance, valid,
        );
    }
}
