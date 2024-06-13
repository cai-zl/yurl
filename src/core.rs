use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::core::error::YurlError;
use crate::core::expression::Expression;
use crate::core::request::Request;

pub mod function;
pub mod error;
mod request;
mod expression;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub vars: HashMap<String, String>,
    pub requests: Vec<Request>,
}

impl Template {
    pub fn from_to_yaml(yaml: &str) -> Result<Self, Box<dyn Error>> {
        let expressions = Expression::parse_from_yaml(yaml)?;
        let template: Template = serde_yaml::from_str(yaml)?;
        let functions = function::Function::functions();
        let mut content = String::from(yaml);
        for expression in expressions {
            match Expression::parse(&expression)? {
                Expression::Variable(expr) => {
                    let key = Expression::variable_parse(&expr)?;
                    match template.vars.get(&key) {
                        Some(v) => {
                            let temp_content = content.replace(&expression, v);
                            content.clear();
                            content.push_str(&temp_content);
                        }
                        None => { return Err(Box::new(YurlError::new(&format!("undefined variable: {}", key)))); }
                    }
                }
                Expression::Function(expr) => {
                    let key = Expression::function_parse(&expr)?;
                    match functions.get(&key) {
                        Some(f) => {
                            let temp_content = content.replace(&expression, &(f.fun)());
                            content.clear();
                            content.push_str(&temp_content);
                        }
                        None => { return Err(Box::new(YurlError::new(&format!("undefined function: {}", key)))); }
                    }
                }
                Expression::Response(expr) => {}
            }
        }
        let template: Template = serde_yaml::from_str(&content)?;
        Ok(template)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::core::Template;

    #[test]
    fn from_to_file() {
        let content = fs::read_to_string("D:\\Projects\\rs\\yurl\\yaml\\template.yaml").unwrap();
        let t = Template::from_to_yaml(&content).unwrap();
        let r = t.requests.get(0).unwrap();
        if let res = r.run().unwrap() {
            println!("{}", res)
        }
    }
}