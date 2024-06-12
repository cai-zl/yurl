pub mod run;
pub mod function;
pub mod generate;

use std::error::Error;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "yurl", subcommand_required = true)]
#[command(about = "A fictional versioning CLI", long_about = None)]
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
    #[command(arg_required_else_help = true, short_flag = 'r')]
    Run(run::RunArg),
    #[command(name = "function", subcommand_required = true, long_flag = "fun", short_flag = 'f')]
    Function(function::FunctionArg),
    #[command(name = "generate", long_flag = "gen", short_flag = 'g')]
    Generate(generate::GenerateArg),
}

pub trait Execute {
    fn run(self) -> Result<(), Box<dyn Error>>;
}