use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

use clap::Args;
use colored::Colorize;
use serde_json::Value;
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::core::error::YurlError;
use crate::core::expression::Expression;
use crate::core::function::Function;
use crate::core::request::Request;
use crate::core::Template;
use crate::{success, yurl_error};

use super::Execute;

#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct RunArg {
    #[arg(short, long, default_value = "template.yaml")]
    pub file: String,
    #[arg(short, long, default_value = "false")]
    pub pretty: bool,
}

struct ExpressionValue<'a> {
    variables: &'a HashMap<String, String>,
    functions: HashMap<String, Function>,
    responses: HashMap<&'a str, &'a Request>,
}

impl Execute for RunArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut template = Template::from_to_file(&self.file)?;
        template.requests.sort();
        {
            let mut ev = ExpressionValue {
                variables: &template.vars,
                functions: Function::functions(),
                responses: Default::default(),
            };
            for request in template.requests.iter_mut() {
                // parse url
                _ = parse(&ev, &mut request.url)?;
                // parse params
                for (_, v) in &mut request.params {
                    _ = parse(&ev, v)?;
                }
                // parse headers
                for (_, v) in &mut request.headers {
                    _ = parse(&ev, v)?;
                }
                let res = request.run()?;
                request.response = Some(res);
                ev.responses.insert(&request.name, request);
            }
        }
        let items: Vec<RequestItem> = template
            .requests
            .iter()
            .map(|m| RequestItem {
                order: m.order,
                name: format!("{}", m.name),
                method: format!("{:?}", m.method),
                url: format!("{}", m.url),
                params: format!("{:?}", m.params),
                headers: format!("{:?}", m.headers),
                response: m.response.clone().unwrap_or(Default::default()),
            })
            .collect();
        if self.pretty {
            let table = Builder::from(Table::new(items))
                .build()
                .with(Style::rounded())
                .to_string();
            success!(table);
        } else {
            for item in items {
                success!(format!(
                    "[{}] -- [{}] -- {}",
                    item.name, item.url, item.response
                ));
            }
        }
        Ok(())
    }
}

fn parse(ev: &ExpressionValue, content: &mut String) -> Result<(), Box<dyn Error>> {
    let expressions = Expression::parse_from_str(&content)?;
    for expression in expressions {
        let new_content: String;
        match Expression::parse(&expression)? {
            Expression::Variable(expr) => {
                let key = Expression::variable_parse(&expr)?;
                match ev.variables.get(&key) {
                    Some(v) => {
                        new_content = content.replace(&expression, v);
                        content.clear();
                        content.push_str(&new_content);
                    }
                    None => {
                        return Err(yurl_error!(&format!("undefined variable: {}", key)));
                    }
                }
            }
            Expression::Function(expr) => {
                let key = Expression::function_parse(&expr)?;
                match ev.functions.get(&key) {
                    Some(f) => {
                        new_content = content.replace(&expression, &(f.fun)());
                        content.clear();
                        content.push_str(&new_content);
                    }
                    None => {
                        return Err(yurl_error!(&format!("undefined function: {}", key)));
                    }
                };
            }
            Expression::Response(expr) => {
                let re = Expression::response_parse(&expr)?;
                match ev.responses.get(&re.parent.as_str()) {
                    Some(r) => {
                        let res = serde_json::from_str(
                            &r.response.clone().unwrap_or(Default::default()),
                        )?;
                        let v = ResponseJson::new(&res, re.path).get_value()?;
                        new_content = content.replace(&expression, &v.to_string());
                        content.clear();
                        content.push_str(&new_content);
                    }
                    None => {
                        return Err(yurl_error!(&format!(
                            "request [{}] does not exist or is not executed.",
                            &re.parent
                        )));
                    }
                };
            }
        };
    }
    Ok(())
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

struct ResponseJson<'a> {
    value: Option<&'a Value>,
    path: Vec<String>,
}

impl<'a> ResponseJson<'a> {
    fn new(value: &'a Value, path: String) -> Self {
        let mut p: Vec<String> = path.split(".").map(|m| m.to_string()).collect();
        p.reverse();
        Self {
            value: Some(value),
            path: p,
        }
    }

    fn get_value(mut self) -> Result<&'a Value, Box<dyn Error>> {
        loop {
            let k = self.path.pop();
            match k {
                Some(k) => {
                    let s: Option<&Value>;
                    if k.starts_with("#") {
                        let k = &k[1..].parse::<usize>()?;
                        s = self.value.unwrap().get(k);
                    } else {
                        s = self.value.unwrap().get(k);
                    }
                    match s {
                        None => self.value = None,
                        Some(v) => self.value = Some(v),
                    };
                }
                None => {
                    break;
                }
            }
        }
        return match self.value {
            None => Err(yurl_error!("response expression parse error.")),
            Some(v) => Ok(v),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, error::Error, str::FromStr};

    use serde_json::json;

    use crate::{cmd::run::ResponseJson, core::function::Function};

    use super::{parse, ExpressionValue};

    #[test]
    fn test_json() -> Result<(), Box<dyn Error>> {
        let v = json!({"a": "s","b": ["2","1"]});
        let v = ResponseJson::new(&v, "b.#1".to_string()).get_value()?;
        dbg!(v);
        Ok(())
    }

    #[test]
    fn test_json2() -> Result<(), Box<dyn Error>> {
        let v = json!({"a": "s","b": ["2","1"]});
        let v = v.get("b").unwrap().get(1);
        dbg!(v);
        Ok(())
    }

    #[test]
    fn test_for() {
        let vars: HashMap<String, String> = Default::default();
        let ev = ExpressionValue {
            variables: &vars,
            functions: Function::functions(),
            responses: Default::default(),
        };
        let mut content = String::from_str("${res.date.da}/dasd").unwrap();
        let res = parse(&ev, &mut content);
        match res {
            Ok(_) => {
                println!("ok")
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }
}
