use std::error::Error;
use std::fs;

use clap::Args;
use colored::Colorize;
use tabled::builder::Builder;
use tabled::{Table, Tabled};
use tabled::settings::Style;

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
        let mut t = Template::from_to_yaml(&content)?;
        t.requests.sort();
        let mut items = Vec::new();
        for request in t.requests.iter() {
            let res = request.run()?;
            items.push(RequestItem{
                order: request.order,
                name: format!("{}",request.name),
                method: format!("{:?}", request.method),
                url: format!("{}",request.url),
                params: format!("{:?}", request.params),
                headers: format!("{:?}", request.headers),
                response: format!("{}", res),
            });
        }
        let table = Builder::from(Table::new(items)).build().with(Style::rounded()).to_string();
        println!("{}", table.green());
        Ok(())
    }
}

#[derive(Tabled)]
struct RequestItem {
    order: i32,
    name: String,
    method: String,
    url: String,
    params: String,
    headers: String,
    response: String,
}