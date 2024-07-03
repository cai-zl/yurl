use std::cell::RefCell;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use std::{env, fs};

use clap::error::Result;
use colored::Colorize;
use md5::Digest;
use serde::{Deserialize, Serialize};

use crate::core::request::Request;
use crate::{success, yurl_error};

use self::error::YurlError;

pub mod error;
pub mod expression;
pub mod function;
pub mod json;
pub mod log;
pub mod multipart;
pub mod request;
pub mod yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(default)]
    pub imports: Vec<String>,
    #[serde(default)]
    pub vars: serde_yaml::Value,
    #[serde(default)]
    pub requests: Vec<Request>,
    #[serde(skip)]
    pub variables: Vec<serde_yaml::Value>,
}

impl Template {
    pub fn from_to_file(file: &str) -> Result<Self, Box<dyn Error>> {
        let parsed_file = Rc::new(RefCell::new(Vec::new()));
        let templates = Self::parse(file, parsed_file)?;
        // merge
        let mut template = Template::default();
        for t in templates {
            if !t.vars.is_null() {
                template.variables.push(t.vars);
            }
            for r in t.requests {
                if template.requests.contains(&r) {
                    return Err(yurl_error!(&format!("duplicated request: {}", &r.name)));
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
        success!(format!("parse file: {}", file));
        parsed_file.borrow_mut().push(digest);
        let template: Template = serde_yaml::from_str(&yaml)?;
        if template.imports.is_empty() {
            env::set_current_dir(current_dir)?;
            templates.push(template);
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
            imports: Vec::default(),
            vars: serde_yaml::Value::default(),
            requests: Vec::default(),
            variables: Vec::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_yaml_to_json() {
        let yaml = r#"vars:
  name: caizl
  age: 18
email: 740662047@qq.com
arr:
  - hello
  - test
obj:
  gate: zuul
  put: map
  list:
    - consul
    - nacos"#;
        let yaml_v: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let json_v = serde_json::json!(yaml_v);
        let json = serde_json::to_string(&json_v);
        println!("{:#?}", json);
    }

    #[test]
    fn test_yaml_merge() {
        let yaml = r#"vars:
  name: caizl
  age: 18
email: 740662047@qq.com
arr:
  - hello
  - test
obj:
  gate: zuul
  put: map
  list:
    - consul
    - nacos"#;
        let yaml_v: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let obj = yaml_v.as_mapping().unwrap();
        for (k, v) in obj {
            match v {
                serde_yaml::Value::Null => break,
                serde_yaml::Value::Bool(_) => todo!(),
                serde_yaml::Value::Number(_) => todo!(),
                serde_yaml::Value::String(_) => todo!(),
                serde_yaml::Value::Sequence(_) => todo!(),
                serde_yaml::Value::Mapping(_) => todo!(),
                serde_yaml::Value::Tagged(_) => todo!(),
            }
        }
    }
}
