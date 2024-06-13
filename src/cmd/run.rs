use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use clap::Args;
use colored::Colorize;
use tabled::{Table, Tabled};
use tabled::builder::Builder;
use tabled::settings::Style;

use crate::core::Template;

use super::Execute;

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct RunArg {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long, default_value = "false")]
    pub pretty: bool,
}

impl Execute for RunArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(self.file)?;
        let mut t = Template::from_to_yaml(&content)?;
        t.requests.sort();
        let mut responses: HashMap<String, RequestItem> = HashMap::new();
        for request in t.requests.iter() {
            let res = request.run()?;
            let item = RequestItem {
                order: request.order,
                name: format!("{}", request.name),
                method: format!("{:?}", request.method),
                url: format!("{}", request.url),
                params: format!("{:?}", request.params),
                headers: format!("{:?}", request.headers),
                response: format!("{}", res),
            };
            responses.insert(item.name.to_string(), item);
        }
        let mut items: Vec<&RequestItem> = responses.values().collect();
        items.sort();
        if self.pretty {
            let table = Builder::from(Table::new(items)).build().with(Style::rounded()).to_string();
            println!("{}", table.green());
        } else {
            for item in items {
                println!("{}", format!("[{}] -- [{}] -- {}", item.name, item.url, item.response).green());
            }
        }
        Ok(())
    }
}

#[derive(Tabled, PartialEq, Eq)]
struct RequestItem {
    order: i32,
    name: String,
    method: String,
    url: String,
    params: String,
    headers: String,
    response: String,
}

impl PartialOrd<Self> for RequestItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for RequestItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}