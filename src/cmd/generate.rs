use std::error::Error;
use clap::Args;
use crate::cmd::Execute;

#[derive(Debug, Args)]
#[command(flatten_help = true)]
pub struct GenerateArg {
    #[arg(long, short)]
    pub from: Option<String>,
    #[arg(long, short)]
    pub out: Option<String>,
}

impl Execute for GenerateArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self.from {
            Some(from) => {}
            None => {}
        }
        match self.out {
            None => {}
            Some(out) => {}
        }
        Ok(())
    }
}