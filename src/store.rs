use std::collections::HashMap;

pub struct Store {
    map: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<&String> {
        return self.map.get(&key);
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}
