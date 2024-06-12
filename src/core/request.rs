use std::collections::HashMap;
use std::error::Error;

use reqwest::blocking::{Body, Client};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::error::YurlError;

const CONTENT_TYPE_KEY: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_FROM: &str = "multipart/form-data; boundary=<calculated when request is sent>";
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
            ContentType::URLENCODED => { (CONTENT_TYPE_KEY, CONTENT_TYPE_URL) }
            ContentType::JSON => { (CONTENT_TYPE_KEY, CONTENT_TYPE_JSON) }
            ContentType::FORM => { (CONTENT_TYPE_KEY, CONTENT_TYPE_FROM) }
            ContentType::FILE => { (CONTENT_TYPE_KEY, CONTENT_TYPE_FROM) }
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

impl Request {
    pub fn run(&self) -> Result<String, Box<dyn Error>> {
        match self.method {
            Method::GET => { self.get() }
            Method::POST => { self.post() }
            Method::PUT => { self.put() }
            Method::DELETE => { self.delete() }
        }
    }

    fn get(&self) -> Result<String, Box<dyn Error>> {
        let request = reqwest::blocking::Request::new(reqwest::Method::GET, Url::parse(&self.url)?);
        let request = self.build(request)?;
        Ok(Client::new().execute(request)?.text().unwrap())
    }

    fn post(&self) -> Result<String, Box<dyn Error>> {
        let request = reqwest::blocking::Request::new(reqwest::Method::POST, Url::parse(&self.url)?);
        let request = self.build(request)?;
        Ok(Client::new().execute(request)?.text().unwrap())
    }

    fn put(&self) -> Result<String, Box<dyn Error>> {
        let request = reqwest::blocking::Request::new(reqwest::Method::PUT, Url::parse(&self.url)?);
        let request = self.build(request)?;
        Ok(Client::new().execute(request)?.text().unwrap())
    }

    fn delete(&self) -> Result<String, Box<dyn Error>> {
        let request = reqwest::blocking::Request::new(reqwest::Method::DELETE, Url::parse(&self.url)?);
        let request = self.build(request)?;
        Ok(Client::new().execute(request)?.text().unwrap())
    }

    fn build(&self, mut request: reqwest::blocking::Request) -> Result<reqwest::blocking::Request, Box<dyn Error>> {
        let mut headers = reqwest::header::HeaderMap::try_from(&self.headers)?;
        headers.insert(self.content_type.to_kv().0, self.content_type.to_kv().1.parse()?);
        match self.content_type {
            ContentType::URLENCODED => {
                let mut url = request.url_mut();
                for (k, v) in self.params.iter() {
                    let mut query = String::new();
                    query.push_str(k);
                    query.push('=');
                    query.push_str(v);
                    url.set_query(Some(&query));
                }
                for (k, v) in headers.into_iter() {
                    request.headers_mut().append(k.unwrap(), v);
                }
            }
            ContentType::FORM => {
                let body: Vec<String> = self.params.iter().map(|(k, v)| {
                    let mut query = String::new();
                    query.push_str(k);
                    query.push('=');
                    query.push_str(v);
                    query
                }).collect();
                *request.body_mut() = Some(body.join("&").into());
                for (k, v) in headers.into_iter() {
                    request.headers_mut().append(k.unwrap(), v);
                }
            }
            ContentType::JSON => {
                *request.body_mut() = Some(Body::from(json!(&self.params).to_string()));
                for (k, v) in headers.into_iter() {
                    request.headers_mut().append(k.unwrap(), v);
                }
            }
            ContentType::FILE => {
                return Err(Box::new(YurlError::new("not support file")));
            }
        }
        Ok(request)
    }
}