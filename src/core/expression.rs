use std::error::Error;

use crate::core::error::YurlError;

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    Function(String),
    Response(String),
}

#[derive(Debug)]
pub struct ResponseExpression {
    pub parent: String,
    pub path: String,
}

impl Expression {
    pub fn parse_from_str(str: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let chars: Vec<char> = str.chars().collect();
        let mut expressions = Vec::new();
        let mut expression = String::new();
        let mut head = false;
        for c in chars {
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
        Ok(expressions)
    }

    pub fn parse(expression: &str) -> Result<Self, Box<dyn Error>> {
        if expression.starts_with("${") && expression.ends_with("}") {
            let expr: &str = &expression[2..expression.len() - 1];
            let fields: Vec<&str> = expr.split(".").map(|m| m).collect();
            return match fields[0] {
                "var" => Ok(Expression::Variable(expr.to_string())),
                "fun" => Ok(Expression::Function(expr.to_string())),
                "res" => Ok(Expression::Response(expr.to_string())),
                _ => Err(Box::new(YurlError::new(&format!(
                    "not supported expression type: {}",
                    fields[0]
                )))),
            };
        }
        Err(Box::new(YurlError::new(&format!(
            "expression {} formatting error",
            expression,
        ))))
    }

    pub fn variable_parse(expression: &str) -> Result<String, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() != 2 {
            return Err(Box::new(YurlError::new(
                "variable expression formatting error",
            )));
        }
        Ok(keys.get(1).unwrap().to_string())
    }

    pub fn function_parse(expression: &str) -> Result<String, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() != 2 {
            return Err(Box::new(YurlError::new(
                "function expression formatting error",
            )));
        }
        Ok(keys.get(1).unwrap().to_string())
    }

    pub fn response_parse(expression: &str) -> Result<ResponseExpression, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() < 3 {
            return Err(Box::new(YurlError::new(
                "response expression formatting error",
            )));
        }
        Ok(ResponseExpression {
            parent: keys.get(1).unwrap().to_string(),
            path: keys.get(2).unwrap().to_string(),
        })
    }
}
