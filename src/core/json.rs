use crate::core::Error;
use crate::core::YurlError;
use crate::yurl_error;
use serde_json::Value;

pub struct Json<'a> {
    value: Option<&'a Value>,
    path: Vec<String>,
}

impl<'a> Json<'a> {
    pub fn new(value: &'a Value, path: String) -> Self {
        let mut p: Vec<String> = path.split(".").map(|m| m.to_string()).collect();
        p.reverse();
        Self {
            value: Some(value),
            path: p,
        }
    }

    pub fn get_value(mut self) -> Result<&'a Value, Box<dyn Error>> {
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
            None => Err(yurl_error!("json parse error.")),
            Some(v) => Ok(v),
        };
    }
}
