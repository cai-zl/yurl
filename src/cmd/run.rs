use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

use clap::Args;
use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::core::error::YurlError;
use crate::core::expression::Expression;
use crate::core::function::Function;
use crate::core::json::Json;
use crate::core::request::Request;
use crate::core::yaml::Yaml;
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
    variables: &'a Vec<serde_yaml::Value>,
    functions: HashMap<String, Function>,
    responses: HashMap<&'a str, &'a Request>,
}

impl Execute for RunArg {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut template = Template::from_to_file(&self.file)?;
        template.requests.sort();
        {
            let mut ev = ExpressionValue {
                variables: &template.variables,
                functions: Function::functions(),
                responses: Default::default(),
            };
            for request in template.requests.iter_mut() {
                // parse url
                _ = parse_str(&ev, &mut request.url)?;
                // parse params
                _ = parse_param(&ev, &mut request.params)?;
                // parse headers
                _ = parse_header(&ev, &mut request.headers)?;
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
                params: format!("{:?}", serde_json::to_string(&m.params).unwrap()),
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

fn parse_str(ev: &ExpressionValue, url: &mut String) -> Result<(), Box<dyn Error>> {
    let expressions = Expression::parse_from_str(&url)?;
    for expression in expressions {
        let mut new_content: String;
        match Expression::parse(&expression)? {
            Expression::Variable(expr) => {
                let key = Expression::variable_parse(&expr)?;
                for variable in ev.variables {
                    let v = Yaml::new(&variable, key.clone()).get_value()?;
                    match v {
                        serde_yaml::Value::Null => {
                            return Err(yurl_error!(&format!("undefined variable: {}", key)))
                        }
                        serde_yaml::Value::Bool(v) => {
                            new_content = url.replace(&expression, &format!("{}", v));
                            url.clear();
                            url.push_str(&new_content);
                        }
                        serde_yaml::Value::Number(v) => {
                            new_content = url.replace(&expression, &format!("{}", v));
                            url.clear();
                            url.push_str(&new_content);
                        }
                        serde_yaml::Value::String(v) => {
                            new_content = url.replace(&expression, v);
                            url.clear();
                            url.push_str(&new_content);
                        }
                        serde_yaml::Value::Sequence(_) => {
                            return Err(yurl_error!(&format!("undefined variable: {}", key)))
                        }
                        serde_yaml::Value::Mapping(_) => {
                            return Err(yurl_error!(&format!("undefined variable: {}", key)))
                        }
                        serde_yaml::Value::Tagged(_) => {
                            return Err(yurl_error!(&format!("undefined variable: {}", key)))
                        }
                    }
                }
            }
            Expression::Function(expr) => {
                let key = Expression::function_parse(&expr)?;
                match ev.functions.get(&key) {
                    Some(f) => {
                        new_content = url.replace(&expression, &(f.fun)());
                        url.clear();
                        url.push_str(&new_content);
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
                        let v = Json::new(&res, re.path).get_value()?;
                        new_content = url.replace(&expression, &v.to_string());
                        url.clear();
                        url.push_str(&new_content);
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

fn parse_param(ev: &ExpressionValue, param: &mut serde_yaml::Value) -> Result<(), Box<dyn Error>> {
    match param {
        serde_yaml::Value::Null => {}
        serde_yaml::Value::Bool(_) => {}
        serde_yaml::Value::Number(_) => {}
        serde_yaml::Value::String(v) => {
            let expr = Expression::parse_from_str(&v)?;
            if expr.len() > 0 {
                *param = parse(ev, v)?;
            }
        }
        serde_yaml::Value::Sequence(v) => {
            for ele in v.iter_mut() {
                parse_param(ev, ele)?;
            }
        }
        serde_yaml::Value::Mapping(v) => {
            for ele in v.iter_mut() {
                parse_param(ev, ele.1)?;
            }
        }
        serde_yaml::Value::Tagged(_) => {
            return Err(yurl_error!("parameter parsing error."));
        }
    }
    Ok(())
}

fn parse_header(
    ev: &ExpressionValue,
    header: &mut HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    for (_, mut v) in header.iter_mut() {
        _ = parse_str(&ev, &mut v)?;
    }
    Ok(())
}

fn parse(ev: &ExpressionValue, content: &String) -> Result<serde_yaml::Value, Box<dyn Error>> {
    let expressions = Expression::parse_from_str(&content)?;
    for expression in expressions {
        match Expression::parse(&expression)? {
            Expression::Variable(expr) => {
                let key = Expression::variable_parse(&expr)?;
                for variable in ev.variables {
                    let v = Yaml::new(&variable, key).get_value()?;
                    return Ok(v.clone());
                }
            }
            Expression::Function(expr) => {
                let key = Expression::function_parse(&expr)?;
                match ev.functions.get(&key) {
                    Some(f) => {
                        return Ok(serde_yaml::Value::String((f.fun)()));
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
                        let v = Json::new(&res, re.path).get_value()?;
                        return Ok(serde_yaml::to_value(v)?);
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
    Ok(serde_yaml::Value::Null)
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::core::{function::Function, json::Json, yaml::Yaml};

    use super::{parse_param, parse_str, ExpressionValue};

    #[test]
    fn test_json() {
        let v = json!({"a": "s","b": ["2","1"]});
        let v = Json::new(&v, "b.#1".to_string()).get_value().unwrap();
        assert_eq!(1, v.as_i64().unwrap());
    }

    #[test]
    fn test_json2() {
        let v = json!({"a": "s","b": ["2","1"]});
        let v = v.get("b").unwrap().get(1);
        assert_eq!(1, v.unwrap().as_i64().unwrap());
    }

    #[test]
    fn test_json_to_yaml() {
        let v = json!({"a": "s","b": ["2","1"]});
        let yv = serde_yaml::to_value(&v).unwrap();
        assert_eq!(true, yv.is_mapping());
    }

    #[test]
    fn test_parse_param() {
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
        let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        let yaml2 = r#"vars:
  name: caizl
  age: 18
email: 740662047@qq.com
arr:
  - hello
  - test
obj:
  gate: ${var.obj.gate}
  put: map
  list:
    - consul
    - nacos"#;
        let v = vec![value];
        let mut value2: serde_yaml::Value = serde_yaml::from_str(&yaml2).unwrap();
        let ev = ExpressionValue {
            variables: &v,
            functions: Function::functions(),
            responses: Default::default(),
        };
        let _ = parse_param(&ev, &mut value2).unwrap();
        let v = Yaml::new(&value2, "obj.gate".to_string())
            .get_value()
            .unwrap();
        assert_eq!("zuul", v.as_str().unwrap());
    }

    #[test]
    fn test_parse_url() {
        let mut url = "http://${var.host}:${var.port}".to_string();
        let yaml = r#"host: localhost
port: 8080"#;
        let value: serde_yaml::Value = serde_yaml::from_str(&yaml).unwrap();
        let v = vec![value];
        let ev = ExpressionValue {
            variables: &v,
            functions: Function::functions(),
            responses: Default::default(),
        };
        let _ = parse_str(&ev, &mut url).unwrap();
        assert_eq!("http://localhost:8080", url);
    }
}
