mod metaparser;
mod symtab;

use clap::{Parser, Subcommand};

use crate::metaparser::interpret;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Interpret {
        filepath: String,
    },
    Eval {
        source: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(x) => match x {
            Commands::Interpret { filepath } => {
                println!("Interpreting: {filepath}");
                let src = std::fs::read_to_string(filepath).unwrap_or_default();
                interpret(&src);
            }
            Commands::Eval { source } => {
                interpret(&source);
            }
        },
        None => {}
    }
}
