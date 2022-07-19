use std::io;

use bollard::{
    container::{Config, StartContainerOptions},
    models::HostConfig,
    Docker,
};
use clap::{Parser, Subcommand};
use futures::future::join_all;
use serde_json;

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
        /// One or more JSON Schema implementations to run under bowtie.
        #[clap(short, long, action)]
        implementations: Vec<String>,
    },

    /// Dump the schema used for communicating with bowtie
    ShowSchema {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ShowSchema {} => show_schema(),
        Commands::Run { implementations } => run(implementations.to_vec()).expect("Failed to run!"),
    };
}

fn show_schema() -> () {
    let schema = include_str!("../io-schema.json");
    print!("{schema}");
}

#[tokio::main]
async fn run(implementations: Vec<String>) -> serde_json::Result<()> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let tasks = implementations
        .iter()
        .map(|image| temporary_container(&docker, &image))
        .collect::<Vec<_>>();
    for each in join_all(tasks).await {
        println!("Container: {}", each.expect("Couldn't start!"));
    }

    for line in io::stdin().lines() {
        let case: Case = serde_json::from_str(&line.unwrap())?;
        run_case(&case, &implementations);
    }
    Ok(())
}

async fn temporary_container(
    docker: &bollard::Docker,
    image: &str,
) -> Result<String, bollard::errors::Error> {
    let config = Config {
        image: Some(image),
        host_config: Some(HostConfig {
            auto_remove: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };
    let id = docker
        .create_container::<&str, &str>(None, config)
        .await?
        .id;
    let _ = docker
        .start_container(&id, None::<StartContainerOptions<String>>)
        .await;
    Ok(id)
}

fn run_case(case: &Case, implementations: &Vec<String>) {
    for test in &case.tests {
        let expected = match test.valid {
            Some(true) => format!(" (valid)"),
            Some(false) => format!(" (invalid)"),
            None => format!(""),
        };
        let results = implementations
            .iter()
            .map(|_| "valid")
            .collect::<Vec<_>>()
            .join(", ");
        println!(
            "{} > {}: {} / {}{} – {}",
            case.description, test.description, case.schema, test.instance, expected, results,
        );
    }
}
