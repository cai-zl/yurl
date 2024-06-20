use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::{env, fs};

use clap::error::Result;
use colored::Colorize;
use md5::Digest;
use serde::{Deserialize, Serialize};

use crate::core::request::Request;

use self::error::YurlError;

pub mod error;
pub mod expression;
pub mod function;
pub mod request;
pub mod multipart;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(default)]
    pub imports: Vec<String>,
    #[serde(default)]
    pub vars: HashMap<String, String>,
    #[serde(default)]
    pub requests: Vec<Request>,
}

impl Template {
    pub fn from_to_file(file: &str) -> Result<Self, Box<dyn Error>> {
        let parsed_file = Rc::new(RefCell::new(Vec::new()));
        let templates = Self::parse(file, parsed_file)?;
        // merge
        let mut template: Self = Default::default();
        for t in templates {
            for (k, v) in t.vars {
                if template.vars.contains_key(&k) {
                    println!(
                        "{}",
                        format!("duplicated variable: [{}], new value: [{}]", &k, &v).yellow()
                    );
                }
                template.vars.insert(k, v);
            }
            for r in t.requests {
                if template.requests.contains(&r) {
                    return Err(Box::new(YurlError::new(&format!(
                        "duplicated request: {}",
                        &r.name
                    ))));
                }
                template.requests.push(r);
            }
        }
        Ok(template)
    }

    fn parse(
        file: &str,
        parsed_file: Rc<RefCell<Vec<Digest>>>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut templates = Vec::new();
        let file_path = Path::new(file);
        let current_dir = env::current_dir()?;
        let yaml = fs::read_to_string(file)?;
        let parent_dir = file_path.parent().unwrap();
        env::set_current_dir(parent_dir)?;
        let digest = md5::compute(&yaml);
        if parsed_file.borrow().contains(&digest) {
            env::set_current_dir(current_dir)?;
            return Ok(templates);
        }
        println!("{}", format!("parse file: {}", file).green());
        parsed_file.borrow_mut().push(digest);
        let template: Template = serde_yaml::from_str(&yaml)?;
        if template.imports.is_empty() {
            env::set_current_dir(current_dir)?;
            return Ok(templates);
        }
        for import in &template.imports {
            let childes = Self::parse(&import, parsed_file.clone())?;
            if !childes.is_empty() {
                for child in childes {
                    templates.push(child);
                }
            }
        }
        env::set_current_dir(current_dir)?;
        templates.push(template);
        Ok(templates)
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            imports: Default::default(),
            vars: Default::default(),
            requests: Default::default(),
        }
    }
}
