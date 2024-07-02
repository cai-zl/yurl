use std::collections::HashMap;

use chrono::Local;

type Fun = fn() -> String;

pub struct Function {
    pub key: &'static str,
    pub about: &'static str,
    pub fun: Fun,
}

macro_rules! functions {
    ($($e:expr),*) => {
        {
            let mut functions:HashMap<String,Function> = HashMap::new();
            $(
                functions.insert($e.key.to_string(),$e);
            )*
            functions
        }
    };
}

impl Function {
    pub fn new(key: &'static str, about: &'static str, fun: Fun) -> Self {
        Self { key, about, fun }
    }

    pub fn functions() -> HashMap<String, Function> {
        // datetime
        let timestamp = Self::new("timestamp", "get current timestamp.", || {
            Local::now().timestamp().to_string()
        });
        let timestamp_millis =
            Self::new("timestamp_millis", "get current timestamp millis.", || {
                Local::now().timestamp_millis().to_string()
            });
        let datetime = Self::new("datetime", "get current datetime.", || {
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        });
        let date = Self::new("date", "get current date.", || {
            Local::now().format("%Y-%m-%d").to_string()
        });
        let time = Self::new("time", "get current time.", || {
            Local::now().format("%H:%M:%S").to_string()
        });
        let date_min = Self::new("date_min", "get current date min time.", || {
            Local::now().format("%Y-%m-%d 00:00:00").to_string()
        });
        let date_max = Self::new("date_max", "get current date max time.", || {
            Local::now().format("%Y-%m-%d 23:59:59").to_string()
        });

        functions![
            timestamp,
            timestamp_millis,
            datetime,
            date,
            time,
            date_min,
            date_max
        ]
    }
}

#[cfg(test)]
mod tests {}
