use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::core::function::Function;
use crate::core::request::Request;

pub mod error;
pub mod expression;
pub mod function;
pub mod request;

#[derive(Serialize, Deserialize)]
pub struct Template<'a> {
    pub vars: HashMap<String, String>,
    pub requests: Vec<Request>,
    #[serde(skip)]
    pub functions: HashMap<String, Function>,
    #[serde(skip)]
    pub responses: HashMap<&'a str, &'a Request>,
}

impl<'a> Template<'a> {
    pub fn from_to_yaml(yaml: &str) -> Result<Self, Box<dyn Error>> {
        let template: Template = serde_yaml::from_str(yaml)?;
        Ok(template)
    }
}
