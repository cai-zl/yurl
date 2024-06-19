use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::{cell::RefCell, fs::File};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(default)]
    pub imports: Vec<String>,
    #[serde(default)]
    pub vars: HashMap<String, String>,
    pub requests: Vec<Request>,
}

impl Template {
    pub fn from_to_yaml(yaml: &str) -> Result<Self, Box<dyn Error>> {
        let template = serde_yaml::from_str(yaml)?;
        Ok(template)
    }

    pub fn from_to_file(file: &str) -> Result<Self, Box<dyn Error>> {
        let ds = RefCell::new(Vec::new());
        let templates = Self::parse_import(file, ds)?;
        // merge
        let mut template: Self = Default::default();
        for t in templates {
            for (k, v) in t.vars {
                if template.vars.contains_key(&k) {
                    println!(
                        "{}",
                        format!("duplicated variable: [{}], new value: [{}].", &k, &v).yellow()
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

    fn parse_import(file: &str, ds: RefCell<Vec<Digest>>) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut templates = Vec::new();
        let yaml = fs::read_to_string(file)?;
        let digest = md5::compute(&yaml);
        {
            if ds.borrow().contains(&digest) {
                return Ok(templates);
            }
        }
        println!("{}", format!("parse import file: {}", file).green());
        {
            ds.borrow_mut().push(digest);
        }
        let template = Template::from_to_yaml(&yaml)?;
        for import in &template.imports {
            let childes = Self::parse_import(&import, ds.clone())?;
            if !childes.is_empty() {
                for child in childes {
                    templates.push(child);
                }
            }
        }
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
