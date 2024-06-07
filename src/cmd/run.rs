use std::error::Error;
use std::fs;

use clap::Args;
use colored::Colorize;

use crate::core::Template;

use super::Execute;

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct RunArg {
    #[arg(short, long)]
    pub file: String,
}

impl Execute for RunArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(self.file)?;
        let t = Template::from_to_yaml(&content)?;
        for request in t.requests.iter() {
            let res = request.run()?;
            println!("{}",res.green());
        }
        Ok(())
    }
}