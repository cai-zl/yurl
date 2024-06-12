use std::collections::HashMap;

type Fun = fn() -> String;

pub struct Function {
    pub key: String,
    pub about: String,
    pub fun: Fun,
}

impl Function {
    pub fn functions() -> HashMap<String, Function> {
        let mut functions = HashMap::new();
        let uuid = Function { key: "uuid".to_string(), about: "hello yurl".to_string(), fun: || { String::from("uuid") } };
        let uid = Function { key: "uid".to_string(), about: "hello yurl".to_string(), fun: || { String::from("uid") } };
        functions.insert(uuid.key.clone(), uuid);
        functions.insert(uid.key.clone(), uid);
        functions
    }
}