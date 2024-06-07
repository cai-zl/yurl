use std::error::Error;
use std::fmt::{Debug, Display};

use crate::core::error::YurlError;

pub enum Expression {
    Variable(String),
    Function(String),
    Response(String),
}

pub struct Response {
    pub parent: String,
    pub path: String,
}

impl Expression {
    pub fn parse_from_yaml(yaml: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let mut chars: Vec<char> = yaml.chars().collect();
        let mut expressions = Vec::new();
        let mut expression = String::new();
        let mut head = false;
        loop {
            match chars.pop() {
                Some(c) => {
                    if c == '$' {
                        expression.push(c);
                    }
                    if expression.len() == 1 && c == '{' {
                        head = true;
                    }
                    if c == '}' && head {
                        expression.push(c);
                        head = false;
                        expressions.push(expression.to_string());
                        expression.clear();
                    }
                    if head {
                        expression.push(c)
                    }
                }
                None => { break; }
            }
        }
        Ok(expressions)
    }

    pub fn parse(expression: &str) -> Result<Self, Box<dyn Error>> {
        if expression.starts_with("${")
            && expression.ends_with("}") {
            return match &expression[2..5] {
                "var" => { Ok(Expression::Variable(expression[2..expression.len()].to_string())) }
                "fun" => { Ok(Expression::Function(expression[2..expression.len()].to_string())) }
                "res" => { Ok(Expression::Response(expression[2..expression.len()].to_string())) }
                _ => { Err(Box::new(YurlError::new("not supported expression type"))) }
            };
        }
        Err(Box::new(YurlError::new("expression formatting error")))
    }

    pub fn variable_parse(expression: &str) -> Result<String, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() != 2 {
            return Err(Box::new(YurlError::new("variable expression formatting error")));
        }
        Ok(keys.get(1).unwrap().to_string())
    }

    pub fn function_parse(expression: &str) -> Result<String, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() != 2 {
            return Err(Box::new(YurlError::new("function expression formatting error")));
        }
        Ok(keys.get(1).unwrap().to_string())
    }

    pub fn response_parse(expression: &str) -> Result<Response, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() > 3 {
            return Err(Box::new(YurlError::new("response expression formatting error")));
        }
        Ok(Response {
            parent: keys.get(1).unwrap().to_string(),
            path: keys.get(2).unwrap().to_string(),
        })
    }
}

