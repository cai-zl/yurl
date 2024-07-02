use std::error::Error;

use clap::{Args, Subcommand};
use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::core::error::YurlError;
use crate::core::function::Function;
use crate::{success, yurl_error};

use super::Execute;

#[derive(Tabled)]
struct FunctionItem<'a> {
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
            Some(commands) => match commands {
                FunctionCommands::List(arg) => arg.run(),
                FunctionCommands::Call(arg) => arg.run(),
                FunctionCommands::Search(arg) => arg.run(),
            },
            None => Ok(()),
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
#[command(about, long_about = None)]
pub struct ListArg {}

impl Execute for ListArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let fs = Function::functions();
        let mut items: Vec<FunctionItem> = fs
            .values()
            .map(|i| FunctionItem {
                key: &i.key,
                about: &i.about,
                result: (i.fun)(),
            })
            .collect();
        items.sort_by(|o1, o2| o1.key.cmp(o2.key));
        let table = Builder::from(Table::new(items))
            .build()
            .with(Style::rounded())
            .to_string();
        success!(table);
        Ok(())
    }
}

#[derive(Args, Debug)]
#[command(about, long_about = None)]
pub struct CallArg {
    #[arg(long, short, required = true)]
    pub key: Option<String>,
}

impl Execute for CallArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let key: String = self.key.unwrap();
        match Function::functions().get(&key) {
            None => Err(yurl_error!(&format!("undefined function: {}", key))),
            Some(f) => Ok(success!((f.fun)())),
        }
    }
}

#[derive(Args, Debug)]
#[command(about, long_about = None)]
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
                items.push(FunctionItem {
                    key: &v.key,
                    about: &v.about,
                    result: (v.fun)(),
                });
            }
        }
        items.sort_by(|o1, o2| o1.key.cmp(o2.key));
        let table = Builder::from(Table::new(items))
            .build()
            .with(Style::rounded())
            .to_string();
        Ok(success!(table))
    }
}
