use std::error::Error;

use crate::{core::error::YurlError, yurl_error};

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
                _ => Err(yurl_error!(&format!(
                    "not supported expression type: {}",
                    fields[0]
                ))),
            };
        }
        Err(yurl_error!(&format!(
            "expression {} formatting error",
            expression,
        )))
    }

    pub fn variable_parse(expression: &str) -> Result<String, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() < 2 {
            return Err(yurl_error!("variable expression formatting error"));
        }
        Ok(keys[1..].join("."))
    }

    pub fn function_parse(expression: &str) -> Result<String, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() != 2 {
            return Err(yurl_error!("function expression formatting error"));
        }
        Ok(keys.get(1).unwrap().to_string())
    }

    pub fn response_parse(expression: &str) -> Result<ResponseExpression, Box<dyn Error>> {
        let keys: Vec<&str> = expression.split(".").collect();
        if keys.len() < 3 {
            return Err(yurl_error!("response expression formatting error"));
        }
        Ok(ResponseExpression {
            parent: keys.get(1).unwrap().to_string(),
            path: keys[2..].join(".").to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;

    #[test]
    fn test_parse_from_str() {
        let expr = Expression::parse_from_str("${var.prefix}/example").unwrap();
        assert_eq!(expr[0], "${var.prefix}");
        let expr = Expression::parse_from_str("${fun.prefix}/example").unwrap();
        assert_eq!(expr[0], "${fun.prefix}");
        let expr = Expression::parse_from_str("${res.prefix}/example").unwrap();
        assert_eq!(expr[0], "${res.prefix}");
    }

    #[test]
    fn test_parse() {
        let expr = Expression::parse("${var.prefix}").unwrap();
        match expr {
            Expression::Variable(v) => assert_eq!(v, "var.prefix"),
            _ => {}
        }
    }

    #[test]
    fn test_variable_parse() {
        let expr = Expression::parse("${var.prefix}").unwrap();
        match expr {
            Expression::Variable(v) => {
                assert_eq!("prefix", Expression::variable_parse(&v).unwrap())
            }
            _ => {}
        }
    }

    #[test]
    fn test_function_parse() {
        let expr = Expression::parse("${fun.uuid}").unwrap();
        match expr {
            Expression::Function(v) => {
                assert_eq!("uuid", Expression::function_parse(&v).unwrap())
            }
            _ => {}
        }
    }

    #[test]
    fn test_response_parse() {
        let expr = Expression::parse("${res.example.data.code}").unwrap();
        match expr {
            Expression::Response(v) => {
                let re = Expression::response_parse(&v).unwrap();
                assert_eq!("example", re.parent);
                assert_eq!("data.code", re.path);
            }
            _ => {}
        }
    }
}
