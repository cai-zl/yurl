use std::error::Error;

use clap::{Parser, Subcommand};

pub mod function;
pub mod generate;
pub mod run;

#[derive(Debug, Parser)]
#[command(name = "yurl", subcommand_required = true)]
#[command(
    about = "http cli tool.",
    long_about = "http requests through yaml files.",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(short_flag = 'r')]
    Run(run::RunArg),
    #[command(
        name = "function",
        subcommand_required = true,
        long_flag = "fun",
        short_flag = 'f'
    )]
    Function(function::FunctionArg),
    #[command(name = "generate", long_flag = "gen", short_flag = 'g')]
    Generate(generate::GenerateArg),
}

pub trait Execute {
    fn run(self) -> Result<(), Box<dyn Error>>;
}
