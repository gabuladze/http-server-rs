use std::collections::HashMap;

pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
