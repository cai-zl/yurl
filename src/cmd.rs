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
    #[command(arg_required_else_help = true)]
    Run(run::RunArg),
    #[command(name = "fun", subcommand_required = true)]
    Function(function::FunctionArg),
}

pub trait Execute {
    fn run(self) -> Result<(), Box<dyn Error>>;
}