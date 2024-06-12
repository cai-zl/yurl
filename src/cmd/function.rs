use std::error::Error;

use clap::{Args, Subcommand};
use colored::Colorize;
use tabled::{Table, Tabled};

use crate::core::error::YurlError;
use crate::core::function::Function;

use super::Execute;

#[derive(Tabled)]
struct Item<'a> {
    key: &'a str,
    about: &'a str,
    result: String,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct FunctionArg {
    #[command(subcommand)]
    pub command: Option<FunctionCommands>,
}

impl Execute for FunctionArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self.command {
            Some(commands) => {
                match commands {
                    FunctionCommands::List(arg) => {
                        arg.run()
                    }
                    FunctionCommands::Call(arg) => {
                        arg.run()
                    }
                    FunctionCommands::Search(arg) => {
                        arg.run()
                    }
                }
            }
            None => { Ok(()) }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum FunctionCommands {
    #[command(short_flag = 'l')]
    List(ListArg),
    #[command(short_flag = 'c')]
    Call(CallArg),
    #[command(short_flag = 's')]
    Search(SearchArg),
}


#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct ListArg {}

impl Execute for ListArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let fs = Function::functions();
        let items: Vec<Item> = fs.values().map(|i| {
            Item {
                key: &i.key,
                about: &i.about,
                result: (i.fun)(),
            }
        }).collect();
        let table = Table::new(items).to_string();
        println!("{}", table.green());
        Ok(())
    }
}

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct CallArg {
    #[arg(long, short, required = true)]
    pub key: Option<String>,
}

impl Execute for CallArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let key: String = self.key.unwrap();
        match Function::functions().get(&key) {
            None => { Err(Box::new(YurlError::new(&format!("undefined function: {}", key)))) }
            Some(f) => {
                Ok(println!("{}", (f.fun)().green()))
            }
        }
    }
}

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct SearchArg {
    #[arg(long, short, required = true)]
    pub key: Option<String>,
}

impl Execute for SearchArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let fs = Function::functions();
        let key: String = self.key.unwrap();
        let mut items = Vec::new();
        for (k, v) in fs.iter() {
            if k.contains(&key) {
                items.push(Item { key: &v.key, about: &v.about, result: (v.fun)() });
            }
        }
        let table = Table::new(items).to_string();
        Ok(println!("{}", table.green()))
    }
}