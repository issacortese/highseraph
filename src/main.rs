use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "highseraph")]
#[command(version = "v0.1.0")]
#[command(about = "a local chaos engineering tool", long_about = None)]
struct Cli {
    /// CLI executions
    #[command(subcommand)]
    command: Commands,

    /// Sets a custom config file
    #[arg(global = true, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// K8s manifest path
    #[arg(global = true, short, long, value_name = "FILE")] // make default the entry in config
    manifest: Option<PathBuf>,

    /// Filepath for output
    #[arg(global = true, short, long, value_name = "PATH")] // make default the entry in config
    output: Option<PathBuf>,

    /// Prints full details
    #[arg(global = true, long)]
    details: Option<bool>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Orchestration Configuration Attack
    Flare {
        /// K8s API Object options
        #[arg(short, long)]
        api: Option<String>,
    },
    /// Perceive Kubernetes API Object Hierarchy
    Libra {
        /// Depth of hierarchy
        #[arg(long)]
        depth: Option<u8>,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Flare { api } => {
            println!("'flare api:' {api:?}");
        }
        Commands::Libra { depth } => {
            println!("'libra depth:' {depth:?}");
        }
    }
}
