use std::collections::HashMap;

use chrono::Local;

type Fun = fn() -> String;

pub struct Function {
    pub key: String,
    pub about: String,
    pub fun: Fun,
}

impl Function {
    pub fn functions() -> HashMap<String, Function> {
        let mut functions = HashMap::new();

        // datetime
        let timestamp = Function {
            key: "timestamp".to_string(),
            about: "get current timestamp.".to_string(),
            fun: || Local::now().timestamp().to_string(),
        };
        let timestamp_millis = Function {
            key: "timestamp_millis".to_string(),
            about: "get current timestamp millis.".to_string(),
            fun: || Local::now().timestamp_millis().to_string(),
        };
        let datetime = Function {
            key: "datetime".to_string(),
            about: "get current datetime.".to_string(),
            fun: || Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let date = Function {
            key: "date".to_string(),
            about: "get current date.".to_string(),
            fun: || Local::now().format("%Y-%m-%d").to_string(),
        };
        let time = Function {
            key: "time".to_string(),
            about: "get current time.".to_string(),
            fun: || Local::now().format("%H:%M:%S").to_string(),
        };
        let date_min = Function {
            key: "date_min".to_string(),
            about: "get current date min time.".to_string(),
            fun: || Local::now().format("%Y-%m-%d 00:00:00").to_string(),
        };
        let date_max = Function {
            key: "date_max".to_string(),
            about: "get current date max time.".to_string(),
            fun: || Local::now().format("%Y-%m-%d 23:59:59").to_string(),
        };

        functions.insert(timestamp.key.clone(), timestamp);
        functions.insert(timestamp_millis.key.clone(), timestamp_millis);
        functions.insert(datetime.key.clone(), datetime);
        functions.insert(date.key.clone(), date);
        functions.insert(time.key.clone(), time);
        functions.insert(date_min.key.clone(), date_min);
        functions.insert(date_max.key.clone(), date_max);

        functions
    }
}

#[cfg(test)]
mod tests {}
