use std::collections::HashMap;

type Fun = fn() -> String;

pub struct Function {
    pub name: String,
    pub about: String,
    pub fun: Fun,
}

impl Function {
    pub fn functions() -> HashMap<String, Function> {
        let mut functions = HashMap::new();
        let uuid = Function { name: "uuid".to_string(), about: "hello yurl".to_string(), fun: || { String::from("uuid") } };
        let uid = Function { name: "uid".to_string(), about: "hello yurl".to_string(), fun: || { String::from("uid") } };
        functions.insert(uuid.name.clone(), uuid);
        functions.insert(uid.name.clone(), uid);
        functions
    }
}