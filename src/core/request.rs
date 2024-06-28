use std::collections::HashMap;
use std::error::Error;
use std::{cmp::Ordering, path::Path};

use serde::{Deserialize, Serialize};

use crate::core::error::YurlError;

use super::multipart::MultipartBuilder;

const CONTENT_TYPE_KEY: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_FROM: &str = "application/x-www-form-urlencoded";
const CONTENT_TYPE_URL: &str = "application/x-www-form-urlencoded";
const CONTENT_TYPE_FILE: &str = "multipart/form-data";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub order: i32,
    pub name: String,
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub content_type: ContentType,
    pub response_type: ResponseType,
    #[serde(skip)]
    pub response: Option<String>,
}

impl Request {
    pub fn run(&self) -> Result<String, Box<dyn Error>> {
        match self.method {
            Method::GET => self.get(),
            Method::POST => self.post(),
            Method::PUT => self.put(),
            Method::DELETE => self.delete(),
        }
    }

    fn get(&self) -> Result<String, Box<dyn Error>> {
        let request = ureq::request("GET", &self.url);
        self.execute(request)
    }

    fn post(&self) -> Result<String, Box<dyn Error>> {
        let request = ureq::request("POST", &self.url);
        self.execute(request)
    }

    fn put(&self) -> Result<String, Box<dyn Error>> {
        let request = ureq::request("PUT", &self.url);
        self.execute(request)
    }

    fn delete(&self) -> Result<String, Box<dyn Error>> {
        let request = ureq::request("DELETE", &self.url);
        self.execute(request)
    }

    fn execute(&self, mut request: ureq::Request) -> Result<String, Box<dyn Error>> {
        let content_type = self.content_type.to_kv();
        request = request.set(content_type.0, content_type.1);
        for (k, v) in self.headers.iter() {
            request = request.set(k, v);
        }
        let response: ureq::Response;
        match self.content_type {
            ContentType::URLENCODED => {
                for (k, v) in self.params.iter() {
                    request = request.query(k, v);
                }
                response = request.call()?
            }
            ContentType::FORM => {
                let body: Vec<(&str, &str)> = self
                    .params
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                response = request.send_form(&body[..])?
            }
            ContentType::JSON => response = request.send_json(&self.params)?,
            ContentType::FILE => {
                if self.method != Method::POST {
                    return Err(Box::new(YurlError::new("file request only support POST")));
                }
                let mut multipart = MultipartBuilder::new();
                for (k, v) in &self.params {
                    if v.starts_with("FILE(") && v.ends_with(")") {
                        multipart = multipart.add_file(k, Path::new(&v[5..v.len() - 1]))?;
                    } else {
                        multipart = multipart.add_text(k, v)?;
                    }
                }
                let (content_type, data) = multipart.finish()?;
                response = request
                    .set(CONTENT_TYPE_KEY, &content_type)
                    .send_bytes(&data)?
            }
        }
        if response.status() == 200 {
            Ok(response.into_string()?)
        } else {
            return Err(Box::new(YurlError::new(&format!(
                "request name: [{}], url: [{}] execute fail, status code: {}, message: {}",
                self.name,
                self.url,
                response.status(),
                response.status_text()
            ))));
        }
    }
}

impl Eq for Request {}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl PartialOrd<Self> for Request {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Request {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl Default for Request {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert("Authorization".to_string(), "xxxxxxxxxxxxxxx".to_string());
        let mut p = HashMap::new();
        p.insert("name".to_string(), "${var.name}".to_string());
        Self {
            order: 1,
            name: "example".to_string(),
            url: "http://127.0.0.1:8080/example".to_string(),
            method: Method::GET,
            headers: h,
            params: p,
            content_type: ContentType::URLENCODED,
            response_type: ResponseType::JSON,
            response: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    URLENCODED,
    FORM,
    JSON,
    FILE,
}

impl ContentType {
    pub fn to_kv(&self) -> (&'static str, &'static str) {
        match &self {
            ContentType::URLENCODED => (CONTENT_TYPE_KEY, CONTENT_TYPE_URL),
            ContentType::JSON => (CONTENT_TYPE_KEY, CONTENT_TYPE_JSON),
            ContentType::FORM => (CONTENT_TYPE_KEY, CONTENT_TYPE_FROM),
            ContentType::FILE => (CONTENT_TYPE_KEY, CONTENT_TYPE_FILE),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    TEXT,
    JSON,
    HTML,
    FILE,
}

#[cfg(test)]
mod tests {
    use serde_yaml::Value;

    use super::Request;

    #[test]
    fn test_run() {
        let request_yaml = r#"order: 1
name: post-form
url: http://127.0.0.1:8000/post/form
method: POST
headers:
params:
  name: post-form
content_type: FORM
response_type: JSON
"#;
        let request:Request = serde_yaml::from_str(request_yaml).unwrap();
        let resp = request.run().unwrap();
        assert_eq!(resp,"{\"code\":200,\"message\":\"success\",\"data\":{\"name\":\"post-form\"}}");
    }

    #[test]
    fn test_yaml_parse() {
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
        let value: Value = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(true, value.is_mapping());
        assert_eq!(
            true,
            value
                .as_mapping()
                .unwrap()
                .get("arr")
                .unwrap()
                .is_sequence()
        );
        assert_eq!(
            true,
            value.as_mapping().unwrap().get("obj").unwrap().is_mapping()
        );
        assert_eq!(
            true,
            value
                .as_mapping()
                .unwrap()
                .get("obj")
                .unwrap()
                .get("list")
                .unwrap()
                .is_sequence()
        );
    }
}
