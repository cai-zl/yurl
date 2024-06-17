use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::core::error::YurlError;

const CONTENT_TYPE_KEY: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_FROM: &str = "application/x-www-form-urlencoded";
const CONTENT_TYPE_URL: &str = "application/x-www-form-urlencoded";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
                return Err(Box::new(YurlError::new("not support file")));
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
            ContentType::FILE => (CONTENT_TYPE_KEY, CONTENT_TYPE_FROM),
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
