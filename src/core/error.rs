use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct YurlError {
    pub message: String,
}

impl Display for YurlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)?;
        Ok(())
    }
}

impl Error for YurlError {}

impl YurlError {
    pub fn new(message: &str) -> Self {
        YurlError {
            message: message.to_string()
        }
    }
}
